use chrono::{DateTime, Local};
use log::{error, info};
use rbnproto::httpapi::{RaceConfig, RaceConfigUpdate, RaceCreate, RaceInfoUpdate, RaceUserState, UserHeart, UserQuery, UserScore};
use rbnproto::httpapi::{UserLogin, UserLogout, RaceInfo, RaceBrief};
use rbnproto::metaapi::{RaceJoin, RaceUpdate, RaceAccess, MetaRaceData};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::db;
use crate::lobby::RaceLobby;
use crate::player::LobbyPlayer;
use crate::series::customize::Customize;
use crate::series::daily::Daily;
use crate::series::Series;
use std::collections::HashMap;
use std::process::exit;
use std::sync::Arc;
use tera::Tera;

#[derive(Default)]
pub struct RacingServer {
    pub tera: Tera,
    tick_time: DateTime<Local>,
    pub lobby: RaceLobby,
    pub races: HashMap<String, Box<dyn Series + Send + Sync>>,
}

impl RacingServer {
    pub fn init(mut self) -> Self {
        self.tera = Tera::new("templates/**/*.html").expect("Failed to compile templates");
        self.check_environment();
        self.races.insert("Daily Challenge".to_string(), Box::new(Daily::default().init()));
        self
    }

    pub fn dynamic_reload_templates(&mut self) {
        if Local::now().signed_duration_since(self.tick_time) > chrono::Duration::seconds(1) {
            self.tick_time = Local::now();
            self.tera.full_reload().expect("Failed to watch tera template directory.");
        }
    }

    pub fn check_environment(&mut self) {
        let path = std::env::current_exe().unwrap().parent().unwrap().join("rsfdata");
        if !path.exists() || !path.is_dir() {
            error!("Fatal Error, Make sure rsfdata directory is exists in your app's running path.");
            exit(1);
        }
    }

    pub fn is_race_exist(&mut self, name: &String) -> bool {
        self.races.contains_key(name)
    }

    pub fn recycle_invalid_races(&mut self) {
        self.races.retain(|_k, v| !v.need_recycle());
    }

    pub fn recycle_invalid_players(&mut self) {
        self.lobby.check_players();
        for (_, race) in self.races.iter_mut() {
            race.check_players(&self.lobby);
        }
    }

    pub fn force_leave_race(&mut self, token: &Uuid) {
        for (_, race) in self.races.iter_mut() {
            race.leave(&token.to_string());
        }
    }

    pub async fn user_login(&mut self, user: UserLogin) -> Option<String> {
        if user.passwd != "simrallycn" {
            return None;
        }

        if let Some(token) = &self.lobby.get_token_by_name(user.name.clone()) {
            self.force_leave_race(token);
            return Some(token.to_string());
        }

        let token = Uuid::new_v4();
        let tokenstr = token.to_string();
        let player: LobbyPlayer = LobbyPlayer::new(&tokenstr, &user.name);
        db::RaceDB::default().on_user_login(&player).await;
        info!("User {} login with token {}", player.profile_name, tokenstr);
        self.lobby.push_player(token, player);
        return Some(tokenstr);
    }

    pub fn user_heartbeat(&mut self, user: UserHeart) {
        if let Ok(token) = Uuid::parse_str(&user.token) {
            if let Some(player) = self.lobby.get_player(token) {
                player.set_alive();
            }
        }
    }

    pub fn user_logout(&mut self, user: UserLogout) -> bool {
        if let Ok(token) = Uuid::parse_str(&user.token) {
            if self.lobby.is_player_exist(Some(&token), None) {
                self.lobby.pop_player(&token);
                return true;
            }
        }
        return false;
    }

    pub async fn get_user_score(&mut self, tokenstr: &String) -> Option<UserScore> {
        if let Ok(token) = Uuid::parse_str(tokenstr) {
            if let Some(player) = self.lobby.get_player(token) {
                return db::RaceDB::default().query_user_score(player).await;
            }
        }

        None
    }

    pub async fn get_all_user_score(&mut self) -> Vec<UserScore> {
        db::RaceDB::default().query_all_user_score().await
    }

    pub fn get_race_news(&mut self) -> String {
        let mut count = 0u32;
        self.races.iter_mut().for_each(|(_, race)| {
            count += race.get_players_counts();
        });

        format!("{} players online, enter [Time Trial] or [Practice] to Join Race !!!", count)
    }

    pub fn get_race_list(&mut self) -> Option<Vec<RaceBrief>> {
        if self.races.is_empty() {
            return None;
        }
    
        let mut racelist = vec![];
        for (_name, race) in self.races.iter_mut() {
            let brief = race.get_race_brief();
            racelist.push(brief);
        }

        Some(racelist)
    }

    pub fn get_race_info(&mut self, name: &String) -> Option<RaceInfo> {
        if let Some(race) = self.races.get_mut(name) {
            return Some(race.get_race_config());
        }

        None
    }

    pub fn update_race_info(&mut self, update: RaceInfoUpdate) -> bool {
        if let Ok(token) = Uuid::parse_str(&update.token) {
            if self.lobby.is_player_exist(Some(&token), None) {
                if let Some(race) = self.races.get_mut(&update.info.name) {
                    race.update_race_config(update.info);
                    return true;
                }
            }
        }
        return false;
    }

    pub fn get_player_race_config(&mut self, query: &UserQuery) -> Option<RaceConfig> {
        for (_, race) in self.races.iter_mut() {
            if let Some(config) = race.get_player_config(&query.token) {
                return Some(config);
            }
        }
        None
    }

    pub fn update_player_race_config(&mut self, update: RaceConfigUpdate) -> bool {
        for (_, race) in self.races.iter_mut() {
            if race.update_player_config(&update.token, update.cfg.clone()) {
                return true;
            }
        }
        return false;
    }

    pub fn get_race_started(&mut self, name: &String) -> Option<bool> {
        if let Some(race) = self.races.get_mut(name) {
            return Some(race.is_started());
        }

        None
    }

    pub fn set_race_started(&mut self, access: &RaceAccess) -> bool {
        if let Ok(token) = Uuid::parse_str(&access.token) {
            if self.lobby.is_player_exist(Some(&token), None) {
                if let Some(race) = self.races.get_mut(&access.room) {
                    return race.set_start();
                }
            }
        }
        return false;
    }

    pub fn get_race_userstate(&mut self, name: &String) -> Option<Vec<RaceUserState>> {
        if let Some(race) = self.races.get_mut(name) {
            return Some(race.get_players_state());
        }

        None
    }

    pub fn create_race(&mut self, create: RaceCreate) -> bool {
        if self.is_race_exist(&create.info.name) {
            return true;
        }

        if let Ok(token) = Uuid::parse_str(&create.token.as_str()) {
            self.force_leave_race(&token);
            if let Some(player) = self.lobby.get_player(token) {
                let mut raceroom = Customize::default();
                raceroom.set_limit(8);
                raceroom.update_race_config(create.info.clone());
                if create.locked {
                    if let Some(passwd) = &create.passwd {
                        raceroom.lock_with_passwd(passwd);
                    }
                }
                raceroom.join(&player);
                self.races.insert(create.info.name, Box::new(raceroom));
                return true;
            }
        }
        return false;
    }

    pub fn join_race(&mut self, join: RaceJoin) -> bool {
        if let Ok(token) = Uuid::parse_str(&join.token.as_str()) {
            if let Some(player) = self.lobby.get_player(token) {
                if let Some(race) = self.races.get_mut(&join.room) {
                    if race.is_joinable(&join) {
                        info!(" Player {} join into race {}", player.profile_name, join.room);
                        race.join(player);
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn leave_race(&mut self, roomname: String, tokenstr: String) -> bool {
        if let Some(race) = self.races.get_mut(&roomname) {
            race.leave(&tokenstr);
            return true;
        }
        return false;
    }

    pub fn destroy_race(&mut self, roomname: String, tokenstr: String) -> bool {
        if let Ok(token) = Uuid::parse_str(&tokenstr) {
            if self.lobby.is_player_exist(Some(&token), None) {
                self.races.remove(&roomname);
                return true;
            }
        }
        return false;
    }

    pub fn race_player_access(&mut self, access: &RaceAccess, writer: Arc<Mutex<OwnedWriteHalf>>) -> bool {
        if let Some(race) = self.races.get_mut(&access.room) {
            return race.access(&access.token, writer);
        }
        return false;
    }

    pub fn update_player_state(&mut self, update: &RaceUpdate) -> bool {
        if let Some(race) = self.races.get_mut(&update.room) {
            return race.update_player_state(&update.token, update.state.clone());
        }
        return false;
    }

    pub fn update_player_race_data(&mut self, data: MetaRaceData) -> bool {
        if let Some(race) = self.races.get_mut(&data.room) {
            return race.update_player_data(&data.token, data.clone());
        }
        return false;
    }

    pub async fn load_image(&mut self, image: &String) -> Option<Vec<u8>> {
        let path = std::env::current_exe().unwrap().parent().unwrap().join("templates").join("assets");
        let image_file = path.join(image);
        tokio::fs::read(image_file).await.ok()
    }

    pub async fn load_file(&mut self, file: &String) -> Option<Vec<u8>> {
        let path = std::env::current_exe().unwrap().parent().unwrap().join("downloads");
        let filepath = path.join(file);
        tokio::fs::read(filepath).await.ok()
    }
}
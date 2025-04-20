use log::info;
use rbnproto::httpapi::{RaceConfig, RaceInfo, RaceState, RoomState};
use rbnproto::metaapi::{MetaRaceData, MetaRaceProgress, MetaRaceResult, MetaRaceRidicule, MetaRaceState, RaceCmd};
use serde::{Serialize, Deserialize};
use crate::db;
use crate::player::RacePlayer;
use chrono::{DateTime, Local};

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum RoomRaceState {
    #[default]
    RoomRaceInit,
    RoomRaceBegin,
    RoomRacePrepare,
    RoomRaceReady,
    RoomRaceLoading,
    RoomRaceLoaded,
    RoomRaceStarting,
    RoomRaceStarted,
    RoomRaceRunning,
    RoomRaceFinished,
    RoomRaceExiting,
    RoomRaceEnd,
}

#[derive(Default)]
pub struct RaceRoom {
    pub info: RaceInfo,
    pub players: Vec<RacePlayer>,
    pub room_state: RoomState,
    pub race_state: RoomRaceState,
    limit: Option<usize>,
    passwd: Option<String>,
    rank_tick: DateTime<Local>,
}

impl RaceRoom {
    pub fn set_limit(&mut self, limit: usize) {
        self.limit = Some(limit);
    }

    pub fn set_pass(&mut self, pass: String) {
        self.passwd = Some(pass);
    }

    pub fn push_player(&mut self, player: RacePlayer) {
        self.players.push(player);
    }

    pub fn pop_player(&mut self, tokenstr: &String) {
        self.players.retain(|x| &x.tokenstr != tokenstr);
    }

    pub fn get_player(&mut self, tokenstr: &String) -> Option<&mut RacePlayer> {
        for (_, player) in self.players.iter_mut().enumerate() {
            if &player.tokenstr == tokenstr {
                return Some(player);
            }
        }
        None
    }

    pub fn is_player_exist(&mut self, name: &String) -> bool {
        for player in &self.players {
            if &player.profile_name == name {
                return true;
            }
        }
        return false;
    }

    pub fn is_empty(&mut self) -> bool {
        self.players.is_empty()
    }

    pub fn is_full(&mut self) -> bool {
        if let Some(limit) = self.limit {
            return self.players.len() >= limit;
        }
        false
    }

    pub fn is_locked(&mut self) -> bool {
        if let Some(_) = self.passwd {
            return true;
        }
        false
    }

    pub fn pass_match(&mut self, passwd: &String) -> bool {
        if let Some(pass) = &self.passwd {
            return passwd == pass;
        }
        return false;
    }

    pub fn is_racing_started(&self) -> bool {
        match self.race_state {
            RoomRaceState::RoomRaceInit => false,
            _ => true,
        }
    }

    pub fn set_racing_started(&mut self) -> bool {
        if !self.is_racing_started() && !self.is_empty() {
            self.race_state = RoomRaceState::RoomRaceBegin;
            return true;
        }
        false
    }

    pub fn sort_players_by_progress(&mut self) {
        self.players.sort_by(|a, b| a.sort_by_progress(b));
    }

    pub fn sort_players_by_time(&mut self) {
        self.players.sort_by(|a, b| a.sort_by_time(b));
    }

    pub fn is_all_players_ready(&mut self) -> bool {
        self.players.iter().all(|x| {
            match x.state {
                RaceState::RaceReady => true,
                _ => false,
            }
        })
    }

    pub fn is_all_players_loaded(&mut self) -> bool {
        self.players.iter().all(|x| {
            match x.state {
                RaceState::RaceLoaded => true,
                _ => false,
            }
        })
    }

    pub fn is_all_players_started(&mut self) -> bool {
        self.players.iter().all(|x| {
            match x.state {
                RaceState::RaceStarted => true,
                _ => false,
            }
        })
    }

    pub fn is_all_players_finish(&mut self) -> bool {
        self.players.iter().all(|x| {
            match x.state {
                RaceState::RaceRetired | RaceState::RaceFinished | RaceState::RaceExitMenu => true,
                _ => false,
            }
        })
    }

    pub fn is_all_players_exitmenu(&mut self) -> bool {
        self.players.iter().all(|x| {
            match x.state {
                RaceState::RaceExitMenu => true,
                _ => false,
            }
        })
    }

    pub fn reset_all_players_state(&mut self) {
        self.players.iter_mut().for_each(|x| {
            x.state = RaceState::RaceDefault;
            x.race_cfg = RaceConfig::default();
            x.last_race_data = MetaRaceData::default();
            x.race_data = MetaRaceData::default();
            x.lastridicule = Local::now();
        });
    }

    pub fn notify_all_players_prepare(&mut self) {
        let cmd = RaceCmd::RaceCmdPrepare(self.info.clone());
        let players = self.players.clone();
        tokio::spawn(async move {
            for player in players {
                player.notify_user_cmd(&cmd).await;
            }
        });
    }

    pub fn notify_all_players_load(&mut self) {
        let cmd = RaceCmd::RaceCmdLoad;
        let players = self.players.clone();
        tokio::spawn(async move {
            for player in players {
                player.notify_user_cmd(&cmd).await;
            }
        });
    }

    pub fn notify_all_players_start(&mut self) {
        let cmd = RaceCmd::RaceCmdStart;
        let players = self.players.clone();
        tokio::spawn(async move {
            for player in players {
                player.notify_user_cmd(&cmd).await;
            }
        });
    }

    pub fn notify_all_players_upload(&mut self) {
        let cmd = RaceCmd::RaceCmdUpload;
        let players = self.players.clone();
        tokio::spawn(async move {
            for player in players {
                player.notify_user_cmd(&cmd).await;
            }
        });
    }

    pub fn get_race_progress(&mut self) -> Vec::<MetaRaceProgress> {
        let mut results = Vec::<MetaRaceProgress>::new();
        let leader = self.players.first().unwrap().clone();
        for player in &self.players {
            let mut result = MetaRaceProgress::default();
            result.profile_name = player.profile_name.clone();
            result.progress = player.race_data.progress;
            let difflength = (leader.race_data.progress - player.race_data.progress) / player.race_data.stagelen * self.info.stage_len as f32;
            if player.race_data.speed != 0f32 {
                result.difffirst = difflength / player.race_data.speed * 3.6;
            }
            else {
                result.difffirst = difflength / 10.0 * 3.6; // default 10km/h as 3.6m/s.
            }
            result.carlook = player.race_data.carlook.clone();
            result.carpos = player.race_data.carpos.clone();
            results.push(result);
        }
        results
    }

    pub fn get_race_result(&mut self) -> Vec::<MetaRaceResult> {
        let mut results = Vec::<MetaRaceResult>::new();
        let leader = self.players.first().unwrap().clone();
        for (i, player) in self.players.iter().enumerate() {
            let mut result = MetaRaceResult::default();
            result.profile_name = player.profile_name.clone();
            result.racecar = player.race_cfg.car.clone();
            result.splittime1 = player.race_data.splittime1;
            result.splittime2 = player.race_data.splittime2;
            result.finishtime = player.race_data.finishtime;
            result.difftime = player.race_data.finishtime - leader.race_data.finishtime;
            if result.finishtime == 3600.0f32 { // if not complete race, default reduce 2 score.
                result.score = -5i32;
            } else {
                result.score = (self.players.len() - i) as i32 * 3;
            }
            results.push(result);
        }
        results
    }

    pub fn notify_all_players_race_state(&mut self) {
        if self.is_empty() {
            return;
        }
        let mut states = vec![];
        for player in &self.players {
            states.push(MetaRaceState {name: player.profile_name.clone(), state: player.state.clone()});
        }
        let players = self.players.clone();
        tokio::spawn(async move {
            for player in players {
                player.notify_racestate(&states).await;
            }
        });
    }

    pub fn notify_all_players_race_data(&mut self) {
        if self.is_empty() {
            return;
        }

        self.sort_players_by_progress();
        let results = self.get_race_progress();
        let players = self.players.clone();
        tokio::spawn(async move {
            for player in players {
                player.notify_racedata(&results).await;
            }
        });
    }

    pub fn notify_all_players_race_ridicule(&mut self) {
        if self.is_empty() {
            return;
        }

        if Local::now().signed_duration_since(self.rank_tick) > chrono::Duration::seconds(1) {
            self.rank_tick = Local::now();

            self.sort_players_by_progress();
            let players = self.players.clone();

            for (i, player) in self.players.iter_mut().enumerate() {
                let player_pos = player.race_data.progress / player.race_data.stagelen * self.info.stage_len as f32;
                let player_last_pos = player.last_race_data.progress / player.last_race_data.stagelen * self.info.stage_len as f32;

                if player_pos < player_last_pos || player.race_data.racetime < 10.0 {
                    continue; // player is in backward state or progress too short.
                }

                let winer: Vec<String> = players[0..i]
                    .iter()
                    .filter(|x| {
                        let pos = x.race_data.progress / player.race_data.stagelen * self.info.stage_len as f32;
                        let last_pos = x.last_race_data.progress / player.last_race_data.stagelen * self.info.stage_len as f32;
                        pos > player_pos && last_pos < player_last_pos
                    })
                    .map(|x| x.profile_name.clone())
                    .collect();

                if winer.len() > 0 {
                    let mut ridicules = MetaRaceRidicule::default();
                    ridicules.players = winer;
            
                    if Local::now().signed_duration_since(player.lastridicule) > chrono::Duration::seconds(10) {
                        let mut playerc = player.clone();
                        tokio::spawn(async move {
                            info!("notify ridicule to player: {} with: {:?}", playerc.profile_name, ridicules);
                            playerc.notify_ridicule(&ridicules).await;
                        });
                        player.lastridicule = Local::now();
                    }
                }

                player.last_race_data = player.race_data.clone();
            }
        }
    }

    pub fn notify_all_players_race_result(&mut self) {
        if self.is_empty() {
            return;
        }

        self.sort_players_by_time();
        let results = self.get_race_result();
        let players = self.players.clone();
        tokio::spawn(async move {
            for player in players {
                player.notify_result(&results).await;
            }
        });
    }

    pub fn store_all_players_race_result(&mut self) {
        if self.is_empty() {
            return;
        }

        let results = self.get_race_result();
        tokio::spawn(async move {
            db::RaceDB::default().on_race_finished(&results).await;
        });
    }

    pub fn guess_race_remain(&mut self) -> u32 {
        if let Some(player) = self.players.get(0) {
            let leftlen = (player.race_data.stagelen - player.race_data.progress) / player.race_data.stagelen * self.info.stage_len as f32;
            return (leftlen / 80.0 * 3.6) as u32; // default average speed 80km/h as 3.6m/s.
        }

        return 0u32;
    }

    pub fn update_room_state(&mut self) {
        if self.is_racing_started() {
            self.room_state = RoomState::RoomRaceOn;
        } else {
            if self.is_full() {
                self.room_state = RoomState::RoomFull;
            } else {
                self.room_state = RoomState::RoomFree;
            }
        }
    }

    pub fn update_race_state(&mut self) {
        match self.race_state {
            RoomRaceState::RoomRaceBegin => {
                info!("notify prepare game: {}", self.info.name);
                self.reset_all_players_state();
                self.notify_all_players_prepare();
                self.race_state = RoomRaceState::RoomRacePrepare;
            }
            RoomRaceState::RoomRacePrepare => {
                if self.is_all_players_ready() {
                    self.race_state = RoomRaceState::RoomRaceReady;
                }
            }
            RoomRaceState::RoomRaceReady => {
                info!("notify load game: {}", self.info.name);
                self.notify_all_players_load();
                self.race_state = RoomRaceState::RoomRaceLoading;
            }
            RoomRaceState::RoomRaceLoading => {
                if self.is_all_players_loaded() {
                    self.race_state = RoomRaceState::RoomRaceLoaded;
                }
            }
            RoomRaceState::RoomRaceLoaded => {
                info!("notify start game: {}", self.info.name);
                self.notify_all_players_start();
                self.race_state = RoomRaceState::RoomRaceStarting;
            }
            RoomRaceState::RoomRaceStarting => {
                if self.is_all_players_started() {
                    self.race_state = RoomRaceState::RoomRaceStarted;
                }
            }
            RoomRaceState::RoomRaceStarted => {
                info!("notify exchange data: {}", self.info.name);
                self.notify_all_players_upload();
                self.race_state = RoomRaceState::RoomRaceRunning;
            }
            RoomRaceState::RoomRaceRunning => {
                self.notify_all_players_race_data();
                self.notify_all_players_race_ridicule();
                if self.is_all_players_finish() {
                    self.race_state = RoomRaceState::RoomRaceFinished;
                }
            }
            RoomRaceState::RoomRaceFinished => {
                info!("notify finished results: {}", self.info.name);
                self.notify_all_players_race_result();
                self.store_all_players_race_result();
                self.race_state = RoomRaceState::RoomRaceExiting;
            }
            RoomRaceState::RoomRaceExiting => {
                if self.is_all_players_exitmenu() {
                    self.race_state = RoomRaceState::RoomRaceEnd;
                }
            }
            RoomRaceState::RoomRaceEnd => {
                self.race_state = RoomRaceState::RoomRaceInit;
            }
            _ => {}
        }
    }
}
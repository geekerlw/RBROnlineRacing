use protocol::httpapi::{RoomState, RaceState, RaceCmd, MetaRaceResult, RaceInfo};
use serde::{Serialize, Deserialize};
use crate::player::RacePlayer;
use log::info;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
enum RoomRaceState {
    #[default]
    RoomRaceInit,
    RoomRaceBegin,
    RoomRaceReady,
    RoomRaceLoading,
    RoomRaceLoaded,
    RoomRaceStarting,
    RoomRaceStarted,
    RoomRaceRunning,
    RoomRaceFinished,
    RoomRaceEnd,
}

#[derive(Default, Serialize, Deserialize)]
pub struct RaceRoom {
    pub info: RaceInfo,
    pub room_state: RoomState,
    pub passwd: Option<String>,
    race_state: RoomRaceState,
    pub players: Vec<RacePlayer>,
}

impl RaceRoom {
    pub fn push_player(&mut self, player: RacePlayer) {
        self.players.push(player);
    }

    pub fn pop_player(&mut self, player: &String) {
        self.players.retain(|x| &x.profile_name != player);
    }

    pub fn pop_player_by_token(&mut self, tokenstr: &String) {
        self.players.retain(|x| &x.tokenstr != tokenstr);
    }

    pub fn is_player_exist(&mut self, name: &String) -> bool {
        for player in &self.players {
            if &player.profile_name == name {
                return true;
            }
        }
        return false;
    }

    pub fn pass_match(&mut self, passwd: &String) -> bool {
        if let Some(pass) = &self.passwd {
            return passwd == pass;
        }
        return false;
    }

    pub fn is_empty(&mut self) -> bool {
        self.players.is_empty()
    }

    pub fn is_full(&mut self) -> bool {
        self.players.len() >= 8
    }

    pub fn is_locked(&mut self) -> bool {
        match self.room_state {
            RoomState::RoomLocked => true,
            _ => false,
        }
    }

    pub fn is_racing_started(&self) -> bool {
        match self.race_state {
            RoomRaceState::RoomRaceInit => false,
            _ => true,
        }
    }

    pub fn set_racing_started(&mut self) {
        if !self.is_racing_started() {
            self.race_state = RoomRaceState::RoomRaceBegin;
        }
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
                RaceState::RaceRetired | RaceState::RaceFinished => true,
                _ => false,
            }
        })
    }

    pub async fn notify_all_players_load(&mut self) {
        let cmd = RaceCmd::RaceCmdLoad;
        for player in &self.players {
            player.notify_user_cmd(&cmd).await;
        }
    }

    pub async fn notify_all_players_start(&mut self) {
        let cmd = RaceCmd::RaceCmdStart;
        for player in &self.players {
            player.notify_user_cmd(&cmd).await;
        }
    }

    pub async fn notify_all_players_upload(&mut self) {
        let cmd = RaceCmd::RaceCmdUpload;
        for player in &self.players {
            player.notify_user_cmd(&cmd).await;
        }
    }

    pub fn get_race_result(&mut self) -> Vec::<MetaRaceResult> {
        let mut results = Vec::<MetaRaceResult>::new();
        let leader = self.players.first().unwrap().clone();
        for player in &self.players {
            let mut result = MetaRaceResult::default();
            result.profile_name = player.profile_name.clone();
            result.racetime = player.race_data.racetime;
            result.progress = player.race_data.progress;
            result.splittime1 = player.race_data.splittime1;
            result.splittime2 = player.race_data.splittime2;
            result.finishtime = player.race_data.finishtime;
            let difflength = (leader.race_data.progress - player.race_data.progress) / player.race_data.stagelen * self.info.stage_len as f32;
            if player.race_data.speed != 0f32 {
                result.difffirst = difflength / player.race_data.speed * 3.6;
            }
            else {
                result.difffirst = difflength * 3.6; // default 1km/h as 3.6m/s.
            }
            result.difftime = player.race_data.finishtime - leader.race_data.finishtime;
            results.push(result);
        }
        results
    }

    pub async fn notify_all_players_race_data(&mut self) {
        self.sort_players_by_progress();
        let results = self.get_race_result();
        for player in &self.players {
            player.notify_racedata(&results).await;
        }
    }

    pub async fn notify_all_players_race_result(&mut self) {
        self.sort_players_by_time();
        let results = self.get_race_result();
        for player in &self.players {
            player.notify_result(&results).await;
        }
    }

    fn update_room_state(&mut self) {
        if !self.is_player_exist(&self.info.owner.clone()) {
            if let Some(owner) = self.players.get(0) {
                self.info.owner = owner.profile_name.clone();
            }
        }

        if self.is_locked() {
            return;
        }

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

    fn update_race_state(&mut self) {
        match self.race_state {
            RoomRaceState::RoomRaceBegin => {
                if self.is_all_players_ready() {
                    self.race_state = RoomRaceState::RoomRaceReady;
                }
            }
            RoomRaceState::RoomRaceLoading => {
                if self.is_all_players_loaded() {
                    self.race_state = RoomRaceState::RoomRaceLoaded;
                }
            }
            RoomRaceState::RoomRaceStarting => {
                if self.is_all_players_started() {
                    self.race_state = RoomRaceState::RoomRaceStarted;
                }
            }
            RoomRaceState::RoomRaceRunning => {
                if self.is_all_players_finish() {
                    self.race_state = RoomRaceState::RoomRaceFinished;
                }
            }
            RoomRaceState::RoomRaceEnd => {
                self.race_state = RoomRaceState::RoomRaceInit;
            }
            _ => {}
        }
    }

    pub async fn check_room_state(&mut self) {
        self.update_room_state();
        self.update_race_state();
        match self.race_state {
            RoomRaceState::RoomRaceReady => {
                info!("notify load game: {}", self.info.name);
                self.notify_all_players_load().await;
                self.race_state = RoomRaceState::RoomRaceLoading;
            },
            RoomRaceState::RoomRaceLoaded => {
                info!("notify start game: {}", self.info.name);
                self.notify_all_players_start().await;
                self.race_state = RoomRaceState::RoomRaceStarting;
            },
            RoomRaceState::RoomRaceStarted => {
                info!("notify exchange data: {}", self.info.name);
                self.notify_all_players_upload().await;
                self.race_state = RoomRaceState::RoomRaceRunning;
            },
            RoomRaceState::RoomRaceRunning => {
                self.notify_all_players_race_data().await;
            },
            RoomRaceState::RoomRaceFinished => {
                self.notify_all_players_race_result().await;
                self.race_state = RoomRaceState::RoomRaceEnd;
            },
            _ => {},
        }
    }
}
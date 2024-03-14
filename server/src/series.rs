use protocol::httpapi::{RaceState, RaceInfo};
use protocol::metaapi::{RaceCmd, MetaRaceResult, MetaRaceProgress};
use serde::{Serialize, Deserialize};
use crate::player::RacePlayer;
use log::info;
use chrono::Utc;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
enum SeriesRaceState {
    #[default]
    SeriesRaceInit,
    SeriesRaceBegin,
    SeriesRaceReady,
    SeriesRaceLoading,
    SeriesRaceLoaded,
    SeriesRaceStarting,
    SeriesRaceStarted,
    SeriesRaceRunning,
    SeriesRaceFinished,
    SeriesRaceEnd,
}

#[derive(Default, Serialize, Deserialize)]
pub struct RaceSeries {
    pub info: RaceInfo,
    race_state: SeriesRaceState,
    pub players: Vec<RacePlayer>,
}

impl RaceSeries {
    pub fn push_player(&mut self, player: RacePlayer) {
        self.players.push(player);
    }

    pub fn pop_player(&mut self, player: &String) {
        self.players.retain(|x| &x.profile_name != player);
    }

    pub fn pop_player_by_token(&mut self, tokenstr: &String) {
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
        self.players.len() >= 8
    }

    pub fn is_racing_started(&self) -> bool {
        match self.race_state {
            SeriesRaceState::SeriesRaceInit => false,
            _ => true,
        }
    }

    pub fn set_racing_started(&mut self) {
        if !self.is_racing_started() {
            self.race_state = SeriesRaceState::SeriesRaceInit;
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
        let starttime = Utc::now().timestamp_millis() + 3 * 1000;
        let cmd = RaceCmd::RaceCmdStart(starttime);
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
            results.push(result);
        }
        results
    }

    pub fn get_race_result(&mut self) -> Vec::<MetaRaceResult> {
        let mut results = Vec::<MetaRaceResult>::new();
        let leader = self.players.first().unwrap().clone();
        for player in &self.players {
            let mut result = MetaRaceResult::default();
            result.profile_name = player.profile_name.clone();
            result.racecar = player.race_cfg.car.clone();
            result.splittime1 = player.race_data.splittime1;
            result.splittime2 = player.race_data.splittime2;
            result.finishtime = player.race_data.finishtime;
            result.difftime = player.race_data.finishtime - leader.race_data.finishtime;
            results.push(result);
        }
        results
    }

    pub async fn notify_all_players_race_data(&mut self) {
        self.sort_players_by_progress();
        let results = self.get_race_progress();
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

    fn update_race_state(&mut self) {
        match self.race_state {
            SeriesRaceState::SeriesRaceInit => {
                if self.is_all_players_ready() {
                    self.race_state = SeriesRaceState::SeriesRaceReady;
                }
            }
            SeriesRaceState::SeriesRaceLoading => {
                if self.is_all_players_loaded() {
                    self.race_state = SeriesRaceState::SeriesRaceLoaded;
                }
            }
            SeriesRaceState::SeriesRaceStarting => {
                if self.is_all_players_started() {
                    self.race_state = SeriesRaceState::SeriesRaceStarted;
                }
            }
            SeriesRaceState::SeriesRaceRunning => {
                if self.is_all_players_finish() {
                    self.race_state = SeriesRaceState::SeriesRaceFinished;
                }
            }
            SeriesRaceState::SeriesRaceEnd => {
                self.race_state = SeriesRaceState::SeriesRaceInit;
            }
            _ => {}
        }
    }

    pub async fn check_room_state(&mut self) {
        self.update_race_state();
        match self.race_state {
            SeriesRaceState::SeriesRaceReady => {
                info!("notify load game: {}", self.info.name);
                self.notify_all_players_load().await;
                self.race_state = SeriesRaceState::SeriesRaceLoading;
            },
            SeriesRaceState::SeriesRaceLoaded => {
                info!("notify start game: {}", self.info.name);
                self.notify_all_players_start().await;
                self.race_state = SeriesRaceState::SeriesRaceStarting;
            },
            SeriesRaceState::SeriesRaceStarted => {
                info!("notify exchange data: {}", self.info.name);
                self.notify_all_players_upload().await;
                self.race_state = SeriesRaceState::SeriesRaceRunning;
            },
            SeriesRaceState::SeriesRaceRunning => {
                self.notify_all_players_race_data().await;
            },
            SeriesRaceState::SeriesRaceFinished => {
                self.notify_all_players_race_result().await;
                self.race_state = SeriesRaceState::SeriesRaceEnd;
            },
            _ => {},
        }
    }
}
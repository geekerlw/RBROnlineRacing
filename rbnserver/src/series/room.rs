use rbnproto::httpapi::{RaceInfo, RaceState, RoomState};
use rbnproto::metaapi::{MetaRaceProgress, MetaRaceResult, MetaRaceState, RaceCmd};
use serde::{Serialize, Deserialize};
use crate::db;
use crate::player::RacePlayer;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
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
                result.score = -2i32;
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
}
use protocol::httpapi::{RoomState, RaceState, MetaRaceCmd, MetaRaceResult, RaceInfo};
use crate::player::RacePlayer;

#[derive(Default)]
pub struct RaceRoom {
    pub info: RaceInfo,
    pub state: RoomState,
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

    pub fn is_empty(&mut self) -> bool {
        self.players.is_empty()
    }

    pub fn is_full(&mut self) -> bool {
        self.players.len() > 8
    }

    pub fn sort_players(&mut self) {
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

    pub fn is_all_players_finish(&mut self) -> bool {
        self.players.iter().all(|x| {
            match x.state {
                RaceState::RaceRetired | RaceState::RaceFinished => true,
                _ => false,
            }
        })
    }

    pub async fn notify_all_players_load(&mut self) {
        let cmd = MetaRaceCmd {state: RaceState::RaceLoad};
        for player in &self.players {
            player.notify_user_cmd(&cmd).await;
        }
    }

    pub async fn notify_all_players_start(&mut self) {
        let cmd = MetaRaceCmd {state: RaceState::RaceStart};
        for player in &self.players {
            player.notify_user_cmd(&cmd).await;
        }
    }

    pub fn get_race_result(&mut self) -> Vec::<MetaRaceResult> {
        let mut results = Vec::<MetaRaceResult>::new();
        self.sort_players();
        let leader = self.players.first().unwrap().clone();
        for player in &self.players {
            let mut result = MetaRaceResult::default();
            result.profile_name = player.profile_name.clone();
            result.racetime = player.race_data.racetime;
            result.process = player.race_data.process;
            result.splittime1 = player.race_data.splittime1;
            result.splittime2 = player.race_data.splittime2;
            result.finishtime = player.race_data.finishtime;
            result.difffirst = player.race_data.racetime - leader.race_data.racetime;
            results.push(result);
        }
        results
    }

    pub async fn notify_all_players_race_data(&mut self) {
        let results = self.get_race_result();
        for player in &self.players {
            player.notify_racedata(&results).await;
        }
    }

    pub async fn notify_all_players_race_result(&mut self) {
        let results = self.get_race_result();
        for player in &self.players {
            player.notify_result(&results).await;
        }
    }

    fn update_room_state(&mut self) {
        match self.state {
            RoomState::RoomDefault => {
                if !self.is_empty() {
                    self.state = RoomState::RoomRaceBegin;
                }
            }
            RoomState::RoomFree => {
                if self.is_full() {
                    self.state = RoomState::RoomFull;
                }
            }
            RoomState::RoomFull => {
                self.state = RoomState::RoomRaceBegin;
            }
            RoomState::RoomRaceBegin => {
                if self.is_all_players_ready() {
                    self.state = RoomState::RoomRaceReady;
                }
            }
            RoomState::RoomRaceLoading => {
                if self.is_all_players_loaded() {
                    self.state = RoomState::RoomRaceLoaded;
                }
            }
            RoomState::RoomRaceRunning => {
                if self.is_all_players_finish() {
                    self.state = RoomState::RoomRaceFinished;
                }
            }
            RoomState::RoomRaceEnd => {
                if self.is_full() {
                    self.state = RoomState::RoomFull;
                } else {
                    self.state = RoomState::RoomFree;
                }
            }
            _ => {}
        }
    }

    pub async fn check_room_state(&mut self) {
        self.update_room_state();
        match self.state {
            RoomState::RoomRaceReady => {
                println!("notify load game: {}", self.info.name);
                self.notify_all_players_load().await;
                self.state = RoomState::RoomRaceLoading;
            },
            RoomState::RoomRaceLoaded => {
                println!("notify start game: {}", self.info.name);
                self.notify_all_players_start().await;
                self.state = RoomState::RoomRaceRunning;
            },
            RoomState::RoomRaceRunning => {
                self.notify_all_players_race_data().await;
            },
            RoomState::RoomRaceFinished => {
                self.notify_all_players_race_result().await;
                self.state = RoomState::RoomRaceEnd;
            },
            _ => {},
        }
    }
}
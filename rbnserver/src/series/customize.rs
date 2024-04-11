use rbnproto::httpapi::{RaceBrief, RaceConfig, RaceInfo, RaceState, RaceUserState};
use rbnproto::metaapi::{MetaRaceData, RaceJoin};
use crate::lobby::RaceLobby;
use crate::player::{LobbyPlayer, RacePlayer};
use super::room::RaceRoom;
use super::Series;

pub struct Customize {
    room: RaceRoom,
}

impl Default for Customize {
    fn default() -> Self {
        let mut room = RaceRoom::default();
        room.set_limit(8);
        Self { room }
    }
}

impl Series for Customize {
    fn join(&mut self, player: &LobbyPlayer){
        self.room.push_player(RacePlayer::new(&player.tokenstr, &player.profile_name));
    }

    fn leave(&mut self, token: &String) {
        self.room.pop_player(token);
    }

    fn access(&mut self, token: &String, writer: std::sync::Arc<tokio::sync::Mutex<tokio::net::tcp::OwnedWriteHalf>>) -> bool {
        if let Some(player) = self.room.get_player(token) {
            player.writer = Some(writer);
            return true;
        }

        false
    }

    fn need_recycle(&mut self) -> bool {
        self.room.is_empty()
    }

    fn check_players(&mut self, lobby: &RaceLobby) {
        self.room.players.retain(|x| lobby.is_player_exist(Some(&x.token), None));
    }

    fn is_joinable(&mut self, join: &RaceJoin) -> bool {
        if self.room.is_full() || self.room.is_racing_started()
        || self.room.is_player_exist(&join.token) {
            return false;
        }

        if self.room.is_locked() {
            if let Some(passwd) = &join.passwd {
                if !self.room.pass_match(&passwd) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    fn is_started(&mut self) -> bool {
        self.room.is_racing_started()
    }

    fn set_start(&mut self) -> bool {
        self.room.set_racing_started()
    }

    fn get_race_brief(&mut self) -> RaceBrief {
        let mut racebrief = RaceBrief::default();
        racebrief.name = self.room.info.name.clone();
        racebrief.stage = self.room.info.stage.clone();
        racebrief.owner = self.room.info.owner.clone();
        racebrief.players = self.room.players.len() as u32;
        racebrief.state = self.room.room_state.clone();
        racebrief
    }

    fn get_race_config(&mut self) -> RaceInfo {
        self.room.info.clone()
    }

    fn update_race_config(&mut self, info: RaceInfo) {
        self.room.info = info;
    }

    fn get_player_config(&mut self, token: &String) -> Option<RaceConfig> {
        if let Some(player) = self.room.get_player(token) {
            return Some(player.race_cfg.clone());
        }
        None
    }

    fn update_player_config(&mut self, token: &String, cfg: RaceConfig) -> bool {
        if let Some(player) = self.room.get_player(token) {
            player.race_cfg = cfg;
            return true;
        }
        false
    }

    fn get_players_counts(&mut self) -> u32 {
        self.room.players.len() as u32
    }

    fn get_players_state(&mut self) -> Vec<RaceUserState> {
        let mut states = vec![];
        for player in &self.room.players {
            let state = RaceUserState {name: player.profile_name.clone(), state: player.state.clone()};
            states.push(state);
        }
        return states;
    }

    fn update_player_state(&mut self, token: &String, state: RaceState) -> bool {
        if let Some(player) = self.room.get_player(token) {
            player.state = state;
            return true;
        }
        false
    }

    fn update_player_data(&mut self, token: &String, data: MetaRaceData) -> bool {
        if let Some(player) = self.room.get_player(token) {
            player.race_data = data;
            return true;
        }
        false
    }

    fn framed_schedule(&mut self) {
        self.update_room_state();
        self.update_race_state();
    }
}

impl Customize {
    pub fn set_limit(&mut self, limit: usize) {
        self.room.set_limit(limit);
    }

    pub fn lock_with_passwd(&mut self, passwd: &String) {
        self.room.set_pass(passwd.clone());
    }

    fn update_room_state(&mut self) {
        self.room.update_room_state();
        let room = &mut self.room;
        if !room.is_player_exist(&room.info.owner.clone()) {
            if let Some(owner) = room.players.get(0) {
                room.info.owner = owner.profile_name.clone();
            }
        }

        if room.is_locked() {
            return;
        }
    }

    fn update_race_state(&mut self) {
        self.room.update_race_state();
    }
}
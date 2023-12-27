use protocol::httpapi::RaceCreate;
use protocol::httpapi::RaceJoin;
use protocol::httpapi::RaceUpdate;
use protocol::httpapi::RaceUserState;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Mutex;
use uuid::Uuid;
use protocol::httpapi::{UserLogin, UserLogout, RaceInfo, RaceBrief, RaceAccess, MetaRaceData};

use crate::lobby::RaceLobby;
use crate::player::LobbyPlayer;
use crate::room::RaceRoom;
use crate::player::RacePlayer;
use std::collections::HashMap;
use std::sync::Arc;
use protocol::httpapi::RoomState;

#[derive(Default)]
pub struct RacingServer {
    pub lobby: RaceLobby,
    pub rooms: HashMap<String, RaceRoom>,
}

impl RacingServer {
    pub fn is_raceroom_exist(&mut self, name: &String) -> bool {
        self.rooms.contains_key(name)
    }

    pub fn remove_empty_rooms(&mut self) {
        self.rooms.retain(|_k, v| !v.is_empty());
    }

    pub fn remove_invalid_players(&mut self) {
        for (_, room) in self.rooms.iter_mut() {
            room.players.retain(|x| self.lobby.is_player_exist(Some(&x.token), None));
        }
    }

    pub fn find_room_by_name_mut(&mut self, name: &String) -> Option<&mut RaceRoom> {
        if let Some(room) = self.rooms.get_mut(name) {
            return Some(room);
        }
        None
    }

    pub fn player_login(&mut self, user: UserLogin) -> Option<String> {
        if user.passwd != "simrallycn" {
            return None;
        }

        if let Some(token) = &self.lobby.get_token_by_name(user.name.clone()) {
            return Some(token.to_string());
        }

        let token = Uuid::new_v4();
        let tokenstr = token.to_string();
        let player: LobbyPlayer = LobbyPlayer {tokenstr: token.to_string(), profile_name: user.name};
        self.lobby.push_player(token, player);
        return Some(tokenstr);
    }

    pub fn player_access(&mut self, access: &RaceAccess) -> bool {
        if let Ok(token) = Uuid::parse_str(access.token.as_str()) {
            return self.lobby.is_player_exist(Some(&token), None);
        }
        return false;
    }

    pub fn player_logout(&mut self, user: UserLogout) -> bool {
        if let Ok(token) = Uuid::parse_str(&user.token) {
            if self.lobby.is_player_exist(Some(&token), None) {
                self.lobby.pop_player(&token);
                return true;
            }
        }
        return false;
    }

    pub fn get_raceroom_list(&self) -> Option<Vec<RaceBrief>> {
        if self.rooms.is_empty() {
            return None;
        }
    
        let mut racelist = vec![];
        for (name, room) in &self.rooms {
            let mut raceitem = RaceBrief::default();
            raceitem.name = name.clone();
            raceitem.stage = room.info.stage.clone();
            if let Some(player) = room.players.get(0) {
                raceitem.owner = player.profile_name.clone();
            }
            raceitem.state = room.room_state.clone();
            racelist.push(raceitem);
        }

        Some(racelist)
    }

    pub fn get_raceroom_info(&self, name: &String) -> Option<RaceInfo> {
        if let Some(room) = self.rooms.get(name) {
            return Some(room.info.clone());
        }

        None
    }

    pub fn get_raceroom_userstate(&self, name: &String) -> Option<Vec<RaceUserState>> {
        let mut results = vec![];
        if let Some(room) = self.rooms.get(name) {
            for player in &room.players {
                let result = RaceUserState {name: player.profile_name.clone(), state: player.state.clone()};
                results.push(result);
            }
            return Some(results);
        }

        None
    }

    pub fn create_raceroom(&mut self, create: RaceCreate) -> bool {
        if self.rooms.contains_key(&create.info.name) {
            return true;
        }

        if let Ok(token) = Uuid::parse_str(&create.token.as_str()) {
            if !self.lobby.is_player_exist(Some(&token), None) {
                return false;
            }

            if let Some(player) = self.lobby.get_player(token) {
                let mut raceroom = RaceRoom::default();
                raceroom.info = create.info.clone();
                if create.locked {
                    raceroom.room_state = RoomState::RoomLocked;
                } else {
                    raceroom.room_state = RoomState::RoomFree;
                }
                raceroom.passwd = create.passwd.clone();
                raceroom.players.insert(0, RacePlayer::new(&player.tokenstr, &player.profile_name));
                self.rooms.insert(create.info.name, raceroom);
                return true;
            }
        }
        return false;
    }

    pub fn join_raceroom(&mut self, join: RaceJoin) -> bool {
        if let Ok(token) = Uuid::parse_str(&join.token.as_str()) {
            if let Some(player) = self.lobby.get_player(token) {
                if let Some(room) = self.rooms.get_mut(&join.room) {
                    if let Some(pass) = join.passwd {
                        if !room.can_enter(&pass) {
                            return false;
                        }
                    }

                    if room.is_full() {
                        return false;
                    }

                    if room.is_player_exist(&player.profile_name) {
                        room.pop_player(&player.profile_name);
                    }
                    room.push_player(RacePlayer::new(&player.tokenstr, &player.profile_name));
                    return true;
                }
            }
        }
        return false;
    }

    pub fn leave_raceroom(&mut self, roomname: String, tokenstr: String) -> bool {
        if let Some(room) = self.rooms.get_mut(&roomname) {
            room.pop_player_by_token(&tokenstr);
            return true;
        }
        return false;
    }

    pub fn race_player_access(&mut self, access: &RaceAccess, writer: Arc<Mutex<OwnedWriteHalf>>) -> bool {
        if let Some(room) = self.rooms.get_mut(&access.room) {
            for player in room.players.iter_mut() {
                if player.tokenstr == access.token {
                    player.writer = Some(writer);
                    return true;
                }
            }
        }
        return false;
    }

    pub fn update_player_state(&mut self, update: &RaceUpdate) -> bool {
        if let Some(room) = self.rooms.get_mut(&update.room) {
            for player in room.players.iter_mut() {
                if player.tokenstr == update.token {
                    player.state = update.state.clone();
                    return true;
                }
            }
        }
        return false;
    }

    pub fn update_player_race_data(&mut self, data: MetaRaceData) -> bool {
        if let Some(room) = self.rooms.get_mut(&data.room) {
            for player in room.players.iter_mut() {
                if player.tokenstr == data.token {
                    player.race_data = data;
                    return true;
                }
            }
        }
        return false;
    }
}
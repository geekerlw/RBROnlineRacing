use uuid::Uuid;
use protocol::httpapi::{UserLogin, RaceInfo, RaceItem, RaceList, UserAccess, MetaRaceData};
use crate::lobby::RaceLobby;
use crate::room::RaceRoom;
use crate::player::RacePlayer;
use std::collections::HashMap;
use protocol::httpapi::RoomState;
use protocol::httpapi::RaceState;

#[derive(Default)]
pub struct RacingServer {
    pub lobby: RaceLobby,
    pub rooms: HashMap<String, RaceRoom>,
}

impl RacingServer {
    pub fn is_raceroom_exist(&mut self, name: &String) -> bool {
        self.rooms.contains_key(name)
    }

    pub fn do_self_check(&mut self) {
        self.rooms.retain(|_k, v| !v.is_empty());
    }

    pub fn find_player_by_token_mut(&mut self, tokenstr: &String) -> Option<&mut RacePlayer> {
        if let Ok(token) = Uuid::parse_str(tokenstr.as_str()) {
            if let Some(player) = self.lobby.get_player(token) {
                return Some(player);
            }
        }
        None
    }

    pub fn find_room_by_name_mut(&mut self, name: &String) -> Option<&mut RaceRoom> {
        if let Some(room) = self.rooms.get_mut(name) {
            return Some(room);
        }
        None
    }

    pub fn player_login(&mut self, user: UserLogin, token: Uuid) -> bool {
        if user.passwd != "simrallycn" {
            return false;
        }

        if !self.lobby.is_player_exist(None, Some(&user.name)) {
            let player: RacePlayer = RacePlayer::new(user.name);
            self.lobby.push_player(token, player);
            return true;
        }
        return false;
    }

    pub fn player_access(&mut self, access: &UserAccess) -> bool {
        if let Ok(token) = Uuid::parse_str(access.token.as_str()) {
            return self.lobby.is_player_exist(Some(&token), None);
        }
        return false;
    }

    pub fn player_logout(&mut self, tokenstr: String) -> bool {
        if let Ok(token) = Uuid::parse_str(&tokenstr.as_str()) {
            if self.lobby.is_player_exist(Some(&token), None) {
                self.lobby.pop_player(&token);
                return true;
            }
        }
        return false;
    }

    pub fn get_raceroom_list(&self) -> Option<RaceList> {
        if self.rooms.is_empty() {
            return None;
        }
    
        let mut racelist = RaceList::default();
        for (name, room) in &self.rooms {
            let mut raceitem = RaceItem::default();
            raceitem.name = name.clone();
            raceitem.stage = room.stage.clone();
            if let Some(owner) = room.players.get(0) {
                raceitem.owner = owner.clone();
            }
            racelist.room.push(raceitem);
        }

        Some(racelist)
    }

    pub fn get_raceroom_info(&self, name: &String) -> Option<RaceInfo> {
        if let Some(room) = self.rooms.get(name) {
            let mut raceinfo = RaceInfo::default();
            raceinfo.name = name.clone();
            raceinfo.stage = room.stage.clone();
            raceinfo.car = room.car.clone();
            raceinfo.damage = room.damage.clone();
            raceinfo.setup = room.setup.clone();
            raceinfo.state = room.state.clone();
            for player in &room.players {
                raceinfo.players.push(player.clone());
            }
            return Some(raceinfo);
        }

        None
    }

    pub fn create_raceroom(&mut self, info: RaceInfo) -> bool {
        if self.rooms.contains_key(&info.name) {
            return true;
        }

        if let Ok(token) = Uuid::parse_str(&info.token.as_str()) {
            if !self.lobby.is_player_exist(Some(&token), None) {
                return false;
            }

            if let Some(player) = self.lobby.get_player(token) {
                player.room_name = info.name.clone();
                let mut raceroom = RaceRoom::default();
                raceroom.stage = info.stage;
                if let Some(car) = info.car {
                    raceroom.car = Some(car);
                }
                if let Some(damage) = info.damage {
                    raceroom.damage = Some(damage);
                }
                if let Some(setup) = info.setup {
                    raceroom.setup = Some(setup);
                }
                raceroom.state = RoomState::default();
                raceroom.players.insert(0, player.profile_name.clone());
                self.rooms.insert(info.name, raceroom);
                return true;
            }
        }
        return false;
    }

    pub fn join_raceroom(&mut self, roomname: String, tokenstr: String) -> bool {
        if let Ok(token) = Uuid::parse_str(&tokenstr.as_str()) {
            if let Some(player) = self.lobby.get_player(token) {
                if !player.room_name.is_empty() {
                    if let Some(room) = self.rooms.get_mut(&player.room_name) {
                        room.pop_player(&player.profile_name);
                    }
                }
                if let Some(room) = self.rooms.get_mut(&roomname) {
                    if room.is_player_exist(&player.profile_name) {
                        return true;
                    }
                    player.room_name = roomname.clone();
                    room.push_player(player.profile_name.clone());
                    return true;
                }
            }
        }
        return false;
    }

    pub fn leave_raceroom(&mut self, tokenstr: &String) -> bool {
        if let Ok(token) = Uuid::parse_str(&tokenstr.as_str()) {
            if let Some(player) = self.lobby.get_player(token) {
                if let Some(room) = self.rooms.get_mut(&player.room_name) {
                    room.pop_player(&player.profile_name);
                    player.room_name.clear();
                    return true;
                }
            }
        }
        return false;
    }

    pub fn update_player_state(&mut self, tokenstr: &String, state: RaceState) -> bool {
        if let Ok(token) = Uuid::parse_str(tokenstr.as_str()) {
            if let Some(player) = self.lobby.get_player(token) {
                player.state = state;
                return true;
            }
        }
        return false;
    }

    pub fn update_player_race_data(&mut self, data: MetaRaceData) -> bool {
        if let Ok(token) = Uuid::parse_str(&data.token.as_str()) {
            if let Some(player) = self.lobby.get_player(token) {
                player.race_data = data;
                return true;
            }
        }
        return false;
    }

    // only can be used in racing on.
    pub fn get_room_all_players(&mut self, tokenstr: &String) -> Option<Vec<RacePlayer>> {
        let mut players = Vec::<RacePlayer>::new();
        if let Ok(token) = Uuid::parse_str(&tokenstr) {
            if let Some(player) = self.lobby.get_player(token) {
                if let Some(room) = self.rooms.get(&player.room_name) {
                    for profile_name in &room.players {
                        if let Some(item) = self.lobby.get_player_by_name(profile_name.clone()) {
                            players.push(item.clone());
                        }
                    }
                    return Some(players);
                }
            }
        }

        None
    }
}
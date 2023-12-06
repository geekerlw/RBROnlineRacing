use uuid::Uuid;
use protocol::httpapi::{RaceInfo, RaceItem, RaceList};
use crate::lobby::RaceLobby;
use crate::player::RacePlayer;
use crate::room::RaceRoom;
use std::collections::HashMap;
use protocol::httpapi::RacePlayerState;

#[derive(Default)]
pub struct RacingServer {
    pub count: i32,
    pub lobby: RaceLobby,
    pub rooms: HashMap<String, RaceRoom>,
}

impl RacingServer {
    pub fn is_raceroom_exist(&mut self, name: &String) -> bool {
        self.rooms.contains_key(name)
    }

    pub fn player_login(&mut self, player: RacePlayer) {
        self.lobby.push_player(player.user_token, player);
    }

    pub fn player_logout(&mut self, tokenstr: String) -> bool {
        if let Ok(token) = Uuid::parse_str(&tokenstr.as_str()) {
            self.lobby.pop_player(&token);
            return true;
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

    pub fn create_raceroom(&mut self, info: RaceInfo) {
        if self.rooms.contains_key(&info.name) {
            return;
        }

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
        raceroom.state = RacePlayerState::default();
        self.rooms.insert(info.name, raceroom);
    }

    pub fn join_raceroom(&mut self, roomname: String, tokenstr: String) -> bool {
        if let Ok(token) = Uuid::parse_str(&tokenstr.as_str()) {
            if let Some(player) = self.lobby.get_player(token) {
                player.room_name = roomname.clone();
                if let Some(room) = self.rooms.get_mut(&roomname) {
                    room.push_player(player.profile_name.clone());
                    return true;
                }
            }
        }
        return false;
    }

    pub fn leave_raceroom(&mut self, tokenstr: String) -> bool {
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

    pub fn update_player_state(&mut self, tokenstr: String, state: RacePlayerState) -> bool {
        if let Ok(token) = Uuid::parse_str(&tokenstr.as_str()) {
            if let Some(player) = self.lobby.get_player(token) {
                player.state = state;
                return true;
            }
        }
        return false;
    }
}
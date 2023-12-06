use uuid::Uuid;
use protocol::httpapi::{RaceInfo, RaceItem, RaceList};
use crate::lobby::RaceLobby;
use crate::player::RacePlayer;
use crate::room::RaceRoom;
use std::collections::HashMap;

#[derive(Default)]
pub struct RacingServer {
    pub count: i32,
    pub lobby: RaceLobby,
    pub room: HashMap<String, RaceRoom>,
}

impl RacingServer {
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
        if self.room.is_empty() {
            return None;
        }
    
        let mut racelist = RaceList::default();
        for (name, room) in &self.room {
            let mut raceitem = RaceItem::default();
            raceitem.name = name.clone();
            raceitem.stage = room.stage.clone();
            if let Some(owner) = room.players.get(0) {
                raceitem.owner = owner.profile_name.clone();
            }
            racelist.room.push(raceitem);
        }

        Some(racelist)
    }

    pub fn get_raceroom_info(&self, name: &String) -> Option<RaceInfo> {
        if let Some(room) = self.room.get(name) {
            let mut raceinfo = RaceInfo::default();
            raceinfo.name = name.clone();
            raceinfo.stage = room.stage.clone();
            raceinfo.car = room.car.clone();
            raceinfo.damage = room.damage.clone();
            raceinfo.setup = room.setup.clone();
            raceinfo.state = room.state.clone();
            for player in &room.players {
                raceinfo.players.push(player.profile_name.clone());
            }
            return Some(raceinfo);
        }

        None
    }

    pub fn create_raceroom(&mut self, info: RaceInfo) {
        if self.room.contains_key(&info.name) {
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
        raceroom.state = 0;
        self.room.insert(info.name, raceroom);
    }

    pub fn join_raceroom(&mut self) {

    }

    pub fn leave_raceroom(&mut self) {

    }

    pub fn update_player_state(&mut self) {

    }
}
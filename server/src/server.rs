use uuid::Uuid;

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
        self.lobby.push_player(player);
    }

    pub fn player_logout(&mut self, tokenstr: String) -> bool {
        if let Ok(token) = Uuid::parse_str(&tokenstr.as_str()) {
            self.lobby.pop_player(&token);
            self.lobby.pop_player(&token);
            return true;
        }
        return false;
    }
}
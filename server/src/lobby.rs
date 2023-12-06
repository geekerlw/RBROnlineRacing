use uuid::Uuid;
use std::collections::HashMap;
use crate::player::RacePlayer;

#[derive(Default)]
pub struct RaceLobby {
    players: HashMap<Uuid, RacePlayer>,
}

impl RaceLobby {
    pub fn push_player(&mut self, token: Uuid, player: RacePlayer) {
        if !self.players.contains_key(&token) {
            self.players.insert(token, player);
        }
    }

    pub fn pop_player(&mut self, token: &Uuid) {
        self.players.retain(|k, _v| k == token);
    }

    pub fn is_player_exist(&mut self, token: &Uuid) -> bool {
        self.players.contains_key(token)
    }

    pub fn get_player(&mut self, token: Uuid) -> Option<&mut RacePlayer> {
        if let Some(player) = self.players.get_mut(&token) {
            return Some(player);
        }
        None
    }
}
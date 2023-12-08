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
            println!("steven: push in num: {}", self.players.len());
        }
    }

    pub fn pop_player(&mut self, token: &Uuid) {
        self.players.retain(|k, _v| k != token);
        println!("steven: remain num: {}", self.players.len());
    }

    pub fn is_player_exist(&mut self, token: Option<&Uuid>, name: Option<&String>) -> bool {
        if let Some(token) = token {
            return self.players.contains_key(token);
        } else if let Some(name) = name {
            for (_k, player) in &self.players {
                if &player.profile_name == name {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn get_player(&mut self, token: Uuid) -> Option<&mut RacePlayer> {
        if let Some(player) = self.players.get_mut(&token) {
            return Some(player);
        }
        None
    }

    pub fn get_player_by_name(&mut self, name: String) -> Option<RacePlayer> {
        for (_, player) in &self.players {
            if player.profile_name == name {
                return Some(player.clone());
            }
        }
        None
    }
}
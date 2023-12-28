use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::player::LobbyPlayer;

#[derive(Default, Serialize, Deserialize)]
pub struct RaceLobby {
    players: HashMap<Uuid, LobbyPlayer>,
}

impl RaceLobby {
    pub fn push_player(&mut self, token: Uuid, player: LobbyPlayer) {
        if !self.players.contains_key(&token) {
            self.players.insert(token, player);
        }
    }

    pub fn pop_player(&mut self, token: &Uuid) {
        self.players.retain(|k, _v| k != token);
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

    pub fn get_player(&mut self, token: Uuid) -> Option<&mut LobbyPlayer> {
        if let Some(player) = self.players.get_mut(&token) {
            return Some(player);
        }
        None
    }

    pub fn get_player_by_name(&mut self, name: String) -> Option<LobbyPlayer> {
        for (_, player) in &self.players {
            if player.profile_name == name {
                return Some(player.clone());
            }
        }
        None
    }

    pub fn get_token_by_name(&mut self, name: String) -> Option<Uuid> {
        for (uuid, player) in &self.players {
            if player.profile_name == name {
                return Some(uuid.clone());
            }
        }
        None
    }
}
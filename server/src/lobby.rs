use uuid::Uuid;

use crate::player::RacePlayer;

#[derive(Default)]
pub struct RaceLobby {
    players: Vec<RacePlayer>,
}

impl RaceLobby {
    pub fn push_player(&mut self, player: RacePlayer) {
        self.players.push(player);
    }

    pub fn pop_player(&mut self, token: &Uuid) {
        self.players.retain(|x| x.user_token == *token);
    }
}
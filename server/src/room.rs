use crate::player::RacePlayer;
use uuid::Uuid;

#[derive(Default)]
pub struct RaceRoom {
    pub name: String,
    pub players: Vec<RacePlayer>,
}

impl RaceRoom {
    pub fn push_player(&mut self, player: RacePlayer) {
        self.players.push(player);
    }

    pub fn pop_player(&mut self, token: &Uuid) {
        self.players.retain(|x| x.user_token == *token);
    }
}
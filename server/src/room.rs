use crate::player::RacePlayer;
use uuid::Uuid;

#[derive(Default)]
pub struct RaceRoom {
    pub stage: String,
    pub car: Option<String>,
    pub damage: Option<u32>,
    pub setup: Option<String>,
    pub players: Vec<RacePlayer>,
    pub state: u32,
}

impl RaceRoom {
    pub fn push_player(&mut self, player: RacePlayer) {
        self.players.push(player);
    }

    pub fn pop_player(&mut self, token: &Uuid) {
        self.players.retain(|x| x.user_token == *token);
    }
}
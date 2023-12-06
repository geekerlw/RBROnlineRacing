use protocol::httpapi::RacePlayerState;

#[derive(Default)]
pub struct RaceRoom {
    pub stage: String,
    pub car: Option<String>,
    pub damage: Option<u32>,
    pub setup: Option<String>,
    pub players: Vec<String>,
    pub state: RacePlayerState,
}

impl RaceRoom {
    pub fn push_player(&mut self, player: String) {
        self.players.push(player);
    }

    pub fn pop_player(&mut self, player: &String) {
        self.players.retain(|x| x == player);
    }
}
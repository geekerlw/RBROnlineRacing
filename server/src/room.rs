use protocol::httpapi::RoomState;

#[derive(Default)]
pub struct RaceRoom {
    pub stage: String,
    pub stage_id: u32,
    pub car: Option<String>,
    pub car_id: Option<u32>,
    pub damage: u32,
    pub players: Vec<String>,
    pub state: RoomState,
}

impl RaceRoom {
    pub fn push_player(&mut self, player: String) {
        self.players.push(player);
    }

    pub fn pop_player(&mut self, player: &String) {
        self.players.retain(|x| x != player);
    }

    pub fn is_player_exist(&mut self, name: &String) -> bool {
        self.players.contains(name)
    }

    pub fn is_empty(&mut self) -> bool {
        self.players.is_empty()
    }
}
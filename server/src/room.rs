use protocol::httpapi::RacePlayerState;
use protocol::httpapi::MetaRaceData;

#[derive(Default)]
pub struct RaceRoom {
    pub stage: String,
    pub car: Option<String>,
    pub damage: Option<u32>,
    pub setup: Option<String>,
    pub players: Vec<String>,
    pub state: RacePlayerState,
    pub racedata: Vec<MetaRaceData>,
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
}
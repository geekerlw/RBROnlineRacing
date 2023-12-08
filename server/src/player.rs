use tokio::net::TcpStream;
use protocol::httpapi::{RaceState, MetaRaceData};

pub struct RacePlayer {
    pub profile_name: String,
    pub room_name: String,
    pub tcpstream: Option<TcpStream>,
    pub state: RaceState,
    pub race_data: MetaRaceData,
}

impl Default for RacePlayer {
    fn default() -> Self {
        Self {
            profile_name: String::from("anonymous"),
            room_name: String::new(),
            tcpstream: None,
            state: RaceState::default(),
            race_data: MetaRaceData::default(),
        }
    }
}

impl RacePlayer {
    pub fn new(username: String) -> Self {
        Self {
            profile_name: username,
            room_name: String::new(),
            tcpstream: None,
            state: RaceState::default(),
            race_data: MetaRaceData::default(),
        }
    }
}
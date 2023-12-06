use tokio::net::TcpStream;
use uuid::Uuid;
use protocol::httpapi::RacePlayerState;

pub struct RacePlayer {
    pub user_token: Uuid,
    pub profile_name: String,
    pub room_name: String,
    tcpstream: Option<TcpStream>,
    pub state: RacePlayerState,
}

impl Default for RacePlayer {
    fn default() -> Self {
        Self {
            user_token: Uuid::new_v4(),
            profile_name: String::from("anonymous"),
            room_name: String::new(),
            tcpstream: None,
            state: RacePlayerState::default()
        }
    }
}

impl RacePlayer {
    pub fn new(token: Uuid, username: String) -> Self {
        Self {
            user_token: token,
            profile_name: username,
            room_name: String::new(),
            tcpstream: None,
            state: RacePlayerState::default()
        }
    }
}
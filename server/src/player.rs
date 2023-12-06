use tokio::net::TcpStream;
use uuid::Uuid;

pub enum RacePlayerState {
    RaceFree,
    RaceJoined,
    RaceLoaded,
    RaceRunning,
    RaceRetired,
    RaceFinished,
}

pub struct RacePlayer {
    pub profile_name: String,
    pub user_token: Uuid,
    tcpstream: Option<TcpStream>,
    pub state: RacePlayerState,
}

impl Default for RacePlayer {
    fn default() -> Self {
        Self { 
            profile_name: String::from("anonymous"),
            user_token: Uuid::new_v4(),
            tcpstream: None,
            state: RacePlayerState::RaceFree
        }
    }
}

impl RacePlayer {
    pub fn new(username: String) -> Self {
        Self {
            profile_name: username,
            user_token: Uuid::new_v4(),
            tcpstream: None,
            state: RacePlayerState::RaceFree
        }
    }
}
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
    pub user_token: Uuid,
    pub profile_name: String,
    tcpstream: Option<TcpStream>,
    pub state: RacePlayerState,
}

impl Default for RacePlayer {
    fn default() -> Self {
        Self {
            user_token: Uuid::new_v4(),
            profile_name: String::from("anonymous"),
            tcpstream: None,
            state: RacePlayerState::RaceFree
        }
    }
}

impl RacePlayer {
    pub fn new(token: Uuid, username: String) -> Self {
        Self {
            user_token: token,
            profile_name: username,
            tcpstream: None,
            state: RacePlayerState::RaceFree
        }
    }
}
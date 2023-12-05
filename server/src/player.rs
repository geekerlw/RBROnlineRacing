use tokio::net::TcpStream;

pub enum RacePlayerState {
    RaceFree,
    RaceJoined,
    RaceLoaded,
    RaceRunning,
    RaceRetired,
    RaceFinished,
}

pub struct RacePlayer {
    profile_name: String,
    tcpstream: Option<TcpStream>,
    state: RacePlayerState,
}

impl Default for RacePlayer {
    fn default() -> Self {
        Self { profile_name: String::from("player"), tcpstream: None, state: RacePlayerState::RaceFree }
    }
}
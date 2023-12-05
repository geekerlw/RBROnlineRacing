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
    tcpstream: TcpStream,
    state: RacePlayerState,
}
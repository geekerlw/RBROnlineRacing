use crate::lobby::RaceLobby;
use crate::room::RaceRoom;

#[derive(Default)]
pub struct RacingServer {
    pub count: i32,
    lobby: RaceLobby,
    room: RaceRoom,
}
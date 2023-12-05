use crate::player::RacePlayer;

#[derive(Default)]
pub struct RaceLobby {
    name: String,
    players: Vec<RacePlayer>,
}
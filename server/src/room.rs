use crate::player::RacePlayer;

#[derive(Default)]
pub struct RaceRoom {
    name: String,
    players: Vec<RacePlayer>,
}
use crate::components::store::RacingStore;

pub mod leaderboard;
pub mod progressbar;

pub trait Overlay {
    fn init(mut self) -> Self;

    fn draw(&self, _store: &RacingStore);
}
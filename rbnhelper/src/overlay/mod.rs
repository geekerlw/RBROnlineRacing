use crate::components::store::RacingStore;

pub mod copyright;
pub mod scoreboard;
pub mod news;

pub trait Overlay {
    fn init(&mut self, width: i16, height: i16);

    fn update(&mut self, _store: &RacingStore) {}

    fn draw_ui(&mut self);
}
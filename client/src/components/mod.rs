pub mod finish;
pub mod loading;
pub mod lobby;
pub mod racing;
pub mod setting;
pub mod login;
pub mod create;
pub mod inroom;

use crate::route::RacingRoute;
use crate::store::RacingStore;

pub trait UiView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, route: &mut RacingRoute, store: &mut RacingStore);
}
pub mod index;
pub mod finish;
pub mod loading;
pub mod lobby;
pub mod racing;
pub mod setting;
pub mod login;
pub mod create;
pub mod inroom;

use crate::components::route::RacingRoute;
use crate::components::store::RacingStore;

#[derive(Default, Clone)]
pub enum UiPageState {
    #[default]
    PageLogin,
    PageLobby,
    PageCreate,
    PageInRoom,
    PageLoading,
    PageRacing,
    PageFinish,
    PageSetting,
}

pub trait UiView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, route: &mut RacingRoute, store: &mut RacingStore);
}
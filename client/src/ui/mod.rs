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
use tokio::sync::mpsc::Sender;

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

pub enum UiMsg {
    MsgGotoPage(UiPageState),
}

pub struct UiPageCtx {
    pub store: RacingStore,
    pub route: RacingRoute,
    pub tx: Sender<UiMsg>,
}

impl UiPageCtx {
    pub fn new(tx: Sender<UiMsg>) -> Self {
        Self {
            store: RacingStore::default(),
            route: RacingRoute::default(),
            tx,
        }
    }
}

pub trait UiView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx);
}
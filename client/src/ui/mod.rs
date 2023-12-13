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
use tokio::sync::mpsc::{channel, Sender, Receiver};

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
    MsgUserLogined(String),
}

pub struct UiPageCtx {
    pub store: RacingStore,
    pub route: RacingRoute,
    pub tx: Sender<UiMsg>,
    pub rx: Receiver<UiMsg>,
}

impl Default for UiPageCtx {
    fn default() -> Self {
        let (tx, mut rx) = channel::<UiMsg>(32);
        Self {
            store: RacingStore::default(),
            route: RacingRoute::default(),
            tx,
            rx,
        }
    }
}

pub trait UiView {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx);
}
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

#[derive(Default, Clone, PartialEq)]
pub enum UiPageState {
    #[default]
    PageLogin = 0,
    PageLobby = 1,
    PageCreate = 2,
    PageInRoom = 3,
    PageLoading = 4,
    PageRacing = 5,
    PageFinish = 6,
    PageSetting = 7,
}

pub enum UiMsg {
    MsgGotoPage(UiPageState),
    MsgUserLogined(String),
    MsgRaceRoomCreated(serde_json::Value),
}

pub struct UiPageCtx {
    pub store: RacingStore,
    pub route: RacingRoute,
    pub tx: Sender<UiMsg>,
    pub rx: Receiver<UiMsg>,
}

impl Default for UiPageCtx {
    fn default() -> Self {
        let (tx, rx) = channel::<UiMsg>(32);
        Self {
            store: RacingStore::default(),
            route: RacingRoute::default(),
            tx,
            rx,
        }
    }
}

pub trait UiView {
    fn init(&mut self) {}

    fn enter(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, _page: &mut UiPageCtx) {}

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, page: &mut UiPageCtx);

    fn exit(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, _page: &mut UiPageCtx) {}

    fn quit(&mut self) {}

    fn set_param(&mut self, _value: serde_json::Value) {}
}
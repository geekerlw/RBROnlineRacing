use eframe::egui;
use egui::RichText;
use tokio::sync::mpsc::{Sender, Receiver};
use crate::components::store::RacingStore;
use crate::components::route::RacingRoute;
use crate::ui::UiPageState;
use super::UiView;

enum UiRacingMsg {
    MsgGotoPage(UiPageState),
}

pub struct UiRacing {
    pub state: bool,
    tx: Sender<UiRacingMsg>,
    rx: Receiver<UiRacingMsg>,
}

impl Default for UiRacing {
    fn default() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel::<UiRacingMsg>(16);
        Self {
            state: false,
            tx,
            rx,
        }
    }
}

impl UiView for UiRacing {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, route: &mut RacingRoute, store: &mut RacingStore) {
        if let Ok(msg) = self.rx.try_recv() {
            match msg {
                UiRacingMsg::MsgGotoPage(page) => route.switch_to_page(page),
            };
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("比赛进行中...").size(40.0));
                if !self.state {
                    self.state = true;
                    let tx_clone = self.tx.clone();
                    let ctx_clone = ctx.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                        tx_clone.clone().send(UiRacingMsg::MsgGotoPage(UiPageState::PageFinish)).await.unwrap();
                        ctx_clone.request_repaint();
                    });
                };
            });
        });
    }
}
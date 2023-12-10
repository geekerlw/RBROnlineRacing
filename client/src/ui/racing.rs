use eframe::egui;
use egui::RichText;
use crate::{store::RacingStore, UiPageState};
use tokio::sync::mpsc::{Sender, Receiver};

enum UiRacingMsg {
    MsgGotoPage(UiPageState),
}

pub struct UiRacing {
    pub state: bool,
    pub tx: Sender<UiRacingMsg>,
    pub rx: Receiver<UiRacingMsg>,
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

impl UiRacing {
    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, store: &mut RacingStore) {
        if let Ok(msg) = self.rx.try_recv() {
            match msg {
                UiRacingMsg::MsgGotoPage(page) => store.switch_to_page(page),
            };
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("比赛进行中...").size(40.0));
                if !self.state {
                    self.state = true;
                    let tx_clone = self.tx.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                        tx_clone.clone().send(UiRacingMsg::MsgGotoPage(UiPageState::PageFinish)).await.unwrap();
                    });
                };
            });
        });
    }
}
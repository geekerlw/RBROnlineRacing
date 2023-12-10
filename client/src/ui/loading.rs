use eframe::egui;
use egui::RichText;
use crate::{store::RacingStore, UiPageState};
use tokio::sync::mpsc::{Sender, Receiver};

enum UiLoadingMsg {
    MsgGotoPage(UiPageState),
}

pub struct UiLoading {
    pub state: bool,
    pub tx: Sender<UiLoadingMsg>,
    pub rx: Receiver<UiLoadingMsg>,
}

impl Default for UiLoading {
    fn default() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel::<UiLoadingMsg>(16);
        Self {
            state: false,
            tx,
            rx,
        }
    }
}

impl UiLoading {
    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, store: &mut RacingStore) {
        if let Ok(msg) = self.rx.try_recv() {
            match msg {
                UiLoadingMsg::MsgGotoPage(page) => {
                    store.switch_to_page(page)
                },
            };
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("游戏加载中...").size(40.0));
                if !self.state {
                    self.state = true;
                    let tx_clone = self.tx.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                        tx_clone.clone().send(UiLoadingMsg::MsgGotoPage(UiPageState::PageRacing)).await.unwrap();
                    });
                };
            });
        });
    }
}
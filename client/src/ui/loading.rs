use eframe::egui;
use egui::RichText;
use tokio::sync::mpsc::{Sender, Receiver};
use crate::ui::UiPageState;
use super::{UiView, UiPageCtx, UiMsg};


#[derive(Default)]
pub struct UiLoading {
    pub state: bool,
}

impl UiView for UiLoading {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("游戏加载中...").size(40.0));
                if !self.state {
                    self.state = true;
                    let tx_clone = page.tx.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                        tx_clone.clone().send(UiMsg::MsgGotoPage(UiPageState::PageRacing)).await.unwrap();
                    });
                };
            });
        });
    }
}
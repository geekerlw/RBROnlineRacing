use eframe::egui;
use crate::{route::RacingRoute, UiPageState};
use crate::store::RacingStore;
use super::PageView;

#[derive(Default, Clone)]
pub struct UiSetting {
    pub content: String,
}

impl PageView for UiSetting {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, route: &mut RacingRoute, store: &mut RacingStore) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(200.0);
                ui.label("Not support now");
            });
        });
    }
}
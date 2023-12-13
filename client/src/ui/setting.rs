use eframe::egui;
use super::{UiView, UiPageCtx};

#[derive(Default, Clone)]
pub struct UiSetting {
    pub content: String,
}

impl UiView for UiSetting {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(200.0);
                ui.label("Not support now");
            });
        });
    }
}
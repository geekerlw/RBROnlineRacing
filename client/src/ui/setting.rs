use eframe::egui;
use crate::components::store::RacingStore;
use crate::components::route::RacingRoute;
use crate::ui::UiPageState;
use super::UiView;

#[derive(Default, Clone)]
pub struct UiSetting {
    pub content: String,
}

impl UiView for UiSetting {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, route: &mut RacingRoute, store: &mut RacingStore) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(200.0);
                ui.label("Not support now");
            });
        });
    }
}
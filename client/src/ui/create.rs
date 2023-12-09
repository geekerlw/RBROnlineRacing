use eframe::egui;
use egui::RichText;

#[derive(Default, Clone)]
pub struct UiCreateRace {
}

impl UiCreateRace {
    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("创建房间...").size(40.0));
            });
        });
    }
}
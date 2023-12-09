use eframe::egui;

#[derive(Default, Clone)]
pub struct UiSetting {
    pub content: String,
}

impl UiSetting {
    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(200.0);
                ui.label("Not support now");
            });
        });
    }
}
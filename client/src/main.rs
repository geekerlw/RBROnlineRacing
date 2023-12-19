use client::RacingClient;
use components::store::RacingStore;

mod ui;
mod game;
mod components;
mod client;

#[tokio::main]
async fn main() {
    let mut app = RacingClient::default().init();
    app.ctx.store = RacingStore::default().init().load_config();

    let mut native_options: eframe::NativeOptions = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2::new(1000.0, 600.0));
    native_options.initial_window_pos = Some(egui::Pos2::new(1280.0, 300.0));
    eframe::run_native("SimRallyCN Online Racing", native_options, Box::new(|cc| 
        Box::new(app.configure_font(&cc.egui_ctx)))).unwrap();
}
//#![windows_subsystem = "windows"]

use client::RacingClient;

mod ui;
mod game;
mod components;
mod client;

#[tokio::main]
async fn main() {
    let app = RacingClient::default().init();
    let mut native_options: eframe::NativeOptions = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2::new(1000.0, 600.0));
    native_options.centered = true;
    if let Ok(icon) = eframe::IconData::try_from_png_bytes(include_bytes!(r"..\icon.png")) {
        native_options.icon_data = Some(icon);
    }
    
    eframe::run_native("模拟拉力对战平台", native_options, Box::new(|cc| 
        Box::new(app.configure_font(&cc.egui_ctx)))).unwrap();
}
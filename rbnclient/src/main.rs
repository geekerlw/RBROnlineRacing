#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use client::RacingClient;
use simplelog::WriteLogger;

mod ui;
mod game;
mod components;
mod client;

#[tokio::main]
async fn main() {
    if let Ok(appdata) = std::env::var("AppData") {
        let log_path = appdata + r"\RBROnlineRacing";
        let log_file = log_path.clone() + r"\Debug.log";
        let path = Path::new(&log_path);
        if !path.exists() {
            std::fs::create_dir(path).unwrap();
        }
        WriteLogger::init(log::LevelFilter::Info, 
            simplelog::Config::default(), std::fs::File::create(log_file).unwrap()).unwrap();
    }

    let app = RacingClient::default().init();
    let mut native_options: eframe::NativeOptions = eframe::NativeOptions::default();
    native_options.centered = true;
    if let Ok(icon) = eframe::icon_data::from_png_bytes(include_bytes!(r"..\icon.png")) {
        native_options.viewport = egui::ViewportBuilder::default()
        .with_icon(icon)
        .with_title("模拟拉力对战平台 - V".to_string() + std::env!("CARGO_PKG_VERSION")).with_inner_size([1000.0, 600.0]);
    }
    
    eframe::run_native("模拟拉力对战平台", native_options, Box::new(|cc| 
        Box::new(app.configure_font(&cc.egui_ctx)))).unwrap();
}
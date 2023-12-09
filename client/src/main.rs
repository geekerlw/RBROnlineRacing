use client::client::RacingClient;
use client::UiPageState;

use protocol::httpapi::RaceState;

#[tokio::main]
async fn main() {
    let mut app = RacingClient::default();
    app.store.curr_page = UiPageState::PageLogin;
    app.store.user_name = String::from("Lw_Ziye");
    app.store.user_passwd = String::from("simrallycn");
    app.store.user_state = RaceState::RaceInit;

    let mut native_options: eframe::NativeOptions = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2::new(1000.0, 600.0));
    native_options.initial_window_pos = Some(egui::Pos2::new(1280.0, 300.0));
    eframe::run_native("SimRallyCn Online Racing", native_options, Box::new(|cc| 
        Box::new(app.configure_font(&cc.egui_ctx)))).unwrap();
}
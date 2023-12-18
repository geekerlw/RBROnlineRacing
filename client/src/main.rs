use ui::UiPageState;
use protocol::httpapi::RaceState;
use client::RacingClient;

mod ui;
mod game;
mod components;
mod client;

#[tokio::main]
async fn main() {
    let mut app = RacingClient::default().init();
    app.ctx.route.prev_page = UiPageState::PageLogin;
    app.ctx.route.curr_page = UiPageState::PageLogin;
    app.ctx.store.server_addr = "127.0.0.1".to_string();
    app.ctx.store.server_port = 8080;
    app.ctx.store.meta_port = 9493;
    app.ctx.store.user_name = String::from("Lw_Ziye");
    app.ctx.store.user_passwd = String::from("simrallycn");
    app.ctx.store.user_state = RaceState::RaceInit;

    let mut native_options: eframe::NativeOptions = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2::new(1000.0, 600.0));
    native_options.initial_window_pos = Some(egui::Pos2::new(1280.0, 300.0));
    eframe::run_native("SimRallyCN Online Racing", native_options, Box::new(|cc| 
        Box::new(app.configure_font(&cc.egui_ctx)))).unwrap();
}
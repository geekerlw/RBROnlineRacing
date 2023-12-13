use client::ui::UiPageState;
use protocol::httpapi::RaceState;
use client::ui::index::UiRacingApp;
use client::ui::{UiPageCtx, UiMsg};
use tokio::sync::mpsc::{channel, Sender, Receiver};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = channel::<UiMsg>(32);
    let mut ctx = UiPageCtx::new(tx);
    ctx.route.prev_page = UiPageState::PageLogin;
    ctx.route.curr_page = UiPageState::PageLogin;
    ctx.store.server_addr = "127.0.0.1".to_string();
    ctx.store.server_port = 8080;
    ctx.store.user_name = String::from("Lw_Ziye");
    ctx.store.user_passwd = String::from("simrallycn");
    ctx.store.user_state = RaceState::RaceInit;
    let app = UiRacingApp::new(ctx, rx);

    let mut native_options: eframe::NativeOptions = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2::new(1000.0, 600.0));
    native_options.initial_window_pos = Some(egui::Pos2::new(1280.0, 300.0));
    eframe::run_native("SimRallyCN Online Racing", native_options, Box::new(|cc| 
        Box::new(app.configure_font(&cc.egui_ctx)))).unwrap();
}
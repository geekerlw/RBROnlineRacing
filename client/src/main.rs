use client::client::RacingClient;
use client::UiPageState;
use client::store::RacingStore;
use protocol::httpapi::RaceState;

#[tokio::main]
async fn main() {
    let mut app = RacingClient::default();
    let mut store = RacingStore::default();
    store.curr_page = UiPageState::PageLogin;
    store.user_name = String::from("Lw_Ziye");
    store.user_passwd = String::from("simrallycn");
    store.user_state = RaceState::RaceInit;
    app.store = store;

    let mut native_options: eframe::NativeOptions = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2::new(1000.0, 600.0));
    native_options.initial_window_pos = Some(egui::Pos2::new(1280.0, 300.0));
    eframe::run_native("SimRallyCN Online Racing", native_options, Box::new(|cc| 
        Box::new(app.configure_font(&cc.egui_ctx)))).unwrap();
}
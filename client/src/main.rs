use client::ui::UiPageState;
use client::components::store::RacingStore;
use client::components::route::RacingRoute;
use protocol::httpapi::RaceState;
use client::ui::index::UiRacingApp;

#[tokio::main]
async fn main() {
    let mut app = UiRacingApp::default();
    let mut store: RacingStore = RacingStore::default();
    let mut route = RacingRoute::default();
    route.prev_page = UiPageState::PageLogin;
    route.curr_page = UiPageState::PageLogin;
    store.server_addr = "127.0.0.1".to_string();
    store.server_port = 8080;
    store.user_name = String::from("Lw_Ziye");
    store.user_passwd = String::from("simrallycn");
    store.user_state = RaceState::RaceInit;
    app.store = store;
    app.route = route;

    let mut native_options: eframe::NativeOptions = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2::new(1000.0, 600.0));
    native_options.initial_window_pos = Some(egui::Pos2::new(1280.0, 300.0));
    eframe::run_native("SimRallyCN Online Racing", native_options, Box::new(|cc| 
        Box::new(app.configure_font(&cc.egui_ctx)))).unwrap();
}
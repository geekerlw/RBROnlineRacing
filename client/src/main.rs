use client::client::RacingClient;

fn main() {
    let app = RacingClient::default();

    let mut native_options: eframe::NativeOptions = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2::new(1000.0, 600.0));
    eframe::run_native("SimRallyCn Online Racing", native_options, Box::new(|cc| 
        Box::new(app.configure_font(&cc.egui_ctx)))).unwrap();
}
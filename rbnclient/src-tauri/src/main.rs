// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_user_name() -> String {
    "anonymous".to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_user_name
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



// mod game;
// mod components;
// mod client;

// #[tokio::main]
// async fn main() {
//     if let Ok(appdata) = std::env::var("AppData") {
//         let log_path = appdata + r"\RBROnlineRacing";
//         let log_file = log_path.clone() + r"\Debug.log";
//         let path = Path::new(&log_path);
//         if !path.exists() {
//             std::fs::create_dir(path).unwrap();
//         }
//         WriteLogger::init(log::LevelFilter::Info, 
//             simplelog::Config::default(), std::fs::File::create(log_file).unwrap()).unwrap();
//     }

//     let app = RacingClient::default().init();
//     let mut native_options: eframe::NativeOptions = eframe::NativeOptions::default();
//     native_options.centered = true;
//     if let Ok(icon) = eframe::icon_data::from_png_bytes(include_bytes!(r"..\icon.png")) {
//         native_options.viewport = egui::ViewportBuilder::default()
//         .with_icon(icon)
//         .with_title("模拟拉力对战平台 - V".to_string() + std::env!("CARGO_PKG_VERSION")).with_inner_size([1000.0, 600.0]);
//     }
    
//     eframe::run_native("模拟拉力对战平台", native_options, Box::new(|cc| 
//         Box::new(app.configure_font(&cc.egui_ctx)))).unwrap();
// }

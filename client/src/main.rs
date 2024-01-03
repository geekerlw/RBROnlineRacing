#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;
use client::RacingClient;
use simplelog::WriteLogger;
use self_update::cargo_crate_version;
use log::{info, trace};

mod ui;
mod game;
mod components;
mod client;

fn auto_upgrade() -> Result<(), Box<dyn ::std::error::Error>> {
    let mut rel_builder = self_update::backends::github::ReleaseList::configure();
    rel_builder.repo_owner("geekerlw");
    rel_builder.with_target("rust-rbronline.exe");
    rel_builder.auth_token("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");

    let releases = rel_builder.repo_name("RBROnlineRacing").build()?.fetch()?;
    info!("found releases:");
    trace!("{:#?}\n", releases);

    let status = self_update::backends::github::Update::configure()
        .repo_owner("geekerlw")
        .repo_name("RBROnlineRacing")
        .auth_token("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")
        .bin_name("rust-rbronline")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    info!("Update to version: `{}`!", status.version());
    Ok(())
}

fn main() {
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

    if let Err(e) = auto_upgrade() {
        info!("Auto Upgrade failed: {:?}", e);
    }

    tokio_main();
}

#[tokio::main]
async fn tokio_main() {
    let app = RacingClient::default().init();
    let mut native_options: eframe::NativeOptions = eframe::NativeOptions::default();
    native_options.centered = true;
    if let Ok(icon) = eframe::icon_data::from_png_bytes(include_bytes!(r"..\icon.png")) {
        native_options.viewport = egui::ViewportBuilder::default()
        .with_icon(icon)
        .with_title("模拟拉力对战平台 - V".to_string() + cargo_crate_version!()).with_inner_size([1000.0, 600.0]);
    }
    
    eframe::run_native("模拟拉力对战平台", native_options, Box::new(|cc| 
        Box::new(app.configure_font(&cc.egui_ctx)))).unwrap();
}
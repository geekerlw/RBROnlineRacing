// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;

use components::store;
use game::rbr::RBRGame;
use serde_json::{json, Value};
use simplelog::WriteLogger;
use tauri::State;
use client::RacingClient;

mod components;
mod game;
mod client;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_user_name() -> String {
    "anonymous".to_string()
}

#[tauri::command]
fn load_store_config(state: State<RacingClient>, key: &str) -> String {
    let store = state.store.lock().unwrap();
    let value = store.get_config(key);
    let json = json!({
        "key": key,
        "value": value,
    });
    json.to_string()
}

#[tauri::command]
fn save_store_config(state: State<RacingClient>, keypair: &str) -> String {
    let mut store = state.store.lock().unwrap();
    let json_value: Value = serde_json::from_str(keypair).unwrap();
    let key = json_value["key"].as_str().unwrap();
    let value = json_value["value"].as_str().unwrap();
    store.set_config(key, value);
    keypair.to_string()
}

#[tauri::command]
fn save_all_store_config(state: State<RacingClient>) -> String {
    let mut store = state.store.lock().unwrap();
    store.save_config();

    let json = json!({
        "result": "ok"
    });
    json.to_string()
}

#[tauri::command]
fn load_game_user_name(state: State<RacingClient>) -> String {
    let store = state.store.lock().unwrap();
    let json = json!({
        "user": store.user_name
    });

    json.to_string()
}

#[tauri::command]
fn load_game_stage_options(state: State<RacingClient>) -> String {
    let store = state.store.lock().unwrap();
    if let Some(rect) = RBRGame::new(&store.game_path).load_game_stages() {
        let jsonstr = serde_json::to_string(&rect).unwrap();
        return jsonstr;
    }
    String::new()
}

#[tauri::command]
fn load_game_stage_wetness_options() -> String {
    let wetnesses = vec![
        json!({"id": 0, "value": "Dry"}),
        json!({"id": 1, "value": "Damp"}),
        json!({"id": 2, "value": "Wet"}),
    ];
    let jsonstr = serde_json::to_string(&wetnesses).unwrap();
    jsonstr
}

#[tauri::command]
fn load_game_stage_weather_options() -> String {
    let weathers = vec![
        json!({"id": 0, "value": "Good"}),
        json!({"id": 1, "value": "Random"}),
        json!({"id": 2, "value": "Bad"}),
    ];
    let jsonstr = serde_json::to_string(&weathers).unwrap();
    jsonstr
}

#[tauri::command]
fn load_game_stage_skytype_options(state: State<RacingClient>, stage_id: u32) -> String {
    let store = state.store.lock().unwrap();
    let mut rbr = RBRGame::new(&store.game_path);
    let mut skytypes = vec![];
    if let Some(skys) = rbr.load_game_stage_weathers(&stage_id) {
        for (i, sky) in skys.iter().enumerate() {
            skytypes.push(json!({"id": i, "value": sky.get_weather_string()}));
        }
    }

    let jsonstr = serde_json::to_string(&skytypes).unwrap();
    jsonstr
}

#[tauri::command]
fn load_game_car_options(state: State<RacingClient>) -> String {
    let store = state.store.lock().unwrap();
    if let Some(rect) = RBRGame::new(&store.game_path).load_game_cars() {
        let jsonstr = serde_json::to_string(&rect).unwrap();
        return jsonstr;
    }
    String::new()
}

#[tauri::command]
fn load_game_car_damage_options() -> String {
    let damages = vec![
        json!({"id": 0, "value": "Off"}),
        json!({"id": 1, "value": "Safe"}),
        json!({"id": 2, "value": "Reduced"}),
        json!({"id": 3, "value": "Realistic"}),
    ];
    let jsonstr = serde_json::to_string(&damages).unwrap();
    jsonstr
}

#[tauri::command]
fn load_game_car_tyre_options() -> String {
    let tyretypes = vec![
        json!({"id": 0, "value": "Dry tarmac"}),
        json!({"id": 1, "value": "Intermediate tarmac"}),
        json!({"id": 2, "value": "Wet tarmac"}),
        json!({"id": 3, "value": "Dry gravel"}),
        json!({"id": 4, "value": "Inter gravel"}),
        json!({"id": 5, "value": "Wet gravel"}),
        json!({"id": 6, "value": "Snow"}),
    ];
    let jsonstr = serde_json::to_string(&tyretypes).unwrap();
    jsonstr
}

#[tauri::command]
fn load_game_car_setup_options(state: State<RacingClient>, path: &str) -> String {
    let store = state.store.lock().unwrap();
    let mut rbr = RBRGame::new(&store.game_path);
    let mut car_setups = vec![];
    car_setups.push(json!({"id": 0, "value": "Default"}));
    if let Some(setups) = rbr.load_game_car_setups(&path.to_string()) {
        for (i, setup) in setups.iter().enumerate() {
            car_setups.push(json!({"id": i + 1, "value": setup}));
        }
    }

    let jsonstr = serde_json::to_string(&car_setups).unwrap();
    jsonstr
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

    let app = RacingClient::default().init();

    tauri::Builder::default()
        .manage(app)
        .invoke_handler(tauri::generate_handler![
            greet,
            get_user_name,
            load_store_config,
            save_store_config,
            save_all_store_config,
            load_game_user_name,
            load_game_stage_options,
            load_game_stage_wetness_options,
            load_game_stage_weather_options,
            load_game_stage_skytype_options,
            load_game_car_options,
            load_game_car_damage_options,
            load_game_car_tyre_options,
            load_game_car_setup_options,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
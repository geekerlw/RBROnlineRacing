// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use game::rbr::RBRGame;
use serde_json::json;
use simplelog::WriteLogger;

mod game;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn load_game_user_name() -> String {
    RBRGame::new().get_user().to_string()
}

#[tauri::command]
fn load_game_stage_options() -> Result<String, String> {
    if let Some(rect) = RBRGame::new().load_game_stages() {
        let jsonstr = serde_json::to_string(&rect).unwrap();
        return Ok(jsonstr);
    }
    Err("Failed to load stage options".to_string())
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
fn load_game_stage_skytype_options(stage_id: u32) -> Result<String, String> {
    let mut rbr = RBRGame::new();
    let mut skytypes = vec![];
    if let Some(skys) = rbr.load_game_stage_weathers(&stage_id) {
        for (i, sky) in skys.iter().enumerate() {
            skytypes.push(json!({"id": i, "value": sky.get_weather_string()}));
        }
        let jsonstr = serde_json::to_string(&skytypes).unwrap();
        return Ok(jsonstr);
    }
    Err("Failed to load skytype options".to_string())
}

#[tauri::command]
fn load_game_car_options() -> Result<String, String> {
    if let Some(rect) = RBRGame::new().load_game_cars() {
        let jsonstr = serde_json::to_string(&rect).unwrap();
        return Ok(jsonstr);
    }
    Err("Failed to load car options".to_string())
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
fn load_game_car_setup_options(path: &str) -> Result<String, String> {
    let mut rbr = RBRGame::new();
    let mut car_setups = vec![];
    car_setups.push(json!({"id": 0, "value": "Default"}));
    if let Some(setups) = rbr.load_game_car_setups(&path.to_string()) {
        for (i, setup) in setups.iter().enumerate() {
            car_setups.push(json!({"id": i + 1, "value": setup}));
        }
        let jsonstr = serde_json::to_string(&car_setups).unwrap();
        return Ok(jsonstr);
    }
    Err("Failed to load car setups options".to_string())
}

fn main() {
    if let Some(root_path) = std::env::current_exe().unwrap().parent() {
        println!("app is running in dir: {:?}", root_path);
        let log_file = root_path.join("RBR-BattleNet.log");
        WriteLogger::init(log::LevelFilter::Info, 
            simplelog::Config::default(), std::fs::File::create(log_file).unwrap()).unwrap();
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
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
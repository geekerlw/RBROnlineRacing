use std::io::Read;
use unicode_normalization::UnicodeNormalization;

use protocol::httpapi::{RaceState, MetaRaceData, RaceInfo, MetaRaceResult};
use ini::Ini;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone)]
pub struct RBRGame {
    pub root_path: String,
    pub test_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RBRStageData {
    pub id: String,
    pub name: String,
    pub deftime: String,
    pub length: String,
    pub surface_id: String,
    pub stage_id: String,
    pub short_country: String,
    pub author: String,
    pub tarmac: String,
    pub gravel: String,
    pub snow: String,
    pub new_update: String,
    pub author_web: String,
    pub author_note: String,
    pub fattrib: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RBRCarData {
    pub id: String,
    pub name: String,
    pub path: String,
    pub hash: String,
    pub carmodel_id: String,
    pub user_id: String,
    pub base_group_id: String,
    pub test: String,
    pub ngp: String,
    pub custom_setups: String,
    pub rev: String,
    pub audio: Option<String>,
    pub audio_hash: String,
}

impl RBRGame {
    pub fn new(path: &String) -> Self {
        Self {
            root_path: path.clone(),
            test_count: 0,
        }
    }

    pub async fn launch(&mut self) {

    }

    pub async fn load(&mut self) {

    }

    pub fn start(&mut self) {
        
    }

    pub fn get_user(&mut self) -> String {
        let default_user = "anonymous".to_string();
        let conf_path = self.root_path.clone() + r"\rallysimfans.ini";
        if let Ok(conf) = Ini::load_from_file(conf_path) {
            return conf.get_from_or(Some("login"), "name", default_user.as_str()).to_string();
        }
        default_user
    }

    pub fn get_race_state(&mut self) -> RaceState {
        let mut state = RaceState::default();
        if self.test_count <= 10 {
            state = RaceState::RaceRunning;
        } else if self.test_count > 10 {
            state = RaceState::RaceFinished;
        } else {
            state = RaceState::default();
        }

        self.test_count += 1;
        return state;
    }

    pub fn get_race_data(&mut self) -> MetaRaceData {
        MetaRaceData::default()
    }

    pub fn set_race_data(&mut self, result: &MetaRaceResult) {
        
    }

    pub fn set_race_result(&mut self, result: &MetaRaceResult) {
        
    }

    pub fn set_race_info(&mut self, info: &RaceInfo) {
        let recent_filepath = self.root_path.clone() + r"\rsfdata\cache\recent.ini";
        if let Ok(mut conf) = Ini::load_from_file(&recent_filepath) {
            if let Some(car_id) = &info.car_id {
                conf.with_section(Some("PracticeCar")).set("id", car_id.to_string());
            }
            conf.with_section(Some("PracticeStage")).set("id", info.stage_id.to_string());
            conf.write_to_file(recent_filepath).unwrap();
        }
        let conf_path = self.root_path.clone() + r"\rallysimfans.ini";
        if let Ok(mut conf) = Ini::load_from_file(&conf_path) {
            conf.with_section(Some("drive")).set("practice_damage", info.damage.to_string());
            conf.write_to_file(conf_path).unwrap();
        }
    }

    pub fn load_game_stages(&mut self) -> Option<Vec<RBRStageData>> {
        let filepath = self.root_path.clone() + r"\rsfdata\cache\stages_data.json";
        if let Ok(mut file) = std::fs::File::open(filepath) {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            let bufstr = String::from_utf8_lossy(&buf).to_string().nfc().collect::<String>();
            if let Ok(mut stages) = serde_json::from_str::<Vec<RBRStageData>>(&bufstr) {
                stages.sort_by(|a, b| a.name.cmp(&b.name));
                return Some(stages);
            }
        }
        return None;
    }

    pub fn load_game_cars(&mut self) -> Option<Vec<RBRCarData>> {
        let filepath = self.root_path.clone() + r"\rsfdata\cache\cars.json";
        if let Ok(file) = std::fs::File::open(filepath) {
            if let Ok(mut cars) = serde_json::from_reader::<std::fs::File, Vec<RBRCarData>>(file) {
                cars.sort_by(|a, b| a.name.cmp(&b.name));
                return Some(cars);
            }
        }
        return None;
    }
}
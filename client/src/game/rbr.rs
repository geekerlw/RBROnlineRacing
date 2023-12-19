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

    }

    pub fn load_game_stages(&mut self) {
        let stage_file = self.root_path.clone() + r"\rsfdata\cache\stages_data.json";
        let file = std::fs::File::open(stage_file).unwrap();
        let stages: Vec<RBRStageData> = serde_json::from_reader(file).unwrap();
    }
}
use protocol::httpapi::{RaceState, MetaRaceData, RaceInfo, MetaRaceResult};


#[derive(Debug, Default, Clone)]
pub struct RBRGame {
    pub root_path: String,
}

impl RBRGame {
    pub fn new(path: &String) -> Self {
        Self {
            root_path: path.clone(),
        }
    }

    pub async fn launch(&mut self) {

    }

    pub async fn load(&mut self) {

    }

    pub fn start(&mut self) {
        
    }

    pub fn get_user(&mut self) {

    }

    pub fn get_race_state(&mut self) -> RaceState {
        RaceState::RaceDefault
    }

    pub fn get_race_data(&mut self) -> MetaRaceData {
        MetaRaceData::default()
    }

    pub fn set_race_result(&mut self, result: &MetaRaceResult) {
        
    }

    pub fn set_race_info(&mut self, info: &RaceInfo) {

    }
}
use protocol::httpapi::{RaceState, MetaRaceData, RaceInfo, MetaRaceResult};


#[derive(Debug, Default, Clone)]
pub struct RBRGame {
    pub root_path: String,
    pub test_count: u32,
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

    pub fn get_user(&mut self) {

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
}
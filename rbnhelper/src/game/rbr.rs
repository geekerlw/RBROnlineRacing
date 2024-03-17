use rbnproto::httpapi::RaceState;
use rbnproto::metaapi::MetaRaceData;
use ini::Ini;
use super::hacker::*;

#[derive(Debug, Default)]
pub struct RBRGame;

impl RBRGame {
    pub fn get_user(&mut self) -> String {
        let default_user = "anonymous".to_string();
        if let Some(game_path) = std::env::current_exe().unwrap().parent() {
            let conf_path = game_path.join("rallysimfans.ini");
            if let Ok(conf) = Ini::load_from_file(conf_path) {
                return conf.get_from_or(Some("login"), "name", default_user.as_str()).to_string();
            }
        }
        default_user
    }

    pub fn get_race_state(&mut self) -> RaceState {
        let mut state = RaceState::RaceDefault;
        unsafe {
            let game_mode = RBR_ReadGameMode();
            let start_count = RBR_ReadRaceStartCount();
            let track_load_state = RBR_ReadTrackLoadState();
            if game_mode == 0x01 && start_count < 0f32 {
                state = RaceState::RaceRunning;
            } else if game_mode == 0x0A && track_load_state == 0x08 && start_count == 7f32 {
                state = RaceState::RaceLoaded;
            } else if game_mode == 0x0C {
                state = RaceState::RaceFinished;
            }
        };
        state
    }

    pub fn get_race_data(&mut self) -> MetaRaceData {
        let mut data = MetaRaceData::default();
        unsafe {
            data.speed = RBR_ReadCarSpeed();
            data.racetime = RBR_ReadCarRaceTime();
            data.progress = RBR_ReadCarStageProgress();
            data.stagelen = RBR_ReadStageLen();
            data.splittime1 = RBR_ReadSplitTime1();
            data.splittime2 = RBR_ReadSplitTime2();
            data.finishtime = RBR_ReadFinishTime();
        }
        data
    }
}
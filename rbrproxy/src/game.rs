use rbnproto::httpapi::RaceState;
use rbnproto::metaapi::MetaRaceData;
use ini::Ini;
use crate::RBRProxy;

#[derive(Default)]
pub struct RBRGame {
    proxy: RBRProxy,
}

impl RBRGame {
    pub fn load(&mut self) {
    }

    pub fn start(&mut self) {
    }

    pub fn game_mode(&mut self) -> i32 {
        self.proxy.read_game_mode()
    }

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
        let game_mode = self.proxy.read_game_mode();
        let start_count = self.proxy.read_race_start_count();
        let track_load_state = self.proxy.read_track_load_state();
        if game_mode == 0x01 && start_count < 0f32 {
            state = RaceState::RaceRunning;
        } else if game_mode == 0x05 {
            state = RaceState::RaceLoading;
        } else if game_mode == 0x0A && track_load_state == 0x08 && start_count == 7f32 {
            state = RaceState::RaceLoaded;
        } else if game_mode == 0x09 {
            state = RaceState::RaceFinished;
        } else if game_mode == 0x0C {
            state = RaceState::RaceExitMenu;
        }
        state
    }

    pub fn get_race_data(&mut self) -> MetaRaceData {
        let mut data = MetaRaceData::default();
        data.speed = self.proxy.read_car_speed();
        data.racetime = self.proxy.read_car_race_time();
        data.progress = self.proxy.read_car_stage_progress();
        data.stagelen = self.proxy.read_stage_len();
        data.splittime1 = self.proxy.read_split1_time();
        data.splittime2 = self.proxy.read_split2_time();
        data.finishtime = self.proxy.read_finish_time();
        data.carlook = self.proxy.read_car_look();
        data.carpos = self.proxy.read_car_pos();
        data
    }
}
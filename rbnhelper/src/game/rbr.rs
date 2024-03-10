use libc::{c_uchar, c_float, c_uint};
use protocol::httpapi::{RaceState, RaceInfo, RaceConfig};
use protocol::metaapi::{MetaRaceData, MetaRaceProgress};
use ini::Ini;
use std::mem::size_of;
use super::hacker::*;

#[derive(Default)]
#[repr(C, packed)]
pub struct RBRRaceSetting {
    pub datatype: c_uint,
    pub external: c_uint,
    pub stage: c_uint,
    pub wetness: c_uint,
    pub weather: c_uint,
    pub skytype: c_uint,
    pub car: c_uint,
    pub damage: c_uint,
    pub tyre: c_uint,
    pub setup: c_uint,
}

impl RBRRaceSetting {
    fn from(info: &RaceInfo, cfg: &RaceConfig) -> Self {
        let mut racesetting = RBRRaceSetting::default();
        racesetting.datatype = 1;
        racesetting.external = 1;
        racesetting.stage = info.stage_id;
        racesetting.wetness = info.wetness;
        racesetting.weather = info.weather;
        racesetting.skytype = info.skytype_id + 1;
        if info.car_fixed {
            racesetting.car = info.car_id;
            racesetting.setup = 0;
        } else {
            racesetting.car = cfg.car_id;
            racesetting.setup = cfg.setup_id;
        }
        racesetting.damage = info.damage;
        racesetting.tyre = cfg.tyre;
        racesetting
    }

    fn as_bytes(self) -> [u8; size_of::<RBRRaceSetting>()] {
        let mut bytes = [0; size_of::<RBRRaceSetting>()];
        unsafe {
            let ptr = &self as *const RBRRaceSetting as *const u8;
            std::ptr::copy(ptr, bytes.as_mut_ptr(), size_of::<RBRRaceSetting>());
        };
        bytes
    }
}

#[derive(Default)]
#[repr(C, packed)]
pub struct RBRRaceItem {
    pub name: [c_uchar; 32],
    pub progress: c_float,
    pub difffirst: c_float,
}

#[derive(Default)]
#[repr(C, packed)]
pub struct RBRRaceData {
    pub datatype: c_uint,
    pub external: c_uint,
    pub count: c_uint,
    pub data: [RBRRaceItem; 8],
}

impl RBRRaceData {
    fn from_result(result: &Vec<MetaRaceProgress>) -> Self {
        let mut racedata = RBRRaceData::default();
        racedata.datatype = 2;
        racedata.external = 1;
        for (index, item) in result.iter().enumerate() {
            if index >= 8 {
                break;
            }

            racedata.count += 1;
            let bytes = item.profile_name.as_bytes();
            for i in 0..bytes.len() {
                racedata.data[index].name[i] = bytes[i];
            }
            racedata.data[index].progress = item.progress.clone();
            racedata.data[index].difffirst = item.difffirst.clone();
        }
        racedata
    }

    fn as_bytes(self) -> [u8; size_of::<RBRRaceData>()] {
        let mut bytes = [0; size_of::<RBRRaceData>()];
        unsafe {
            let ptr = &self as *const RBRRaceData as *const u8;
            std::ptr::copy(ptr, bytes.as_mut_ptr(), size_of::<RBRRaceData>());
        };
        bytes
    }
}

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
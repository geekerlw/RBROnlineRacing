use libc::{c_uchar, c_float, c_uint};
use protocol::httpapi::{RaceState, RaceInfo, RaceConfig};
use protocol::metaapi::{MetaRaceData, MetaRaceProgress};
use ini::Ini;
use std::mem::size_of;

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

    // pub fn get_race_state(&mut self) -> RaceState {
    //     let handle = (self.pid as Pid).try_into_process_handle().unwrap().set_arch(Architecture::Arch32Bit);
    //     let game_mode_addr = DataMember::<i32>::new_offset(handle, vec![0x7EAC48, 0x728]);
    //     let loading_mode_addr = DataMember::<i32>::new_offset(handle, vec![0x7EA678, 0x70, 0x10]);
    //     let startcount_addr = DataMember::<f32>::new_offset(handle, vec![0x165FC68, 0x244]);
    //     let game_mode: i32 = unsafe {game_mode_addr.read().unwrap()};
    //     let loading_mode: i32 = unsafe {loading_mode_addr.read().unwrap()};
    //     let start_count: f32 = unsafe {startcount_addr.read().unwrap()};
    //     if game_mode == 0x01 && start_count < 0f32 {
    //         return RaceState::RaceRunning;
    //     } else if game_mode == 0x0A && loading_mode == 0x08 && start_count == 7f32 {
    //         return RaceState::RaceLoaded;
    //     } else if game_mode == 0x0C {
    //         return RaceState::RaceFinished;
    //     }
    //     RaceState::RaceDefault
    // }

    // pub fn get_race_data(&mut self) -> MetaRaceData {
    //     let mut data = MetaRaceData::default();
    //     let handle = (self.pid as Pid).try_into_process_handle().unwrap().set_arch(Architecture::Arch32Bit);
    //     let speed_addr = DataMember::<f32>::new_offset(handle, vec![0x165FC68, 0x0C]);
    //     let racetime_addr = DataMember::<f32>::new_offset(handle, vec![0x165FC68, 0x140]);
    //     let progress_addr = DataMember::<f32>::new_offset(handle, vec![0x165FC68, 0x13C]);
    //     let stagelen_addr = DataMember::<i32>::new_offset(handle, vec![0x1659184, 0x75310]);
    //     let split1_addr = DataMember::<f32>::new_offset(handle, vec![0x165FC68, 0x258]);
    //     let split2_addr = DataMember::<f32>::new_offset(handle, vec![0x165FC68, 0x25C]);
    //     let line_finished_addr = DataMember::<i32>::new_offset(handle, vec![0x165FC68, 0x2C4]);

    //     unsafe {
    //         data.speed = speed_addr.read().unwrap();
    //         data.racetime = racetime_addr.read().unwrap();
    //         data.progress = progress_addr.read().unwrap();
    //         data.stagelen = stagelen_addr.read().unwrap() as f32;
    //         data.splittime1 = split1_addr.read().unwrap();
    //         data.splittime2 = split2_addr.read().unwrap();
    //         if line_finished_addr.read().unwrap() == 1 {
    //             data.finishtime = racetime_addr.read().unwrap();
    //         } else {
    //             data.finishtime = 3600.0;
    //         }
    //     }

    //     data
    // }
}
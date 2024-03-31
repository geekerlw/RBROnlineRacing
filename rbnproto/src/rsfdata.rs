use libc::{c_uchar, c_float, c_uint};
use crate::httpapi::{RaceInfo, RaceConfig};
use crate::metaapi::{MetaRaceProgress, MetaRaceResult};
use serde::{Serialize, Deserialize};
use std::mem::size_of;


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

impl RBRStageData {
    pub fn get_surface(&self) -> String {
        let gravel = self.gravel.parse::<u32>().unwrap();
        let tarmac = self.tarmac.parse::<u32>().unwrap();
        let snow = self.snow.parse::<u32>().unwrap();
        if gravel >= tarmac && gravel >= snow {
            String::from("Gravel")
        } else if tarmac >= gravel && tarmac >= snow {
            String::from("Tarmac")
        } else {
            String::from("Snow")
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RBRStageWeather {
    pub stage_id: String,
    pub timeofday: String,
    pub timeofday2: String,
    pub skytype: String,
    pub skycloudtype: String,
}

impl RBRStageWeather {
    pub fn get_weather_string(&self) -> String {
        let mut fmtstr = String::new();
        match self.timeofday2.as_str() {
            "0" => fmtstr.push_str("Morning "),
            "1" => fmtstr.push_str("Noon "),
            "2" => fmtstr.push_str("Evening "),
            _ => fmtstr.push_str("Morning "),
        }
        match self.skycloudtype.as_str() {
            "0" => fmtstr.push_str("Clear "),
            "1" => fmtstr.push_str("PartCloud "),
            "2" => fmtstr.push_str("LightCloud "),
            "3" => fmtstr.push_str("HeavyCloud "),
            _ => fmtstr.push_str("Clear "),
        }
        match self.skytype.as_str() {
            "0" => fmtstr.push_str("Crisp"),
            "1" => fmtstr.push_str("Hazy"),
            "2" => fmtstr.push_str("NoRain"),
            "3" => fmtstr.push_str("LightRain"),
            "4" => fmtstr.push_str("HeavyRain"),
            "5" => fmtstr.push_str("NoSnow"),
            "6" => fmtstr.push_str("LightSnow"),
            "7" => fmtstr.push_str("HeaveSnow"),
            "8" => fmtstr.push_str("LightFog"),
            "9" => fmtstr.push_str("HeavyFog"),
            _ => fmtstr.push_str("Crisp"),
        }
        fmtstr
    }

    pub fn get_weight(&self) -> u32 {
        let timeofday2 = self.timeofday2.parse::<u32>().unwrap();
        let skycloud = self.skycloudtype.parse::<u32>().unwrap();
        let skytype = self.skytype.parse::<u32>().unwrap();

        skycloud << 8 | skytype << 4 | timeofday2
    }
}

#[derive(Default)]
#[repr(C, packed)]
pub struct RBRRaceSetting {
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
    pub fn from(info: &RaceInfo, cfg: &RaceConfig) -> Self {
        let mut racesetting = RBRRaceSetting::default();
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

    pub fn as_bytes(self) -> [u8; size_of::<RBRRaceSetting>()] {
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
    pub external: c_uint,
    pub count: c_uint,
    pub data: [RBRRaceItem; 8],
}

impl RBRRaceData {
    pub fn from_result(result: &Vec<MetaRaceProgress>) -> Self {
        let mut racedata = RBRRaceData::default();
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

    pub fn as_bytes(self) -> [u8; size_of::<RBRRaceData>()] {
        let mut bytes = [0; size_of::<RBRRaceData>()];
        unsafe {
            let ptr = &self as *const RBRRaceData as *const u8;
            std::ptr::copy(ptr, bytes.as_mut_ptr(), size_of::<RBRRaceData>());
        };
        bytes
    }
}

#[derive(Default)]
#[repr(C, packed)]
pub struct RBRRaceResultItem {
    pub name: [c_uchar; 32],
    pub racecar: [c_uchar; 32],
    pub splittime1: f32,
    pub splittime2: f32,
    pub finishtime: f32,
    pub difftime: f32,
    pub score: i32,
}

#[derive(Default)]
#[repr(C, packed)]
pub struct RBRRaceResult {
    pub external: c_uint,
    pub count: c_uint,
    pub data: [RBRRaceResultItem; 8],
}

impl RBRRaceResult {
    pub fn from_result(result: &Vec<MetaRaceResult>) -> Self {
        let mut raceresult = RBRRaceResult::default();
        raceresult.external = 1;
        for (index, item) in result.iter().enumerate() {
            if index >= 8 {
                break;
            }

            raceresult.count += 1;
            let bytes = item.profile_name.as_bytes();
            for i in 0..bytes.len() {
                raceresult.data[index].name[i] = bytes[i];
            }

            let bytes = item.racecar.as_bytes();
            for i in 0..bytes.len() {
                raceresult.data[index].racecar[i] = bytes[i];
            }

            raceresult.data[index].splittime1 = item.splittime1.clone();
            raceresult.data[index].splittime2 = item.splittime2.clone();
            raceresult.data[index].finishtime = item.finishtime.clone();
            raceresult.data[index].difftime = item.difftime.clone();
            raceresult.data[index].score = 30;
        }
        raceresult
    }

    pub fn as_bytes(self) -> [u8; size_of::<RBRRaceResult>()] {
        let mut bytes = [0; size_of::<RBRRaceResult>()];
        unsafe {
            let ptr = &self as *const RBRRaceResult as *const u8;
            std::ptr::copy(ptr, bytes.as_mut_ptr(), size_of::<RBRRaceResult>());
        };
        bytes
    }
}
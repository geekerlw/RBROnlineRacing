use libc::{c_uchar, c_float, c_uint};
use crate::httpapi::{RaceInfo, RaceConfig};
use crate::metaapi::{MetaRaceProgress, MetaRaceResult, MetaRaceState};
use crate::{D3DMatrix, D3DQuaternion};
use serde::{Serialize, Deserialize};
use std::mem::size_of;
use nalgebra::{Quaternion, UnitQuaternion};

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
pub struct RBRRaceStateItem {
    pub name: [c_uchar; 32],
    pub state: [c_uchar; 32],
}

#[derive(Default)]
#[repr(C, packed)]
pub struct RBRRaceState {
    pub external: c_uint,
    pub count: c_uint,
    pub data: [RBRRaceStateItem; 16],
}

impl RBRRaceState {
    pub fn from_result(result: &Vec<MetaRaceState>) -> Self {
        let mut racestate = RBRRaceState::default();
        racestate.external = 1;
        for (index, item) in result.iter().enumerate() {
            if index >= 16 {
                break;
            }

            racestate.count += 1;
            let bytes = item.name.as_bytes();
            for i in 0..bytes.len() {
                racestate.data[index].name[i] = bytes[i];
            }

            let statestr = format!("{:?}", item.state);
            let bytes = statestr.as_bytes();
            for i in 4..bytes.len() { // trim 'Race' in RaceState like RaceReady.
                racestate.data[index].state[i - 4] = bytes[i];
            }
        }
        racestate
    }

    pub fn as_bytes(self) -> [u8; size_of::<RBRRaceState>()] {
        let mut bytes = [0; size_of::<RBRRaceState>()];
        unsafe {
            let ptr = &self as *const RBRRaceState as *const u8;
            std::ptr::copy(ptr, bytes.as_mut_ptr(), size_of::<RBRRaceState>());
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
    pub carlook: D3DQuaternion,
    pub carpos: D3DQuaternion,
    pub ghost: D3DMatrix,
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
            racedata.data[index].carlook = item.carlook.clone();
            racedata.data[index].carpos = item.carpos.clone();
            racedata.data[index].ghost = Self::generate_ghost(&item.carlook, &item.carpos);
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

    pub fn generate_ghost(quat: &D3DQuaternion, pos: &D3DQuaternion) -> D3DMatrix {
        let quaternion = Quaternion::new(quat.m[3], quat.m[0], quat.m[2], quat.m[1]);
        let rotation = UnitQuaternion::from_quaternion(quaternion);
        let matrix = rotation.to_homogeneous();

        let mut ghost = D3DMatrix::default();
        ghost.m[0] = [matrix.m11, matrix.m12, matrix.m13, matrix.m14];
        ghost.m[1] = [matrix.m21, matrix.m22, matrix.m23, matrix.m24];
        ghost.m[2] = [matrix.m31, matrix.m32, matrix.m33, matrix.m34];
        ghost.m[3] = [pos.m[0], pos.m[2], pos.m[1], matrix.m44];
        ghost
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
            raceresult.data[index].score = item.score.clone();
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
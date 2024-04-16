use serde::{Deserialize, Serialize};
use crate::{httpapi::RaceInfo, D3DMATRIX};
use super::httpapi::RaceState;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum RaceCmd {
    #[default]
    RaceCmdDefault,
    RaceCmdPrepare(RaceInfo),
    RaceCmdLoad,
    RaceCmdStart,
    RaceCmdUpload,
    RaceCmdFinish,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum DataFormat {
    #[default]
    FmtDefault,
    FmtUserAccess = 1,
    FmtUpdateState = 2,
    FmtUploadData = 3,
    FmtRaceCommand = 4,
    FmtSyncRaceState = 5,
    FmtSyncRaceData = 6,
    FmtSyncRaceResult = 7,
    FmtSyncRaceNotice = 8,
    FmtResponse = 0x8000,
}

///
/// Race meta data socket protocols.
/// 
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceJoin {
    pub token: String,
    pub room: String,
    pub passwd: Option<String>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceAccess {
    pub token: String,
    pub room: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceLeave {
    pub token: String,
    pub room: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceUpdate {
    pub token: String,
    pub room: String,
    pub state: RaceState,
}

pub static META_HEADER_LEN: usize = 6;
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetaHeader {
    pub length: u16,
    pub format: DataFormat,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetaRaceState {
    pub name: String,
    pub state: RaceState,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetaRaceData {
    pub token: String,
    pub room: String,
    pub speed: f32,
    pub racetime: f32,
    pub progress: f32,
    pub stagelen: f32,
    pub splittime1: f32,
    pub splittime2: f32,
    pub finishtime: f32,
    pub carpos: D3DMATRIX,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetaRaceProgress {
    pub profile_name: String,
    pub progress: f32,
    pub difffirst: f32,
    pub carpos: D3DMATRIX,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetaRaceResult {
    pub profile_name: String,
    pub racecar: String,
    pub splittime1: f32,
    pub splittime2: f32,
    pub finishtime: f32,
    pub difftime: f32,
    pub score: i32,
}
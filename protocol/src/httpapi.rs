use serde::{Deserialize, Serialize};

pub static API_VERSION_STRING: &'static str = "v1.0";

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum RaceState {
    #[default]
    RaceDefault,
    RaceInit,
    RaceReady,
    RaceLoading,
    RaceLoaded,
    RaceStarting,
    RaceStarted,
    RaceRunning,
    RaceRetired,
    RaceFinished,
    RaceError(String),
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum RoomState {
    #[default]
    RoomFree,
    RoomFull,
    RoomLocked,
    RoomRaceOn,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum RaceCmd {
    #[default]
    RaceCmdDefault,
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
    FmtSyncRaceData = 5,
    FmtSyncRaceResult = 6,
    FmtResponse = 0x8000,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserLogin {
    pub name: String,
    pub passwd: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserLogout {
    pub token: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceQuery {
    pub name: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceBrief {
    pub name: String,
    pub stage: String,
    pub owner: String,
    pub state: RoomState,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceInfo {
    pub name: String,
    pub stage: String,
    pub stage_id: u32,
    pub stage_len: u32,
    pub car: Option<String>,
    pub car_id: Option<u32>,
    pub damage: u32,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceUserState {
    pub name: String,
    pub state: RaceState,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceCreate {
    pub token: String,
    pub info: RaceInfo,
    pub locked: bool,
    pub passwd: Option<String>,
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
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetaRaceResult {
    pub profile_name: String,
    pub racetime: f32,
    pub progress: f32,
    pub splittime1: f32,
    pub splittime2: f32,
    pub finishtime: f32,
    pub difffirst: f32,
    pub difftime: f32,
}
use serde::{Deserialize, Serialize};

pub static API_VERSION_STRING: &'static str = "v1.0";

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum RaceState {
    #[default]
    RaceDefault,
    RaceInit,
    RaceReady,
    RaceLoad,
    RaceLoaded,
    RaceStart,
    RaceRunning,
    RaceRetired,
    RaceFinished,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum RoomState {
    #[default]
    RoomDefault,
    RoomFree,
    RoomLocked,
    RoomRaceOn,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum DataFormat {
    #[default]
    FmtDefault,
    FmtUserAccess = 1,
    FmtUpdateState = 2,
    FmtUploadData = 3,
    FmtRaceCommand = 4,
    FmtPushData = 5,
    FmtResponse = 0x8000,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserLogin {
    pub name: String,
    pub passwd: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserAccess {
    pub token: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceItem {
    pub name: String,
    pub stage: String,
    pub owner: String,
    pub state: RoomState,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceList {
    pub room: Vec<RaceItem>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceInfo {
    pub token: String,
    pub name: String,
    pub stage: String,
    pub car: Option<String>,
    pub damage: Option<u32>,
    pub setup: Option<String>,
    pub state: RoomState,
    pub players: Vec<String>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserJoin {
    pub token: String,
    pub room: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserUpdate {
    pub token: String,
    pub state: RaceState,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetaHeader {
    pub length: u16,
    pub format: DataFormat,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetaRaceData {
    pub token: String,
    pub profile_name: String,
    pub starttime: f32,
    pub racetime: f32,
    pub process: f32,
    pub splittime1: f32,
    pub splittime2: f32,
    pub finishtime: f32,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MetaRaceResult {
    pub state: RaceState,
    pub board: Vec<MetaRaceData>,
}
use serde::{Deserialize, Serialize};

pub static API_VERSION_STRING: &'static str = "v1.0";

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum RacePlayerState {
    #[default]
    RaceDefault,
    RaceRunning(u32),
    RaceFinished(u32),
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UserLogin {
    pub name: String,
    pub passwd: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UserAccess {
    pub token: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RaceItem {
    pub name: String,
    pub stage: String,
    pub owner: String,
    pub state: RacePlayerState,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RaceList {
    pub room: Vec<RaceItem>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RaceInfo {
    pub token: String,
    pub name: String,
    pub stage: String,
    pub car: Option<String>,
    pub damage: Option<u32>,
    pub setup: Option<String>,
    pub state: RacePlayerState,
    pub players: Vec<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UserJoin {
    pub token: String,
    pub room: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct UserUpdate {
    pub token: String,
    pub state: RacePlayerState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaHeader {
    pub length: u16,
    pub format: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaRaceData {
    pub token: String,
    pub process: f32,
    pub racetime: f32,
}
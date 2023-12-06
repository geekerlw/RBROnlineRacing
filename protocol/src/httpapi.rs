use serde::{Deserialize, Serialize};

pub static API_VERSION_STRING: &'static str = "v1.0";

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
    pub state: u32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RaceList {
    pub room: Vec<RaceItem>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RaceInfo {
    pub name: String,
    pub stage: String,
    pub car: Option<String>,
    pub damage: Option<u32>,
    pub setup: Option<String>,
    pub state: u32,
    pub players: Vec<String>,
}
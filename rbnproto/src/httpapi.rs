use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
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
    RaceExitMenu,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum RoomState {
    #[default]
    RoomFree,
    RoomFull,
    RoomRaceOn,
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
pub type UserQuery = UserLogout;
pub type UserHeart = UserLogout;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceQuery {
    pub name: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UserScore {
    pub name: String,
    pub license: String,
    pub score: i32,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceBrief {
    pub name: String,
    pub stage: String,
    pub owner: String,
    pub players: u32,
    pub state: RoomState,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceInfo {
    pub name: String,
    pub owner: String,
    pub stage: String,
    pub stage_id: u32,
    pub stage_type: String,
    pub stage_len: u32,
    pub car_fixed: bool,
    pub car: String,
    pub car_id: u32,
    pub damage: u32,
    pub weather: u32,
    pub wetness: u32,
    pub skytype: String,
    pub skytype_id: u32,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceConfig {
    pub car: String,
    pub car_id: u32,
    pub tyre: u32,
    pub setup: String,
    pub setup_id: u32,
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

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceInfoUpdate {
    pub token: String,
    pub info: RaceInfo,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RaceConfigUpdate {
    pub token: String,
    pub cfg: RaceConfig,
}
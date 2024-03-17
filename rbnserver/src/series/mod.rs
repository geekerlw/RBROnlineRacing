use std::sync::Arc;

use rbnproto::{httpapi::{RaceBrief, RaceConfig, RaceInfo, RaceState, RaceUserState}, metaapi::{MetaRaceData, RaceJoin}};
use serde::{Deserialize, Serialize};
use tokio::{net::tcp::OwnedWriteHalf, sync::Mutex};

use crate::player::LobbyPlayer;

pub mod customize;
pub mod daily;
pub mod room;
pub mod randomer;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub enum RoomRaceState {
    #[default]
    RoomRaceInit,
    RoomRaceBegin,
    RoomRaceReady,
    RoomRaceLoading,
    RoomRaceLoaded,
    RoomRaceStarting,
    RoomRaceStarted,
    RoomRaceRunning,
    RoomRaceFinished,
    RoomRaceEnd,
}

pub trait Series {
    fn join(&mut self, user: &LobbyPlayer);

    fn leave(&mut self, token: &String);

    fn access(&mut self, token: &String, writer: Arc<Mutex<OwnedWriteHalf>>) -> bool;

    fn is_joinable(&mut self, join: &RaceJoin) -> bool;

    fn need_recycle(&mut self) -> bool;
    
    fn is_started(&mut self) -> bool;

    fn set_start(&mut self) -> bool;

    fn get_race_brief(&mut self) -> RaceBrief;

    fn get_race_config(&mut self) -> RaceInfo;

    fn update_race_config(&mut self, info: RaceInfo);

    fn get_player_config(&mut self, token: &String) -> Option<RaceConfig>;

    fn update_player_config(&mut self, token: &String, cfg: RaceConfig) -> bool;

    fn get_players_state(&mut self) -> Vec<RaceUserState>;

    fn update_player_state(&mut self, token: &String, state: RaceState) -> bool;

    fn update_player_data(&mut self, token: &String, data: MetaRaceData) -> bool;

    fn framed_schedule(&mut self);
}
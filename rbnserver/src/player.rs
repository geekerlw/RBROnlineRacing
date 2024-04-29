use std::sync::Arc;
use chrono::{DateTime, Local};
use rbnproto::httpapi::{RaceConfig, RaceState};
use rbnproto::metaapi::{DataFormat, MetaHeader, MetaRaceData, MetaRaceProgress, MetaRaceResult, MetaRaceState, RaceCmd};
use serde::{Serialize, Deserialize};
use tokio::{sync::Mutex, net::tcp::OwnedWriteHalf, io::AsyncWriteExt};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct LobbyPlayer {
    pub tokenstr: String,
    pub profile_name: String,

    #[serde(skip)]
    lastactive: DateTime<Local>,
}

impl LobbyPlayer {
    pub fn new(token: &String, name: &String) -> Self {
        Self { 
            tokenstr: token.clone(),
            profile_name: name.clone(),
            lastactive: Local::now()
        }
    }

    pub fn set_alive(&mut self) {
        self.lastactive = Local::now();
    }

    pub fn is_alive(&mut self) -> bool {
        Local::now().signed_duration_since(self.lastactive) < chrono::Duration::seconds(60)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RacePlayer {
    pub token: Uuid,
    pub tokenstr: String,
    pub profile_name: String,
    #[serde(skip)]
    pub writer: Option<Arc<Mutex<OwnedWriteHalf>>>,
    pub state: RaceState,
    pub race_data: MetaRaceData,
    pub race_cfg: RaceConfig,
}

impl RacePlayer {
    pub fn new(tokenstr: &String, username: &String) -> Self {
        Self {
            token: Uuid::parse_str(&tokenstr.as_str()).unwrap(),
            tokenstr: tokenstr.clone(),
            profile_name: username.clone(),
            writer: None,
            state: RaceState::default(),
            race_data: MetaRaceData::default(),
            race_cfg: RaceConfig::default(),
        }
    }

    pub fn sort_by_progress(&self, player: &RacePlayer) -> std::cmp::Ordering {
        if self.race_data.progress < player.race_data.progress {
            return std::cmp::Ordering::Greater;
        } else if self.race_data.progress == player.race_data.progress {
            return std::cmp::Ordering::Equal;
        } else {
            return std::cmp::Ordering::Less;
        }
    }

    pub fn sort_by_time(&self, player: &RacePlayer) -> std::cmp::Ordering {
        if self.race_data.finishtime > player.race_data.finishtime {
            return std::cmp::Ordering::Greater;
        } else if self.race_data.finishtime == player.race_data.finishtime {
            return std::cmp::Ordering::Equal;
        } else {
            return std::cmp::Ordering::Less;
        }
    }

    pub async fn notify_user_cmd(&self, cmd: &RaceCmd) {
        let body = bincode::serialize(cmd).unwrap();
        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtRaceCommand}).unwrap();
        if let Some(writer) = &self.writer {
            writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap_or(());
        }
    }

    pub async fn notify_racestate(&self, result: &Vec::<MetaRaceState>) {
        let body = bincode::serialize(result).unwrap();
        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtSyncRaceState}).unwrap();
        if let Some(writer) = &self.writer {
            writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap_or(());
        }
    }

    pub async fn notify_racedata(&self, result: &Vec::<MetaRaceProgress>) {
        let body = bincode::serialize(result).unwrap();
        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtSyncRaceData}).unwrap();
        if let Some(writer) = &self.writer {
            writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap_or(());
        }
    }

    pub async fn notify_result(&self, result: &Vec::<MetaRaceResult>) {
        let body = bincode::serialize(result).unwrap();
        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtSyncRaceResult}).unwrap();
        if let Some(writer) = &self.writer {
            writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap_or(());
        }
    }

    pub async fn notify_racenotice(&self, result: &String) {
        let body = bincode::serialize(result).unwrap();
        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtSyncRaceNotice}).unwrap();
        if let Some(writer) = &self.writer {
            writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap_or(());
        }
    }
}
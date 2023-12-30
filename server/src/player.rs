use std::sync::Arc;
use protocol::httpapi::{RaceState, MetaRaceData, RaceCmd, MetaHeader, DataFormat, MetaRaceResult};
use serde::{Serialize, Deserialize};
use tokio::{sync::Mutex, net::tcp::OwnedWriteHalf, io::AsyncWriteExt};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct LobbyPlayer {
    pub tokenstr: String,
    pub profile_name: String,
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
            writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
        }
    }

    pub async fn notify_racedata(&self, result: &Vec::<MetaRaceResult>) {
        let body = bincode::serialize(result).unwrap();
        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtSyncRaceData}).unwrap();
        if let Some(writer) = &self.writer {
            writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
        }
    }

    pub async fn notify_result(&self, result: &Vec::<MetaRaceResult>) {
        let body = bincode::serialize(result).unwrap();
        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtSyncRaceResult}).unwrap();
        if let Some(writer) = &self.writer {
            writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
        }
    }
}
use log::info;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{Sender, Receiver};
use std::sync::Arc;
use rbnproto::httpapi::{RaceConfig, RaceInfo, RaceQuery, RaceState, UserLogin};
use rbnproto::metaapi::{DataFormat, MetaHeader, MetaRaceProgress, MetaRaceResult, RaceAccess, RaceCmd, RaceJoin, RaceLeave, RaceUpdate, META_HEADER_LEN};
use rbnproto::rsfdata::RBRRaceSetting;
use rbnproto::API_VERSION_STRING;
use reqwest::StatusCode;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use crate::game::rbr::RBRGame;


pub enum TaskMsg {
    MsgStartGame,
}

#[derive(Default, Clone)]
pub struct RBNBackend {
    tx: Option<Sender<TaskMsg>>,
}

impl RBNBackend {
    pub fn init(&mut self) {

    }

    pub fn run(&mut self, tx: Sender<TaskMsg>, mut rx: Receiver<TaskMsg>) {
        self.tx = Some(tx.clone());
        std::thread::spawn(move || {
            Runtime::new().unwrap().block_on(async move {
                loop {
                    if let Some(task) = rx.recv().await {
                        match task {
                            TaskMsg::MsgStartGame => {
                                test_func().await;
                            },
                        }
                    }
                }
            });
        });
    }

    pub fn triger(&mut self, task: TaskMsg) {
        if let Some(tx) = &self.tx {
            tx.blocking_send(task).unwrap();
        }
    }
}

async fn test_func() {
    tokio::spawn(async {
        info!("this is an task to run.");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        info!("after 2 secs, task over.");
    });
}

// maybe start task after joined race.
fn start_race(race: &String) {
    let meta_addr = self.store.get_meta_url();
    let user_token = self.store.user_token.clone();
    let room_name = race.clone();

    tokio::spawn(async move {
        let stream = TcpStream::connect(meta_addr).await.unwrap();
        let (mut reader, mut writer) = stream.into_split();

        let access = RaceAccess {token: user_token.clone(), room: room_name.clone()};
        let body = bincode::serialize(&access).unwrap();
        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUserAccess}).unwrap();
        writer.write_all(&[&head[..], &body[..]].concat()).await.unwrap_or(());

        let update = RaceUpdate {token: user_token.clone(), room: room_name.clone(), state: RaceState::RaceReady};
        let body = bincode::serialize(&update).unwrap();
        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
        writer.write_all(&[&head[..], &body[..]].concat()).await.unwrap_or(());

        let mut recvbuf = vec![0u8; 1024];
        let mut remain = Vec::<u8>::new();
        let writer_clone = Arc::new(Mutex::new(writer));
        while let Ok(n) = reader.read(&mut recvbuf).await {
            if n == 0 {
                break;
            }

            // 处理接收的数据
            // 这里只是简单地将接收到的数据打印出来
            // trace!("Received data: {:?}", &recvbuf[..n]);

            let buffer = [&remain[..], &recvbuf[..n]].concat();
            let datalen = buffer.len();
            let mut offset = 0 as usize;

            while offset + META_HEADER_LEN <= datalen {
                if datalen < META_HEADER_LEN {
                    break;
                }
                let head: MetaHeader = bincode::deserialize(&buffer[offset..offset+META_HEADER_LEN]).unwrap();

                if (offset + META_HEADER_LEN + head.length as usize) > datalen {
                    break;
                }     
                let pack_data = &buffer[offset+META_HEADER_LEN..offset+META_HEADER_LEN+head.length as usize];

                meta_message_handle(head.clone(), pack_data, &user_token, &room_name, writer_clone.clone(), tx.clone()).await;
                offset += META_HEADER_LEN + head.length as usize;
            }
            remain = (&buffer[offset..]).to_vec();
        }
    });
}

async fn meta_message_handle(head: MetaHeader, pack_data: &[u8], token: &String, room: &String, writer: Arc<Mutex<OwnedWriteHalf>>, tx: Sender<InnerMsg>) {
    match head.format {
        DataFormat::FmtRaceCommand => {
            let cmd: RaceCmd = bincode::deserialize(pack_data).unwrap();
            match cmd {
                RaceCmd::RaceCmdLoad => {
                    info!("recv cmd to load game");
                    tokio::spawn(start_game_load(token.clone(), room.clone(), writer.clone()));
                }
                RaceCmd::RaceCmdStart => {
                    info!("recv cmd to start game");
                    tokio::spawn(start_game_race(token.clone(), room.clone(), writer.clone()));
                }
                RaceCmd::RaceCmdUpload => {
                    info!("recv cmd to upload race data");
                    tokio::spawn(start_game_upload(token.clone(), room.clone(), writer.clone()));
                }
                _ => {}
            }
        }

        DataFormat::FmtSyncRaceData => {
            let progress: Vec<MetaRaceProgress> = bincode::deserialize(pack_data).unwrap();
        }

        DataFormat::FmtSyncRaceResult => {
            let result: Vec<MetaRaceResult> = bincode::deserialize(pack_data).unwrap();
        }
        _ => {}
    }
}


// need to start this task when stage loaded.
async fn start_game_load(token: String, room: String, writer: Arc<Mutex<OwnedWriteHalf>>) {
    let mut rbr = RBRGame::default();
    let user_token = token.clone();
    let room_name = room.clone();
    tokio::spawn(async move {
        loop {
            let state = rbr.get_race_state();
            match state {
                RaceState::RaceLoaded | RaceState::RaceRunning => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    let update = RaceUpdate {token: user_token.clone(), room: room_name.clone(), state: RaceState::RaceLoaded};
                    let body = bincode::serialize(&update).unwrap();
                    let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
                    writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap_or(());
                    break;
                },
                _ => {},
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }
    });
}

async fn start_game_race(token: String, room: String, writer: Arc<Mutex<OwnedWriteHalf>>) {
    let user_token = token.clone();
    let room_name = room.clone();
    tokio::spawn(async move {
        let update = RaceUpdate {token: user_token.clone(), room: room_name, state: RaceState::RaceStarted};
        let body = bincode::serialize(&update).unwrap();
        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
        writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap_or(());
    });
}

async fn start_game_upload(token: String, room: String, writer: Arc<Mutex<OwnedWriteHalf>>) {
    let mut rbr = RBRGame::default();
    let user_token = token.clone();
    let room_name = room.clone();
    tokio::spawn(async move {
        loop {
            let state = rbr.get_race_state();
            match state {
                RaceState::RaceRetired | RaceState::RaceFinished => {
                    let update = RaceUpdate {token: user_token.clone(), room: room_name.clone(), state: state.clone()};
                    let body = bincode::serialize(&update).unwrap();
                    let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
                    writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap_or(());
                    break;
                },
                RaceState::RaceRunning => {
                    let mut data = rbr.get_race_data();
                    data.token = user_token.clone();
                    data.room = room_name.clone();
                    let body = bincode::serialize(&data).unwrap();
                    let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUploadData}).unwrap();
                    writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap_or(());
                },
                _ => {},
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }
    });
}
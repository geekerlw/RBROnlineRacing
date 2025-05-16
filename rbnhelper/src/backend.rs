use log::info;
use tokio::runtime::Builder;
use tokio::sync::mpsc::{Sender, Receiver};
use tokio::task::JoinHandle;
use tokio::time::Instant;
use std::sync::Arc;
use std::time::Duration;
use rbnproto::httpapi::{RaceInfo, RaceState};
use rbnproto::metaapi::{DataFormat, MetaHeader, MetaRaceProgress, MetaRaceResult, MetaRaceRidicule, MetaRaceState, RaceAccess, RaceCmd, RaceUpdate, META_HEADER_LEN};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::sync::{Mutex, OnceCell};

use crate::components::input::is_horn_pressed;
use crate::components::player::AudioPlayer;
use crate::components::store::RacingStore;
use rbrproxy::game::RBRGame;
use crate::rbnhelper::InnerMsg;


pub enum TaskMsg {
    MsgStartStage(String),
    MsgStopStage,
}

#[derive(Default, Clone)]
pub struct RBNBackend {
    meta_addr: String,
    user_token: String,
    tx: Option<Sender<TaskMsg>>,
}

impl RBNBackend {
    pub fn init(&mut self, store: &RacingStore) {
        self.meta_addr = store.get_meta_url();
        self.user_token = store.user_token.clone();
    }

    pub fn run(&mut self, tx: Sender<TaskMsg>, mut rx: Receiver<TaskMsg>, notifier: &Sender<InnerMsg>) {
        self.tx = Some(tx.clone());
        let server = self.meta_addr.clone();
        let token = self.user_token.clone();
        let notifier = notifier.clone();
        std::thread::spawn(move || {
            Builder::new_multi_thread().enable_all().build().unwrap().block_on(async move {
                let mut stage_task = None;
                loop {
                    if let Some(task) = rx.recv().await {
                        match task {
                            TaskMsg::MsgStartStage(room) => {
                                stage_task = Some(spawn_one_stage(&server, &token, &room, &notifier));
                            },
                            TaskMsg::MsgStopStage => {
                                if let Some(mission) = &stage_task {
                                    mission.abort();
                                    stage_task = None;
                                }
                            }
                        }
                    }
                }
            });
        });
    }

    pub fn trigger(&mut self, task: TaskMsg) {
        if let Some(tx) = &self.tx {
            tx.blocking_send(task).unwrap();
        }
    }
}

fn spawn_one_stage(server: &String, token: &String, race: &String, notifier: &Sender<InnerMsg>) -> JoinHandle<()> {
    let meta_addr = server.clone();
    let user_token = token.clone();
    let room_name = race.clone();
    let notifier = notifier.clone();

    tokio::spawn(async move {
        let stream = TcpStream::connect(meta_addr).await.unwrap();
        let (mut reader, mut writer) = stream.into_split();

        let access = RaceAccess {token: user_token.clone(), room: room_name.clone()};
        let body = bincode::serialize(&access).unwrap();
        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUserAccess}).unwrap();
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

                meta_message_handle(head.clone(), pack_data, &user_token, &room_name, &writer_clone, &notifier).await;
                offset += META_HEADER_LEN + head.length as usize;
            }
            remain = (&buffer[offset..]).to_vec();
        }
    })
}

async fn meta_message_handle(head: MetaHeader, pack_data: &[u8], token: &String, room: &String, writer: &Arc<Mutex<OwnedWriteHalf>>, notifier: &Sender<InnerMsg>) {
    match head.format {
        DataFormat::FmtRaceCommand => {
            let cmd: RaceCmd = bincode::deserialize(pack_data).unwrap();
            match cmd {
                RaceCmd::RaceCmdPrepare(info) => {
                    info!("recv cmd to prepare game: {:?}", info);
                    tokio::spawn(start_game_prepare(token.clone(), room.clone(), writer.clone(), info, notifier.clone()));
                }
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
                RaceCmd::RaceCmdHorn => {
                    info!("recv cmd to play horn");
                    AudioPlayer::horn().set_timeout(2).play();
                }
                _ => {}
            }
        }

        DataFormat::FmtSyncRaceState => {
            let state: Vec<MetaRaceState> = bincode::deserialize(pack_data).unwrap();
            //TODO: RBRGame::default().feed_race_state(&state);
        }

        DataFormat::FmtSyncRaceData => {
            let progress: Vec<MetaRaceProgress> = bincode::deserialize(pack_data).unwrap();
            //TODO: RBRGame::default().feed_race_data(&progress);
        }

        DataFormat::FmtSyncRaceRidicule => {
            let ridicule: MetaRaceRidicule = bincode::deserialize(pack_data).unwrap();
            info!("recv cmd to ridicule: {:?}", ridicule);
            tokio::spawn(start_game_ridicule(ridicule));
        }

        DataFormat::FmtSyncRaceResult => {
            let result: Vec<MetaRaceResult> = bincode::deserialize(pack_data).unwrap();
            //TODO: RBRGame::default().feed_race_result(&result);
        }

        DataFormat::FmtSyncRaceNotice => {
            let notice: String = bincode::deserialize(pack_data).unwrap();
            notifier.send(InnerMsg::MsgUpdateNotice(notice)).await.unwrap();
        }
        _ => {}
    }
}

async fn start_game_prepare(token: String, room: String, writer: Arc<Mutex<OwnedWriteHalf>>, info: RaceInfo, notifier: Sender<InnerMsg>) {
    let mut rbr = RBRGame::default();
    let user_token = token.clone();
    let room_name = room.clone();
    let notifier = notifier.clone();
    //TODO: rbr.config(&info);
    tokio::spawn(async move {
        AudioPlayer::notification("prepare.wav").set_timeout(5).play();
        tokio::time::sleep_until(Instant::now() + Duration::from_secs(1)).await;
        let start_time = std::time::SystemTime::now();
        loop {
            if std::time::SystemTime::now().duration_since(start_time).unwrap() > std::time::Duration::from_secs(30) {
                break;
            }

            let state = rbr.get_race_state();
            match state {
                RaceState::RaceLoading => break,
                _ => {
                    let remain = std::time::Duration::from_secs(30) - std::time::SystemTime::now().duration_since(start_time).unwrap();
                    notifier.send(InnerMsg::MsgUpdateNotice(format!("Check Car tyre and setup, Stage will auto start after {}.{} seconds.", remain.as_secs(), remain.subsec_millis()))).await.unwrap();
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }

        let update = RaceUpdate {token: user_token.clone(), room: room_name.clone(), state: RaceState::RaceReady};
        let body = bincode::serialize(&update).unwrap();
        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
        writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap_or(());
    });
}

// need to start this task when stage loaded.
async fn start_game_load(token: String, room: String, writer: Arc<Mutex<OwnedWriteHalf>>) {
    let mut rbr = RBRGame::default();
    let user_token = token.clone();
    let room_name = room.clone();
    rbr.load();
    tokio::spawn(async move {
        AudioPlayer::notification("load_race.wav").set_timeout(5).play();
        loop {
            let state = rbr.get_race_state();
            match state {
                RaceState::RaceLoaded | RaceState::RaceRunning => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
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
    AudioPlayer::notification("begin_race.wav").play();
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    RBRGame::default().start();
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
        let once_finished: OnceCell<()> = OnceCell::new();
        loop {
            let state = rbr.get_race_state();
            match state {
                RaceState::RaceExitMenu => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await; // give some time to execute exit to menu state.
                    let update = RaceUpdate {token: user_token.clone(), room: room_name.clone(), state: state.clone()};
                    let body = bincode::serialize(&update).unwrap();
                    let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
                    writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap_or(());
                    break;
                },
                RaceState::RaceRetired | RaceState::RaceFinished => {
                    once_finished.get_or_init(|| {
                        async {
                            let update = RaceUpdate {token: user_token.clone(), room: room_name.clone(), state: state.clone()};
                            let body = bincode::serialize(&update).unwrap();
                            let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
                            writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap_or(());
                        }
                    }).await;
                },
                RaceState::RaceRunning => {
                    let horn = is_horn_pressed();
                    if horn {
                        AudioPlayer::horn().set_timeout(2).play();
                    }
                    let mut data = rbr.get_race_data();
                    data.token = user_token.clone();
                    data.room = room_name.clone();
                    data.horn = horn;
                    let body = bincode::serialize(&data).unwrap();
                    let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUploadData}).unwrap();
                    writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap_or(());
                },
                _ => {},
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
    });
}

async fn start_game_ridicule(ridicule: MetaRaceRidicule) {
    for profile in ridicule.players {
        AudioPlayer::ridicule(&profile, "overtake").set_timeout(5).play();
    }
}
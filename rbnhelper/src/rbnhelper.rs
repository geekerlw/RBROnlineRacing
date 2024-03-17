use std::sync::Arc;

use libc::c_char;
use log::info;
use rbnproto::httpapi::{RaceBrief, RaceConfig, RaceInfo, RaceQuery, RaceState, UserLogin, UserQuery};
use rbnproto::metaapi::{DataFormat, MetaHeader, MetaRaceProgress, MetaRaceResult, RaceAccess, RaceCmd, RaceJoin, RaceUpdate, META_HEADER_LEN};
use rbnproto::API_VERSION_STRING;
use reqwest::StatusCode;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use crate::game::plugin::IPlugin;
use crate::game::hacker::*;
use crate::game::rbr::RBRGame;
use ini::Ini;
use tokio::runtime::Handle;
use crate::components::store::RacingStore;
use tokio::sync::mpsc::{channel, Receiver, Sender};

enum InnerMsg {
    MsgUserLogined(String),
    MsgMenuMessage(String),
    MsgInGameMessage(String),
    MsgErrorMessage(String),
    MsgSetRaceInfo(RaceInfo),
}

pub struct RBNHelper {
    tokio: Handle,
    rx: Receiver<InnerMsg>,
    tx: Sender<InnerMsg>,
    store: RacingStore,
    menu_message: String,
    ingame_message: String,
    error_message: String,
    copyright: String,
}

impl Default for RBNHelper {
    fn default() -> Self {
        let (tx, rx) = channel::<InnerMsg>(64);
        Self {
            tokio: tokio::runtime::Builder::new_multi_thread().enable_all().build().expect("Failed to init tokio runtime.").handle().clone(),
            rx,
            tx,
            store: RacingStore::default(),
            menu_message: String::new(),
            ingame_message: String::new(),
            error_message: String::new(),
            copyright: format!("Welcome to use RBN Helper [{}], Copyright Lw_Ziye 2023-2024.", std::env!("CARGO_PKG_VERSION")),
        }
    }
}

impl IPlugin for RBNHelper {
    fn GetName(&self) -> *const libc::c_char {
        let name = std::ffi::CString::new("RBN Helper").unwrap();
        name.into_raw()
    }
}

impl RBNHelper {
    pub fn init(&mut self) {
        self.store.init();
        self.load_dashboard_config();
        self.init_async_runtime();
        self.check_and_login();
    }

    pub fn is_logined(&self) -> bool {
        !self.store.user_state.is_empty()
    }

    fn init_async_runtime(&mut self) {
        let runtime = self.tokio.clone();
        std::thread::spawn(move || {
            runtime.block_on(async {
                info!("started tokio runtime success.");
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                };
            });
        });
    }

    fn check_and_login(&mut self) {
        let url_ver = self.store.get_http_url("api/version");
        let url_login = self.store.get_http_url("api/user/login");
        let user = UserLogin{name: self.store.user_name.clone(), passwd: self.store.user_passwd.clone()};
        let tx = self.tx.clone();
        self.tokio.spawn(async move {
            let res = reqwest::get(url_ver).await.unwrap();
            if res.status() == StatusCode::OK {
                let version = res.text().await.unwrap();
                if version != API_VERSION_STRING {
                    tx.send(InnerMsg::MsgErrorMessage("Version is out of date, please upgrade!".to_string())).await.unwrap();
                } else {
                    let res = reqwest::Client::new().post(url_login).json(&user).send().await.unwrap();
                    if res.status() == StatusCode::OK {
                        let token = res.text().await.unwrap();
                        tx.send(InnerMsg::MsgUserLogined(token)).await.unwrap();
                    }
                }
            }
        });
    }

    fn load_dashboard_config(&mut self) {
        if let Some(game_path) = std::env::current_exe().unwrap().parent() {
            let conf_path = game_path.join("Plugins").join("RBNHelper").join("RBRDashBoard.ini");
            if let Ok(conf) = Ini::load_from_file(conf_path) {
                let enable_leader = conf.get_from_or(Some("Setting"), "LeaderEnable", "false");
                match enable_leader {
                    "true" => {
                        unsafe {
                            RBR_EnableLeaderBoard();
                            RBR_CfgLeaderBoardPos(
                            conf.get_from_or(Some("Pos"), "LeaderBoardPosX", "20").parse().unwrap(),
                            conf.get_from_or(Some("Pos"), "LeaderBoardPosY", "100").parse().unwrap()
                            );
                            RBR_CfgLeaderBoardStyle(
                                conf.get_from_or(Some("Color"), "LeaderBriefColor", "0xFFFF00FF").as_ptr() as *const c_char,
                                conf.get_from_or(Some("Color"), "LeaderBackGroundColor", "0xFFFFFF1F").as_ptr() as *const c_char,
                            );
                        };
                    },
                    _ => {},
                };
                let enable_progress = conf.get_from_or(Some("Setting"), "ProgressEnable", "false");
                match enable_progress {
                    "true" => {
                        unsafe {
                            RBR_EnableProgressBar();
                            RBR_CfgProgressBarPos(
                            conf.get_from_or(Some("Pos"), "ProgressBarPosX", "40").parse().unwrap(),
                            conf.get_from_or(Some("Pos"), "ProgressBarPosY", "300").parse().unwrap()
                            );
                            RBR_CfgProgressBarStyle(
                                conf.get_from_or(Some("Color"), "ProgressBarBackColor", "0xFFFFFFFF").as_ptr() as *const c_char,
                                conf.get_from_or(Some("Color"), "ProgressBarSplitColor", "0x00FF00FF").as_ptr() as *const c_char,
                                conf.get_from_or(Some("Color"), "ProgressBarPointerColor", "0x00FF00FF").as_ptr() as *const c_char,
                            );
                        };
                    }
                    _ => {},
                }

                if enable_leader.eq("true") || enable_progress.eq("true") {
                    unsafe {
                        RBR_CfgProfileStyle(
                            conf.get_from_or(Some("Color"), "UserColor1", "0xFF0000FF").as_ptr() as *const c_char,
                            conf.get_from_or(Some("Color"), "UserColor2", "0x00FF00FF").as_ptr() as *const c_char,
                        );
                    }
                }
            }
        }
    }

    pub fn async_message_handle(&mut self) {
        if let Ok(msg) = self.rx.try_recv() {
            match msg {
                InnerMsg::MsgUserLogined(token) => {
                    self.store.user_token = token;
                }
                InnerMsg::MsgMenuMessage(msg) => {
                    self.error_message = msg;
                }
                InnerMsg::MsgInGameMessage(msg) => {
                    self.ingame_message = msg;
                }
                InnerMsg::MsgErrorMessage(msg) => {
                    self.error_message = msg;
                }
                InnerMsg::MsgSetRaceInfo(raceinfo) => {
                    //TODO: set race stage info.
                }
            }
        }
    }

    pub fn draw_on_end_frame(&mut self) {
        self.async_message_handle();
        //unsafe {RBR_ShowText(50.0, 200.0, self.copyright.as_ptr() as *const c_char)};
    }

    // need to be an timed task when we in game menu.
    pub fn fetch_race_list(&mut self) {
        if self.is_logined() {
            let url = self.store.get_http_url("api/race/brief");
            let tx = self.tx.clone();
            tokio::spawn(async move {
                let res = reqwest::Client::new().get(&url).send().await.unwrap();
                if res.status() == StatusCode::OK {
                    let brief = res.text().await.unwrap();
                    tx.send(InnerMsg::MsgMenuMessage(brief)).await.unwrap();
                }
            });
        }
    }

    // need to call by hooking hotlap and practice menu in.
    pub fn join_race(&mut self, race: &String) {
        if self.is_logined() {
            let race_join = RaceJoin {token: self.store.user_token.clone(), room: race.clone(), passwd: None};
            let url = self.store.get_http_url("api/race/join");
            let tx = self.tx.clone();
            tokio::spawn(async move {
                let res = reqwest::Client::new().post(url).json(&race_join).send().await.unwrap();
                match res.status() {
                    StatusCode::OK => {
                        tx.send(InnerMsg::MsgMenuMessage("Joined Race".to_string())).await.unwrap();
                    }
                    _ => {
                        tx.send(InnerMsg::MsgErrorMessage("Failed Join Race".to_string())).await.unwrap();
                    }
                }
            });
        }
    }

    // need to call by hooking hotlap and practice menu in.
    pub fn fetch_race_info(&mut self, race: &String) {
        if self.is_logined() {
            let info_url = self.store.get_http_url("api/race/info");
            let info_tx = self.tx.clone();
            let info_query = RaceQuery {name: race.clone()};
            tokio::spawn(async move {
                let res = reqwest::Client::new().get(&info_url).json(&info_query).send().await.unwrap();
                if res.status() == StatusCode::OK {
                    let text = res.text().await.unwrap();
                    let raceinfo: RaceInfo = serde_json::from_str(text.as_str()).unwrap();
                    info_tx.send(InnerMsg::MsgSetRaceInfo(raceinfo)).await.unwrap();
                }
            });
        }
    }

    // maybe start task after joined race.
    fn start_race(&mut self, race: &String) {
        if !self.is_logined() {
            return;
        }

        let meta_addr = self.store.get_meta_url();
        let user_token = self.store.user_token.clone();
        let tx = self.tx.clone();
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
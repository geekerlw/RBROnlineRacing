use eframe::egui;
use egui::Grid;
use egui::RichText;
use protocol::httpapi::MetaHeader;
use protocol::httpapi::RaceCmd;
use protocol::httpapi::RaceAccess;
use protocol::httpapi::RaceQuery;
use protocol::httpapi::RaceState;
use protocol::httpapi::DataFormat;
use protocol::httpapi::RaceUpdate;
use protocol::httpapi::META_HEADER_LEN;
use protocol::httpapi::RaceUserState;
use reqwest::StatusCode;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::mpsc::{Sender, Receiver};
use tokio::task::JoinHandle;
use crate::game::rbr::RBRGame;
use crate::ui::UiPageState;
use crate::components::time::format_seconds;
use super::{UiView, UiPageCtx};
use protocol::httpapi::MetaRaceResult;
use std::sync::Arc;
use tokio::sync::Mutex;

enum UiRacingMsg {
    MsgRaceState(RaceState),
    MsgRaceUserState(Vec<RaceUserState>),
    MsgRaceResult(Vec<MetaRaceResult>),
    MsgRaceAllReady,
}

pub struct UiRacing {
    pub state: RaceState,
    pub userstates: Vec<RaceUserState>,
    tx: Sender<UiRacingMsg>,
    rx: Receiver<UiRacingMsg>,
    pub table_head: Vec<&'static str>,
    pub table_data: Vec<MetaRaceResult>,
    pub rbr_task: Option<JoinHandle<()>>,
    pub timed_task: Option<JoinHandle<()>>,
}

impl Default for UiRacing {
    fn default() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel::<UiRacingMsg>(16);
        Self {
            state: RaceState::RaceReady,
            userstates: vec![],
            tx,
            rx,
            table_head: vec!["排名", "车手", "分段1", "分段2", "完成时间", "头佬差距"],
            table_data: vec![],
            rbr_task: None,
            timed_task: None,
        }
    }
}

impl UiView for UiRacing {
    fn enter(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        self.state = RaceState::RaceReady;
        let meta_addr = page.store.get_meta_url();
        let user_token = page.store.user_token.clone();
        let tx = self.tx.clone();
        let game_path = page.store.game_path.clone();
        let room_name = page.store.curr_room.clone();

        let state_url = page.store.get_http_url("api/race/state");
        let state_tx = self.tx.clone();
        let state_query = RaceQuery {name: room_name.clone()};
        let task = tokio::spawn(async move {
            loop {
                let res = reqwest::Client::new().get(&state_url).json(&state_query).send().await.unwrap();
                if res.status() == StatusCode::OK {
                    let text = res.text().await.unwrap();
                    let userstate: Vec<RaceUserState> = serde_json::from_str(text.as_str()).unwrap();
                    state_tx.send(UiRacingMsg::MsgRaceUserState(userstate)).await.unwrap();
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        });
        self.timed_task = Some(task);

        let task = tokio::spawn(async move {
            let stream = TcpStream::connect(meta_addr).await.unwrap();
            let (mut reader, mut writer) = stream.into_split();
            let mut rbr = RBRGame::new(&game_path).open_udp().await;

            let access = RaceAccess {token: user_token.clone(), room: room_name.clone()};
            let body = bincode::serialize(&access).unwrap();
            let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUserAccess}).unwrap();
            writer.write_all(&[&head[..], &body[..]].concat()).await.unwrap();

            let update = RaceUpdate {token: user_token.clone(), room: room_name.clone(), state: RaceState::RaceReady};
            let body = bincode::serialize(&update).unwrap();
            let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
            writer.write_all(&[&head[..], &body[..]].concat()).await.unwrap();

            let mut recvbuf = vec![0u8; 1024];
            let mut remain = Vec::<u8>::new();
            let writer_clone = Arc::new(Mutex::new(writer));
            while let Ok(n) = reader.read(&mut recvbuf).await {
                if n == 0 {
                    break;
                }

                // 处理接收的数据
                // 这里只是简单地将接收到的数据打印出来
                // println!("Received data: {:?}", &recvbuf[..n]);

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

                    meta_message_handle(head.clone(), pack_data, &mut rbr, &user_token, &room_name, writer_clone.clone(), tx.clone()).await;
                    offset += META_HEADER_LEN + head.length as usize;
                }
                remain = (&buffer[offset..]).to_vec();
            }
        });
        self.rbr_task = Some(task);
    }

    fn exit(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, _page: &mut UiPageCtx) {
        if let Some(task) = &self.rbr_task {
            task.abort();
            self.rbr_task = None;
        }
        if let Some(task) = &self.timed_task {
            task.abort();
            self.timed_task = None;
        }
        self.state = RaceState::RaceReady;
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        if let Ok(msg) = self.rx.try_recv() {
            match msg {
                UiRacingMsg::MsgRaceState(state) => self.state = state,
                UiRacingMsg::MsgRaceUserState(state) => self.userstates = state,
                UiRacingMsg::MsgRaceResult(result) => self.table_data = result,
                UiRacingMsg::MsgRaceAllReady => {
                    if let Some(task) = &self.timed_task {
                        task.abort();
                    }
                },
            };
        }

        match self.state {
            RaceState::RaceReady => self.show_waiting(ctx, frame, page),
            RaceState::RaceLoading => self.show_loading(ctx, frame, page),
            RaceState::RaceRunning => self.show_racing(ctx, frame, page),
            RaceState::RaceFinished | RaceState::RaceRetired => self.show_result(ctx, frame, page),
            _ => {},
        }
    }
}

impl UiRacing {
    fn show_waiting(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, _page: &mut UiPageCtx) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(360.0);
                ui.label(RichText::new("等待玩家就绪...").size(32.0));
            });
            ui.add_space(60.0);
            ui.horizontal(|ui| {
                ui.add_space(400.0);
                ui.vertical(|ui| {
                    Grid::new("race players state table")
                    .min_col_width(80.0)
                    .min_row_height(24.0)
                    .show(ui, |ui| {
                        for player in self.userstates.iter() {
                            ui.label(&player.name);
                            match &player.state {
                                RaceState::RaceReady => ui.label("已就绪"),
                                RaceState::RaceLoaded => ui.label("加载完成"),
                                RaceState::RaceFinished | RaceState::RaceRetired => ui.label("已完成"),
                                _ => ui.label("未就绪"),
                            };
                            ui.end_row();
                        }
                    });
                });
            });
        });
    }

    fn show_loading(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, _page: &mut UiPageCtx) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("游戏加载中...").size(40.0));
            });
        });
    }

    fn show_racing(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, _page: &mut UiPageCtx) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("比赛进行中...").size(40.0));
            });
        });
    }

    fn show_result(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(120.0);
                ui.vertical(|ui| {
                    Grid::new("race result").min_col_width(120.0).show(ui, |ui| {
                        for content in &self.table_head {
                            ui.label(*content);
                        }
                        ui.end_row();

                        for (index, result) in self.table_data.iter().enumerate() {
                            let table = vec![(index+1).to_string(),
                                result.profile_name.clone(),
                                format_seconds(result.splittime1),
                                format_seconds(result.splittime2),
                                format_seconds(result.finishtime),
                                format_seconds(result.difffirst),
                            ];
                            for content in table {
                                ui.label(content);
                            }
                            ui.end_row();
                        }
                    });

                    ui.add_space(40.0);
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(250.0);
                        if ui.button("确认").clicked() {
                            self.reset_user_state(page);
                            page.route.switch_to_page(UiPageState::PageInRoom);
                        }
                    });
                })
            });
        });
    }

    fn reset_user_state(&mut self, page: &mut UiPageCtx) {
        let state_url = page.store.get_http_url("api/race/state");
        let state = RaceUpdate {token: page.store.user_token.clone(), room: page.store.curr_room.clone(), state: RaceState::RaceInit};
        tokio::spawn(async move {
            let _ = reqwest::Client::new().put(&state_url).json(&state).send().await.unwrap();
        });
    }
}


async fn meta_message_handle(head: MetaHeader, pack_data: &[u8], rbr: &mut RBRGame, token: &String, room: &String, writer: Arc<Mutex<OwnedWriteHalf>>, tx: Sender<UiRacingMsg>) {
    match head.format {
        DataFormat::FmtRaceCommand => {
            let cmd: RaceCmd = bincode::deserialize(pack_data).unwrap();
            match cmd {
                RaceCmd::RaceCmdLoad => {
                    println!("recv cmd to load game");
                    tokio::spawn(start_game_load(rbr.root_path.clone(), token.clone(), room.clone(), writer.clone()));
                    tx.send(UiRacingMsg::MsgRaceState(RaceState::RaceLoading)).await.unwrap();
                    tx.send(UiRacingMsg::MsgRaceAllReady).await.unwrap();
                }
                RaceCmd::RaceCmdStart => {
                    println!("recv cmd to start game");
                    tokio::spawn(start_game_race(rbr.root_path.clone(), token.clone(), room.clone(), writer.clone()));
                    tx.send(UiRacingMsg::MsgRaceState(RaceState::RaceStarting)).await.unwrap();
                }
                RaceCmd::RaceCmdUpload => {
                    println!("recv cmd to upload race data");
                    tokio::spawn(start_game_upload(rbr.root_path.clone(), token.clone(), room.clone(), writer.clone()));
                    tx.send(UiRacingMsg::MsgRaceState(RaceState::RaceRunning)).await.unwrap();
                }
                _ => {}
            }
        }

        DataFormat::FmtSyncRaceData => {
            let result: Vec<MetaRaceResult> = bincode::deserialize(pack_data).unwrap();
            rbr.set_race_data(&result).await;
        }

        DataFormat::FmtSyncRaceResult => {
            let result: Vec<MetaRaceResult> = bincode::deserialize(pack_data).unwrap();
            rbr.set_race_result(&result).await;
            tx.send(UiRacingMsg::MsgRaceResult(result)).await.unwrap();
            tx.send(UiRacingMsg::MsgRaceState(RaceState::RaceFinished)).await.unwrap();
        }
        _ => {}
    }
}

async fn start_game_load(gamepath: String, token: String, room: String, writer: Arc<Mutex<OwnedWriteHalf>>) {
    let mut rbr: RBRGame = RBRGame::new(&gamepath);
    let user_token = token.clone();
    let room_name = room.clone();
    tokio::spawn(async move {
        rbr.launch().await;
        rbr.enter_practice();

        loop {
            let state = rbr.get_race_state();
            match state {
                RaceState::RaceLoaded | RaceState::RaceRunning => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    let update = RaceUpdate {token: user_token.clone(), room: room_name.clone(), state: RaceState::RaceLoaded};
                    let body = bincode::serialize(&update).unwrap();
                    let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
                    writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
                    break;
                },
                _ => {},
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }
    });
}

async fn start_game_race(gamepath: String, token: String, room: String, writer: Arc<Mutex<OwnedWriteHalf>>) {
    let mut rbr = RBRGame::new(&gamepath);
    let user_token = token.clone();
    let room_name = room.clone();
    tokio::spawn(async move {
        rbr.start();
        let update = RaceUpdate {token: user_token.clone(), room: room_name, state: RaceState::RaceStarted};
        let body = bincode::serialize(&update).unwrap();
        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
        writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
    });
}

async fn start_game_upload(gamepath: String, token: String, room: String, writer: Arc<Mutex<OwnedWriteHalf>>) {
    let mut rbr: RBRGame = RBRGame::new(&gamepath);
    let user_token = token.clone();
    let room_name = room.clone();
    tokio::spawn(async move {
        rbr.attach();

        loop {
            let state = rbr.get_race_state();
            match state {
                RaceState::RaceRetired | RaceState::RaceFinished => {
                    let update = RaceUpdate {token: user_token.clone(), room: room_name.clone(), state: state.clone()};
                    let body = bincode::serialize(&update).unwrap();
                    let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
                    writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
                    break;
                },
                RaceState::RaceRunning => {
                    let mut data = rbr.get_race_data();
                    data.token = user_token.clone();
                    data.room = room_name.clone();
                    let body = bincode::serialize(&data).unwrap();
                    let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUploadData}).unwrap();
                    writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
                },
                _ => {},
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }
    });
}
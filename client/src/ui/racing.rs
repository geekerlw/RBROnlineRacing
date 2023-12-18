use eframe::egui;
use egui::Grid;
use egui::RichText;
use protocol::httpapi::MetaHeader;
use protocol::httpapi::RaceState;
use protocol::httpapi::UserAccess;
use protocol::httpapi::DataFormat;
use protocol::httpapi::UserUpdate;
use protocol::httpapi::META_HEADER_LEN;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::{Sender, Receiver};
use crate::game::rbr::RBRGame;
use crate::ui::UiPageState;
use super::{UiView, UiPageCtx};
use protocol::httpapi::{MetaRaceResult, MetaRaceData};
use std::sync::Arc;
use tokio::sync::Mutex;

enum UiRacingMsg {
    MsgRaceResult(MetaRaceResult),
}

pub struct UiRacing {
    pub state: RaceState,
    tx: Sender<UiRacingMsg>,
    rx: Receiver<UiRacingMsg>,
    pub table_head: Vec<&'static str>,
    pub table_data: MetaRaceResult,
}

impl Default for UiRacing {
    fn default() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel::<UiRacingMsg>(16);
        Self {
            state: RaceState::RaceInit,
            tx,
            rx,
            table_head: vec!["排名", "车手", "分段1", "分段2", "完成时间"],
            table_data: MetaRaceResult {
                state: protocol::httpapi::RaceState::RaceFinished,
                board: vec![MetaRaceData {
                    token: String::from("token"),
                    profile_name: String::from("Ziye"),
                    starttime: 0.0,
                    racetime: 120.0,
                    process: 100.0,
                    splittime1: 30.0,
                    splittime2: 80.0,
                    finishtime: 120.0,
                },
                MetaRaceData {
                    token: String::from("token"),
                    profile_name: String::from("somechen"),
                    starttime: 0.0,
                    racetime: 120.0,
                    process: 100.0,
                    splittime1: 33.0,
                    splittime2: 90.0,
                    finishtime: 140.0,
                }]
            }
        }
    }
}

impl UiView for UiRacing {
    fn enter(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        let meta_addr = page.store.get_meta_url();
        let user_token = page.store.user_token.clone();
        let tx = self.tx.clone();
        let game_path = page.store.game_path.clone();

        tokio::spawn(async move {
            let socket = TcpStream::connect(meta_addr).await.unwrap();
            let stream = Arc::new(Mutex::new(socket));
            let mut rbr = RBRGame::new(&game_path);

            let user = UserAccess {token: user_token.clone()};
            let body = bincode::serialize(&user).unwrap();
            let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUserAccess}).unwrap();
            stream.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();

            let update = UserUpdate {token: user_token.clone(), state: RaceState::RaceReady};
            let body = bincode::serialize(&update).unwrap();
            let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
            stream.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();

            let mut recvbuf = vec![0u8; 1024];
            let mut remain = Vec::<u8>::new();
            while let Ok(n) = stream.lock().await.read(&mut recvbuf).await {
                if n == 0 {
                    break;
                }

                // 处理接收的数据
                // 这里只是简单地将接收到的数据打印出来
                println!("Received data: {:?}", &recvbuf[..n]);

                let buffer = [&remain[..], &recvbuf[..]].concat();
                let datalen = buffer.len();

                if datalen <= META_HEADER_LEN {
                    remain = buffer.to_vec();
                    continue;
                }

                let head: MetaHeader = bincode::deserialize(&buffer[..META_HEADER_LEN]).unwrap();
                if datalen <= head.length as usize + META_HEADER_LEN {
                    remain = buffer.to_vec();
                    continue;
                }

                let pack_data = &buffer[META_HEADER_LEN..META_HEADER_LEN+head.length as usize];
                match head.format {
                    DataFormat::FmtRaceCommand => {
                        let state: UserUpdate = bincode::deserialize(pack_data).unwrap();
                        match state.state {
                            RaceState::RaceLoad => {
                                tokio::spawn(start_game_load(game_path.clone(), user_token.clone(), stream.clone()));
                            }
                            RaceState::RaceStart => {
                                tokio::spawn(start_game_race(game_path.clone(), user_token.clone(), stream.clone()));
                            }
                            _ => continue
                        }
                    }

                    DataFormat::FmtPushData => {
                        let result: MetaRaceResult = bincode::deserialize(pack_data).unwrap();
                        rbr.set_race_result(&result);
                        tx.send(UiRacingMsg::MsgRaceResult(result)).await.unwrap();
                    }
                    _ => {
                        break; //data type error, auto close.
                    }
                }

                remain = (&buffer[META_HEADER_LEN + head.length as usize..]).to_vec();
            }
        });
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        if let Ok(msg) = self.rx.try_recv() {
            match msg {
                UiRacingMsg::MsgRaceResult(result) => self.table_data = result,
            };
        }

        match self.state {
            RaceState::RaceRunning => self.show_racing(ctx, frame, page),
            RaceState::RaceFinished | RaceState::RaceRetired => self.show_result(ctx, frame, page),
            _ => self.show_loading(ctx, frame, page),
        }
    }
}

impl UiRacing {
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

                        for (index, result) in self.table_data.board.iter().enumerate() {
                            let table = vec![index.to_string(),
                                result.profile_name.clone(),
                                result.splittime1.to_string(),
                                result.splittime2.to_string(),
                                result.finishtime.to_string(),
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
                            page.route.switch_to_page(UiPageState::PageInRoom);
                        }
                    });
                })
            });
        });
    }
}


async fn start_game_load(gamepath: String, token: String, stream: Arc<Mutex<TcpStream>>) {
    let mut rbr: RBRGame = RBRGame::new(&gamepath);
    let user_token = token.clone();
    tokio::spawn(async move {
        rbr.launch().await;
        rbr.load().await;

        let update = UserUpdate {token: user_token.clone(), state: RaceState::RaceLoaded};
        let body = bincode::serialize(&update).unwrap();
        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
        stream.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
    });
}

async fn start_game_race(gamepath: String, token: String, stream: Arc<Mutex<TcpStream>>) {
    let mut rbr = RBRGame::new(&gamepath);
    let user_token = token.clone();
    tokio::spawn(async move {
        rbr.start();
        let update = UserUpdate {token: user_token.clone(), state: RaceState::RaceRunning};
        let body = bincode::serialize(&update).unwrap();
        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
        stream.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();

        loop {
            let state = rbr.get_race_state();
            match state {
                RaceState::RaceRetired | RaceState::RaceFinished => {
                    let update = UserUpdate {token: user_token.clone(), state: state.clone()};
                    let body = bincode::serialize(&update).unwrap();
                    let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
                    stream.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
                    break;
                },
                RaceState::RaceRunning => {
                    let data = rbr.get_race_data();
                    let body = bincode::serialize(&data).unwrap();
                    let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUploadData}).unwrap();
                    stream.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
                },
                _ => {},
            }
        }
    });
}
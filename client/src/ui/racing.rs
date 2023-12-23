use eframe::egui;
use egui::Grid;
use egui::RichText;
use protocol::httpapi::MetaHeader;
use protocol::httpapi::RaceState;
use protocol::httpapi::DataFormat;
use protocol::httpapi::UserUpdate;
use protocol::httpapi::META_HEADER_LEN;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::mpsc::{Sender, Receiver};
use crate::game::rbr::RBRGame;
use crate::ui::UiPageState;
use crate::components::time::format_duration;
use super::{UiView, UiPageCtx};
use protocol::httpapi::{MetaRaceResult, MetaRaceData};
use std::sync::Arc;
use tokio::sync::Mutex;

enum UiRacingMsg {
    MsgRaceState(RaceState),
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
            state: RaceState::RaceReady,
            tx,
            rx,
            table_head: vec!["排名", "车手", "分段1", "分段2", "完成时间"],
            table_data: MetaRaceResult {
                state: protocol::httpapi::RaceState::RaceFinished,
                board: vec![]
            }
        }
    }
}

impl UiView for UiRacing {
    fn enter(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        let meta_addr = page.store.get_meta_url();
        let user_token = page.store.user_token.clone();
        let user_profile = page.store.user_name.clone();
        let tx = self.tx.clone();
        let game_path = page.store.game_path.clone();

        tokio::spawn(async move {
            let stream = TcpStream::connect(meta_addr).await.unwrap();
            let (mut reader, mut writer) = stream.into_split();
            let mut rbr = RBRGame::new(&game_path);

            let update = UserUpdate {token: user_token.clone(), state: RaceState::RaceReady};
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
                    if datalen < head.length as usize + META_HEADER_LEN {
                        break;
                    }
        
                    let pack_data = &buffer[offset+META_HEADER_LEN..offset+META_HEADER_LEN+head.length as usize];
                    meta_message_handle(head.clone(), pack_data, &mut rbr, &user_token, &user_profile, writer_clone.clone(), tx.clone()).await;
                    offset += META_HEADER_LEN + head.length as usize;
                }
                remain = (&buffer[offset..]).to_vec();
            }
        });
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        if let Ok(msg) = self.rx.try_recv() {
            match msg {
                UiRacingMsg::MsgRaceState(state) => self.state = state,
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
                                format_duration(result.splittime1),
                                format_duration(result.splittime2),
                                format_duration(result.finishtime),
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


async fn meta_message_handle(head: MetaHeader, pack_data: &[u8], rbr: &mut RBRGame, token: &String, profile: &String, writer: Arc<Mutex<OwnedWriteHalf>>, tx: Sender<UiRacingMsg>) {
    match head.format {
        DataFormat::FmtRaceCommand => {
            let state: UserUpdate = bincode::deserialize(pack_data).unwrap();
            match state.state {
                RaceState::RaceLoad => {
                    tokio::spawn(start_game_load(rbr.root_path.clone(), token.clone(), profile.clone(), writer.clone()));
                    tx.send(UiRacingMsg::MsgRaceState(RaceState::RaceLoad)).await.unwrap();
                }
                RaceState::RaceStart => {
                    tokio::spawn(start_game_race(rbr.root_path.clone(), token.clone(), profile.clone(), writer.clone()));
                    tx.send(UiRacingMsg::MsgRaceState(RaceState::RaceRunning)).await.unwrap();
                }
                _ => {}
            }
        }

        DataFormat::FmtSyncRaceData => {
            let result: MetaRaceResult = bincode::deserialize(pack_data).unwrap();
            rbr.set_race_result(&result);
        }

        DataFormat::FmtSyncRaceResult => {
            let result: MetaRaceResult = bincode::deserialize(pack_data).unwrap();
            rbr.set_race_result(&result);
            tx.send(UiRacingMsg::MsgRaceResult(result)).await.unwrap();
            tx.send(UiRacingMsg::MsgRaceState(RaceState::RaceFinished)).await.unwrap();
        }
        _ => {}
    }
}

async fn start_game_load(gamepath: String, token: String, profile: String, writer: Arc<Mutex<OwnedWriteHalf>>) {
    let mut rbr: RBRGame = RBRGame::new(&gamepath);
    let user_token = token.clone();
    tokio::spawn(async move {
        rbr.launch().await;
        rbr.load();
        let mut loaded = false;

        loop {
            let state = rbr.get_race_state();
            match state {
                RaceState::RaceLoaded => {
                    if !loaded {
                        loaded = true;
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                        let update = UserUpdate {token: user_token.clone(), state: RaceState::RaceLoaded};
                        let body = bincode::serialize(&update).unwrap();
                        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
                        writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
                    }
                },
                RaceState::RaceRetired | RaceState::RaceFinished => {
                    let update = UserUpdate {token: user_token.clone(), state: state.clone()};
                    let body = bincode::serialize(&update).unwrap();
                    let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
                    writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
                    break;
                },
                RaceState::RaceRunning => {
                    let mut data = rbr.get_race_data();
                    data.token = user_token.clone();
                    data.profile_name = profile.clone();
                    let body = bincode::serialize(&data).unwrap();
                    let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUploadData}).unwrap();
                    writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
                },
                _ => {},
            }
            tokio::time::sleep(tokio::time::Duration::from_micros(500)).await;
        }
    });
}

async fn start_game_race(gamepath: String, token: String, _profile: String, writer: Arc<Mutex<OwnedWriteHalf>>) {
    let mut rbr = RBRGame::new(&gamepath);
    let user_token = token.clone();
    tokio::spawn(async move {
        rbr.start();
        let update = UserUpdate {token: user_token.clone(), state: RaceState::RaceRunning};
        let body = bincode::serialize(&update).unwrap();
        let head = bincode::serialize(&MetaHeader{length: body.len() as u16, format: DataFormat::FmtUpdateState}).unwrap();
        writer.lock().await.write_all(&[&head[..], &body[..]].concat()).await.unwrap();
    });
}
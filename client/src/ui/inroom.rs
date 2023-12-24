use eframe::egui;
use egui::Grid;
use protocol::httpapi::{RaceQuery, RaceInfo, RaceLeave, RaceUserState, RaceState};
use reqwest::StatusCode;
use crate::{ui::UiPageState, game::rbr::RBRGame};
use super::{UiView, UiPageCtx};
use tokio::{sync::mpsc::{channel, Receiver, Sender}, task::JoinHandle};

enum UiInRoomMsg {
   MsgInRoomRaceInfo(RaceInfo),
   MsgInRoomUserState(Vec<RaceUserState>),
}

pub struct UiInRoom {
    pub room_name: String,
    pub raceinfo: RaceInfo,
    pub userstates: Vec<RaceUserState>,
    pub damages: Vec<&'static str>,
    rx: Receiver<UiInRoomMsg>,
    tx: Sender<UiInRoomMsg>,
    pub timed_task: Option<JoinHandle<()>>,
}

impl Default for UiInRoom {
    fn default() -> Self {
        let (tx, rx) = channel::<UiInRoomMsg>(8);
        Self { 
            room_name: "No Room Info".to_string(),
            raceinfo: RaceInfo::default(),
            userstates: vec![],
            damages: vec!["Off", "Safe", "Reduced", "Realistic"],
            rx,
            tx,
            timed_task: None,
        }
    }
}

impl UiView for UiInRoom {
    fn enter(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        self.room_name = page.store.curr_room.clone();
        let info_url = page.store.get_http_url("api/race/info");
        let info_tx = self.tx.clone();
        let info_query = RaceQuery {name: self.room_name.clone()};
        tokio::spawn(async move {
            let res = reqwest::Client::new().get(info_url).json(&info_query).send().await.unwrap();
            if res.status() == StatusCode::OK {
                let text = res.text().await.unwrap();
                let raceinfo: RaceInfo = serde_json::from_str(text.as_str()).unwrap();
                info_tx.send(UiInRoomMsg::MsgInRoomRaceInfo(raceinfo)).await.unwrap();
            }
        });

        let state_url = page.store.get_http_url("api/race/state");
        let state_tx = self.tx.clone();
        let state_query = RaceQuery {name: self.room_name.clone()};
        let task = tokio::spawn(async move {
            loop {
                let res = reqwest::Client::new().get(&state_url).json(&state_query).send().await.unwrap();
                if res.status() == StatusCode::OK {
                    let text = res.text().await.unwrap();
                    let userstate: Vec<RaceUserState> = serde_json::from_str(text.as_str()).unwrap();
                    state_tx.send(UiInRoomMsg::MsgInRoomUserState(userstate)).await.unwrap();
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        });
        self.timed_task = Some(task);
    }

    fn exit(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, _page: &mut UiPageCtx) {
        if let Some(task) = &self.timed_task {
            task.abort();
            self.timed_task = None;
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        if let Ok(msg) = self.rx.try_recv() {
            match msg {
                UiInRoomMsg::MsgInRoomRaceInfo(info) => {
                    self.raceinfo = info;
                }
                UiInRoomMsg::MsgInRoomUserState(states) => {
                    self.userstates = states;
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(120.0);
                ui.vertical(|ui| {
                    Grid::new("race room table")
                    .min_col_width(80.0)
                    .min_row_height(24.0)
                    .show(ui, |ui| {
                        ui.label("比赛房间：");
                        ui.label(&self.room_name);
                        ui.end_row();

                        ui.label("比赛赛道：");
                        ui.label(self.raceinfo.stage.clone());
                        ui.end_row();

                        ui.label("比赛车辆: ");
                        if let Some(car) = &self.raceinfo.car {
                            ui.label(car);
                        } else {
                            ui.label("不限");
                        }
                        ui.end_row();

                        ui.label("车辆损坏：");
                        ui.label(self.damages[self.raceinfo.damage as usize]);
                        ui.end_row();
                    });

                    ui.add_space(20.0);

                    Grid::new("race players table")
                    .min_col_width(80.0)
                    .min_row_height(24.0)
                    .show(ui, |ui| {
                        ui.label("序号");
                        ui.label("车手");
                        ui.end_row();
                        for (index, player) in self.userstates.iter().enumerate() {
                            ui.label((index+1).to_string());
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

                    ui.add_space(20.0);
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(80.0);
                        if ui.button("退出").clicked() {
                            self.leave_raceroom(page);
                            page.route.switch_to_page(UiPageState::PageLobby);
                        }
                        if ui.button("准备").clicked() {
                            self.start_game_racing(page);
                            page.route.switch_to_page(UiPageState::PageRacing);
                        }
                    });
                });
            });
        });
    }
}

impl UiInRoom {
    fn leave_raceroom(&mut self, page: &mut UiPageCtx) {
        if !page.store.user_token.is_empty() && !page.store.curr_room.is_empty() {
            let user: RaceLeave = RaceLeave{token: page.store.user_token.clone(), room: page.store.curr_room.clone()};
            let url = page.store.get_http_url("api/race/leave");
            tokio::spawn(async move {
                let _res = reqwest::Client::new().post(url).json(&user).send().await.unwrap();
            });
        }

        page.store.curr_room.clear();
        self.room_name.clear();
    }

    fn start_game_racing(&mut self, page: &mut UiPageCtx) {
        let mut rbr = RBRGame::new(&page.store.game_path);
        rbr.set_race_info(&self.raceinfo);
    }
}
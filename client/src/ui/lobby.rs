use eframe::egui;
use egui::Grid;
use protocol::httpapi::{RaceBrief, RoomState, RaceJoin};
use crate::ui::UiPageState;
use super::{UiView, UiPageCtx, UiMsg};
use reqwest::StatusCode;
use tokio::{sync::mpsc::{channel, Receiver, Sender}, task::JoinHandle};

pub struct UiLobby {
    pub table_head: Vec<&'static str>,
    pub table_data: Vec<RaceBrief>,
    rx: Receiver<Vec<RaceBrief>>,
    tx: Sender<Vec<RaceBrief>>,
    pub timed_task: Option<JoinHandle<()>>,
}

impl Default for UiLobby {
    fn default() -> Self {
        let (tx, rx) = channel::<Vec<RaceBrief>>(8);
        Self {
            table_head: vec!["序号", "房名", "赛道", "房主", "状态"],
            table_data: vec![],
            rx,
            tx,
            timed_task: None,
        }
    }
}

impl UiLobby {
    pub fn join_raceroom(&mut self, room: &String, page: &mut UiPageCtx) {
        let race_join = RaceJoin {token: page.store.user_token.clone(), room: room.clone(), passwd: None};
        let url = page.store.get_http_url("api/race/join");
        let tx = page.tx.clone();
        page.store.curr_room = room.clone();
        tokio::spawn(async move {
            let res = reqwest::Client::new().post(url).json(&race_join).send().await.unwrap();
            if res.status() == StatusCode::OK {
                tx.send(UiMsg::MsgGotoPage(UiPageState::PageInRoom)).await.unwrap();
            }
        });
    }
}

impl UiView for UiLobby {
    fn enter(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        let url = page.store.get_http_url("api/race/list");
        let tx = self.tx.clone();
        let task = tokio::spawn(async move {
            loop {
                let res = reqwest::Client::new().get(&url).send().await.unwrap();
                match res.status() {
                    StatusCode::OK => {
                        let text = res.text().await.unwrap();
                        let racelist: Vec<RaceBrief> = serde_json::from_str(text.as_str()).unwrap();
                        tx.send(racelist).await.unwrap();
                    },
                    StatusCode::NO_CONTENT => {
                        tx.send(vec![]).await.unwrap();
                    }
                    _ => {},
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
            self.table_data = msg;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(120.0);
                ui.vertical_centered(|ui| {
                    Grid::new("race rooms").min_col_width(120.0).show(ui, |ui| {
                        for content in &self.table_head {
                            ui.label(*content);
                        }
                        ui.end_row();

                        let table_data = self.table_data.clone();
                        for (index, race) in table_data.iter().enumerate() {
                            let table = vec![index.to_string(),
                                race.name.clone(),
                                race.stage.clone(),
                                race.owner.clone(),
                                match race.state {
                                    RoomState::RoomFree => String::from("空闲"),
                                    RoomState::RoomFull => String::from("满员"),
                                    RoomState::RoomLocked => String::from("禁止加入"),
                                    RoomState::RoomRaceOn => String::from("比赛中"),
                                }
                            ];
                            for content in table {
                                ui.label(content);
                            }
                            if &race.name != &page.store.curr_room {
                                if ui.button("加入").clicked() {
                                    self.join_raceroom(&race.name, page);
                                }
                            } else {
                                if ui.button("进入").clicked() {
                                    page.route.switch_to_page(UiPageState::PageInRoom);
                                }
                            }
                            ui.end_row();
                        }
                    });
                });
                ui.add_space(120.0);
            });
        });
    }
}
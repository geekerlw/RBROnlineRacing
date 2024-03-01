use eframe::egui;
use egui::Grid;
use protocol::httpapi::{RaceBrief, RoomState};
use protocol::metaapi::RaceJoin;
use crate::ui::UiPageState;
use super::{UiView, UiPageCtx, UiMsg};
use reqwest::StatusCode;
use tokio::{sync::mpsc::{channel, Receiver, Sender}, task::JoinHandle};

pub struct UiLobby {
    pub table_head: Vec<&'static str>,
    pub table_data: Vec<RaceBrief>,
    pub show_passwin: bool,
    pub select_room: String,
    pub select_room_pass: String,
    rx: Receiver<Vec<RaceBrief>>,
    tx: Sender<Vec<RaceBrief>>,
    pub timed_task: Option<JoinHandle<()>>,
}

impl Default for UiLobby {
    fn default() -> Self {
        let (tx, rx) = channel::<Vec<RaceBrief>>(8);
        Self {
            table_head: vec!["序号", "房名", "赛道", "房主", "人数", "状态"],
            table_data: vec![],
            show_passwin: false,
            select_room: String::new(),
            select_room_pass: String::new(),
            rx,
            tx,
            timed_task: None,
        }
    }
}

impl UiLobby {
    pub fn join_raceroom(&mut self, room: &String, passwd: Option<String>, page: &mut UiPageCtx) {
        let race_join = RaceJoin {token: page.store.user_token.clone(), room: room.clone(), passwd};
        let url = page.store.get_http_url("api/race/join");
        let tx = page.tx.clone();
        page.store.curr_room = room.clone();
        tokio::spawn(async move {
            let res = reqwest::Client::new().post(url).json(&race_join).send().await.unwrap();
            match res.status() {
                StatusCode::OK => {
                    tx.send(UiMsg::MsgGotoPage(UiPageState::PageInRoom)).await.unwrap();
                }
                _ => {
                    tx.send(UiMsg::MsgSetErrState("加入房间失败, 请稍后重试。".to_string())).await.unwrap();
                }
            }
        });
    }

    pub fn enter_raceroom(&mut self, room: &String, passwd: Option<String>, page: &mut UiPageCtx) {
        if room == &page.store.curr_room {
            page.route.switch_to_page(UiPageState::PageInRoom);
            return;
        }

        return self.join_raceroom(room, passwd, page);
    }

    pub fn show_passwindow(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        egui::Window::new("pop pass window").fixed_pos(egui::pos2(350.0, 200.0)).fixed_size([200.0, 80.0])
        .title_bar(false)
        .show(ctx, |ui| {
            ui.label("请输入房间密码: ");   
            ui.horizontal_centered(|ui| {
                ui.text_edit_singleline(&mut self.select_room_pass);
                if ui.button("确认").clicked() {
                    self.show_passwin = false;
                    let pass = Some(self.select_room_pass.clone());
                    self.join_raceroom(&self.select_room.clone(), pass, page);
                }
            });
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

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        if let Ok(msg) = self.rx.try_recv() {
            self.table_data = msg;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(80.0);
                ui.vertical_centered(|ui| {
                    Grid::new("race rooms").min_col_width(120.0).show(ui, |ui| {
                        for content in &self.table_head {
                            ui.label(*content);
                        }
                        ui.end_row();

                        let table_data = self.table_data.clone();
                        for (index, race) in table_data.iter().enumerate() {
                            let table = vec![
                                (index+1).to_string(),
                                race.name.clone(),
                                race.stage.clone(),
                                race.owner.clone(),
                                race.players.to_string() + "/8", 
                                match race.state {
                                    RoomState::RoomFree => String::from("空闲"),
                                    RoomState::RoomFull => String::from("满员"),
                                    RoomState::RoomLocked => String::from("需要密码"),
                                    RoomState::RoomRaceOn => String::from("比赛中"),
                                }
                            ];
                            for content in table {
                                ui.label(content);
                            }

                            match race.state {
                                RoomState::RoomFree => {
                                    if ui.button("加入").clicked() {
                                        self.enter_raceroom(&race.name, None, page);
                                    }
                                },
                                RoomState::RoomLocked => {
                                    if ui.button("加入").clicked() {
                                        self.select_room = race.name.clone();
                                        self.show_passwin = true;
                                    }
                                }
                                _ => {},
                            }
                            ui.end_row();
                        }
                    });
                });
                ui.add_space(120.0);
            });

            if self.show_passwin {
                self.show_passwindow(ctx, frame, page);
            }
        });
    }
}
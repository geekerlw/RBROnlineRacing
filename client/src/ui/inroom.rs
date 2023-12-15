use eframe::egui;
use egui::Grid;
use protocol::httpapi::{RaceQuery, RaceInfo, UserAccess};
use reqwest::StatusCode;
use crate::ui::UiPageState;
use super::{UiView, UiPageCtx};
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct UiInRoom {
    pub room_name: String,
    pub raceinfo: RaceInfo,
    rx: Receiver<RaceInfo>,
    tx: Sender<RaceInfo>,
}

impl Default for UiInRoom {
    fn default() -> Self {
        let (tx, rx) = channel::<RaceInfo>(8);
        Self { 
            room_name: "No Room Info".to_string(),
            raceinfo: RaceInfo::default(),
            rx,
            tx,
        }
    }
}

impl UiView for UiInRoom {
    fn enter(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        self.room_name = page.store.curr_room.clone();
        let url = page.store.get_http_url("api/race/info");
        let tx = self.tx.clone();
        let query = RaceQuery {name: self.room_name.clone()};
        tokio::spawn(async move {
            let res = reqwest::Client::new().get(url).json(&query).send().await.unwrap();
            if res.status() == StatusCode::OK {
                let text = res.text().await.unwrap();
                let raceinfo: RaceInfo = serde_json::from_str(text.as_str()).unwrap();
                tx.send(raceinfo).await.unwrap();
            }
        });
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        if let Ok(msg) = self.rx.try_recv() {
            self.raceinfo = msg;
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
                        ui.label("Always new");
                        ui.end_row();

                        ui.label("车辆调教: ");
                        ui.label("Default");
                    });

                    ui.add_space(20.0);

                    Grid::new("race players table")
                    .min_col_width(80.0)
                    .min_row_height(24.0)
                    .show(ui, |ui| {
                        ui.label("序号");
                        ui.label("车手");
                        ui.end_row();
                        for (index, player) in self.raceinfo.players.iter().enumerate() {
                            ui.label(index.to_string());
                            ui.label(player);
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
        if !page.store.user_token.is_empty() {
            let user: UserAccess = UserAccess{token: page.store.user_token.clone()};
            let url = page.store.get_http_url("api/race/leave");
            tokio::spawn(async move {
                let _res = reqwest::Client::new().post(url).json(&user).send().await.unwrap();
            });
        }

        page.store.curr_room.clear();
        self.room_name.clear();
    }
}
use eframe::egui;
use egui::Grid;
use reqwest::StatusCode;
use crate::ui::UiPageState;
use super::{UiView, UiPageCtx, UiMsg};

#[derive(Clone)]
pub struct UiInRoom {
    pub room_name: String,
    pub stage: String,
    pub stage_id: u32,
    pub car: String,
    pub car_id: u32,
    pub damage: u32,
    pub setup: String,
    pub players: Vec<String>,
}

impl Default for UiInRoom {
    fn default() -> Self {
        Self { 
            room_name: "Test Room".to_string(),
            stage: "Semetin 2009".to_string(),
            stage_id: 0,
            car: "Ford Fiesta 2019".to_string(),
            car_id: 1,
            damage: 0,
            setup: "Default".to_string(),
            players: vec!["Ziye".to_string(), "Somechen".to_string(), "Shanyin".to_string()],
        }
    }
}

impl UiView for UiInRoom {
    fn set_param(&mut self, value: serde_json::Value) {
        self.room_name = String::from(value["name"].as_str().unwrap());
    }

    fn enter(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        let url = page.store.get_http_url("api/race/info");
        let tx = page.tx.clone();
        let roomname = self.room_name.clone();
        tokio::spawn(async move {
            let res = reqwest::Client::new().get(url).query(&[("name", roomname)]).send().await.unwrap();
            if res.status() == StatusCode::OK {
                //tx.send(UiMsg::MsgRaceRoomCreated(create_info)).await.unwrap();
            }
        });
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(120.0);
                ui.vertical(|ui| {
                    Grid::new("race room table")
                    .min_col_width(80.0)
                    .min_row_height(24.0)
                    .show(ui, |ui| {
                        ui.label("比赛赛道：");
                        ui.label(self.stage.clone());
                        ui.end_row();

                        ui.label("比赛车辆: ");
                        ui.label(self.car.clone());
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
                        for (index, player) in self.players.iter().enumerate() {
                            ui.label(index.to_string());
                            ui.label(player);
                            ui.end_row();
                        }
                    });

                    ui.add_space(20.0);
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(80.0);
                        if ui.button("退出").clicked() {
                            page.route.switch_to_page(UiPageState::PageLobby);
                        }
                        if ui.button("准备").clicked() {
                            page.route.switch_to_page(UiPageState::PageLoading);
                        }
                    });
                });
            });
        });
    }
}
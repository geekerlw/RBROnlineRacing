use eframe::egui;
use egui::Grid;
use egui::ComboBox;
use protocol::httpapi::RaceInfo;
use protocol::httpapi::RoomState;
use reqwest::StatusCode;
use super::{UiView, UiPageCtx, UiMsg};
use crate::game::rbr::{RBRGame, RBRStageData, RBRCarData};
use crate::ui::UiPageState;

#[derive(Clone)]
pub struct UiCreateRace {
    pub room_name: String,
    pub stages: Vec<RBRStageData>,
    pub select_stage: usize,
    pub cars: Vec<RBRCarData>,
    pub select_car: usize,
    pub damages: Vec<&'static str>,
    pub select_damage: usize,
}

impl Default for UiCreateRace {
    fn default() -> Self {
        Self { 
            room_name: "Test Room".to_string(),
            stages: vec![],
            select_stage: 0,
            cars: vec![],
            select_car: 0,
            damages: vec!["Off", "Safe", "Reduced", "Realistic"],
            select_damage: 3,
        }
    }
}

impl UiView for UiCreateRace {
    fn init(&mut self, page: &mut UiPageCtx) {
        let mut rbr = RBRGame::new(&page.store.game_path);
        if let Some(stages) = rbr.load_game_stages() {
            self.stages = stages;
        }
        if let Some(cars) = rbr.load_game_cars() {
            self.cars = cars;
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(120.0);
                ui.vertical(|ui| {
                    Grid::new("race create table")
                    .min_col_width(80.0)
                    .min_row_height(24.0)
                    .show(ui, |ui| {
                        ui.label("房间名称：");
                        ui.text_edit_singleline(&mut self.room_name);
                        ui.end_row();

                        ui.label("比赛赛道：");
                        ComboBox::from_id_source("select stage").selected_text(self.stages[self.select_stage].name.clone())
                        .show_ui(ui, |ui| {
                            for (index, stage) in self.stages.iter().enumerate() {
                                if ui.selectable_label(self.select_stage == index, &stage.name).clicked() {
                                    self.select_stage = index;
                                }
                            }
                        });
                        ui.end_row();

                        ui.label("比赛车辆: ");
                        ComboBox::from_id_source("select car").selected_text(self.cars[self.select_car].name.clone())
                        .show_ui(ui, |ui| {
                            for (index, car) in self.cars.iter().enumerate() {
                                if ui.selectable_label(self.select_car == index, &car.name).clicked() {
                                    self.select_car = index;
                                }
                            }
                        });
                        ui.end_row();

                        ui.label("车辆损坏：");
                        ComboBox::from_id_source("select damage").selected_text(self.damages[self.select_damage])
                        .show_ui(ui, |ui| {
                            for (index, damage) in self.damages.iter().enumerate() {
                                if ui.selectable_label(self.select_car == index, damage.to_string()).clicked() {
                                    self.select_damage = index;
                                }
                            }
                        });
                        ui.end_row();
                    });

                    ui.add_space(20.0);

                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(80.0);
                        if ui.button("取消").clicked() {
                            page.route.back_from_page(UiPageState::PageCreate);
                        }
                        if ui.button("确认").clicked() {
                            self.create_room(page);
                        }
                    });
                });
            });
        });
    }
}

impl UiCreateRace {
    fn create_room(&mut self, page: &mut UiPageCtx) {
        let raceinfo = RaceInfo{
            token: page.store.user_token.clone(),
            name: self.room_name.clone(),
            stage: self.stages[self.select_stage].name.clone(),
            stage_id: self.stages[self.select_stage].stage_id.parse().unwrap(),
            car: Some(self.cars[self.select_car].name.clone()),
            car_id: Some(self.cars[self.select_car].id.parse().unwrap()),
            damage: self.select_damage as u32,
            state: RoomState::default(),
            players: Vec::<String>::new(),
        };

        let url = page.store.get_http_url("api/race/create");
        let tx = page.tx.clone();
        let room_name = self.room_name.clone();
        tokio::spawn(async move {
            let res = reqwest::Client::new().post(url).json(&raceinfo).send().await.unwrap();
            match res.status() {
                StatusCode::OK => {
                    tx.send(UiMsg::MsgSetRoomInfo(room_name)).await.unwrap();
                    tx.send(UiMsg::MsgGotoPage(UiPageState::PageInRoom)).await.unwrap();
                }
                _ => {
                    tx.send(UiMsg::MsgSetErrState("Failed to create race room".to_string())).await.unwrap();
                }
            }
        });
    }
}
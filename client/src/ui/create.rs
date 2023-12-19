use eframe::egui;
use egui::Grid;
use egui::ComboBox;
use protocol::httpapi::RaceInfo;
use protocol::httpapi::RoomState;
use reqwest::StatusCode;
use super::{UiView, UiPageCtx, UiMsg};
use crate::game::rbr::RBRGame;
use crate::ui::UiPageState;

#[derive(Clone)]
pub struct UiCreateRace {
    pub room_name: String,
    pub stage: Vec<String>,
    pub stage_id: Vec<u32>,
    pub stage_index: usize,
    pub car: Vec<String>,
    pub car_id: Vec<u32>,
    pub car_index: usize,
    pub damage: u32,
    pub setup: String,
}

impl Default for UiCreateRace {
    fn default() -> Self {
        Self { 
            room_name: "Test Room".to_string(),
            stage: vec!["Semetin 2009".to_string(), "Semetin 2010".to_string()],
            stage_id: vec![0, 1],
            stage_index: 0,
            car: vec!["Ford Fiesta 2019".to_string(), "Ford Fiesta R2".to_string()],
            car_id: vec![1, 2],
            car_index: 0,
            damage: 0,
            setup: "Default".to_string(),
        }
    }
}

impl UiView for UiCreateRace {
    fn init(&mut self, page: &mut UiPageCtx) {
        let mut rbr = RBRGame::new(&page.store.game_path);
        //rbr.load_game_stages();
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
                        ComboBox::from_id_source("select stage").selected_text(self.stage[self.stage_index].clone())
                        .show_ui(ui, |ui| {
                            for (index, text) in self.stage.iter().enumerate() {
                                if ui.selectable_label(self.stage_index == index, text).clicked() {
                                    self.stage_index = index;
                                }
                            }
                        });
                        ui.end_row();

                        ui.label("比赛车辆: ");
                        ComboBox::from_id_source("select car").selected_text(self.car[self.car_index].clone())
                        .show_ui(ui, |ui| {
                            for (index, text) in self.car.iter().enumerate() {
                                if ui.selectable_label(self.car_index == index, text).clicked() {
                                    self.car_index = index;
                                }
                            }
                        });
                        ui.end_row();

                        ui.label("车辆损坏：");
                        ui.label("Always new");
                        ui.end_row();

                        ui.label("车辆调教: ");
                        ui.label("Default");
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
            stage: self.stage[self.stage_index].clone(),
            car: Some(self.car[self.car_index].clone()),
            damage: Some(self.damage),
            setup: Some(self.setup.clone()),
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
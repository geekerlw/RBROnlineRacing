use eframe::egui;
use egui::Grid;
use crate::{route::RacingRoute, UiPageState};
use crate::store::RacingStore;
use super::PageView;

#[derive(Clone)]
pub struct UiInRoom {
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
        Self { stage: "Semetin 2009".to_string(),
            stage_id: 0,
            car: "Ford Fiesta 2019".to_string(),
            car_id: 1,
            damage: 0,
            setup: "Default".to_string(),
            players: vec!["Ziye".to_string(), "Somechen".to_string(), "Shanyin".to_string()],
        }
    }
}

impl PageView for UiInRoom {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, route: &mut RacingRoute, store: &mut RacingStore) {
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
                            route.switch_to_page(UiPageState::PageLobby);
                        }
                        if ui.button("准备").clicked() {
                            route.switch_to_page(UiPageState::PageLoading);
                        }
                    });
                });
            });
        });
    }
}
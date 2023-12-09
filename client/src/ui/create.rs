use eframe::egui;
use egui::Grid;
use egui::RichText;
use egui::ComboBox;

#[derive(Clone)]
pub struct UiCreateRace {
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
        Self { stage: vec!["Semetin 2009".to_string(), "Semetin 2010".to_string()],
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

impl UiCreateRace {
    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(60.0);
            ui.horizontal(|ui| {
                ui.add_space(180.0);
                Grid::new("race create table").show(ui, |ui| {
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
            });
        });
    }
}
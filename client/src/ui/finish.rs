use eframe::egui;
use egui::Grid;
use protocol::httpapi::{MetaRaceResult, MetaRaceData};

#[derive(Clone)]
pub struct UiFinish {
    pub table_head: Vec<&'static str>,
    pub table_data: MetaRaceResult,
}

impl Default for UiFinish {
    fn default() -> Self {
        Self {
            table_head: vec!["排名", "车手", "分段1", "分段2", "完成时间"],
            table_data: MetaRaceResult {
                state: protocol::httpapi::RaceState::RaceFinished,
                board: vec![MetaRaceData {
                    token: String::from("token"),
                    profile_name: String::from("Ziye"),
                    starttime: 0.0,
                    racetime: 120.0,
                    process: 100.0,
                    splittime1: 30.0,
                    splittime2: 80.0,
                    finishtime: 120.0,
                },
                MetaRaceData {
                    token: String::from("token"),
                    profile_name: String::from("somechen"),
                    starttime: 0.0,
                    racetime: 120.0,
                    process: 100.0,
                    splittime1: 33.0,
                    splittime2: 90.0,
                    finishtime: 140.0,
                }]
            }
        }
    }
}

impl UiFinish {
    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(120.0);
                ui.vertical_centered(|ui| {
                    Grid::new("race result").min_col_width(120.0).show(ui, |ui| {
                        for content in &self.table_head {
                            ui.label(*content);
                        }
                        ui.end_row();

                        for (index, result) in self.table_data.board.iter().enumerate() {
                            let table = vec![index.to_string(),
                                result.profile_name.clone(),
                                result.splittime1.to_string(),
                                result.splittime2.to_string(),
                                result.finishtime.to_string(),
                            ];
                            for content in table {
                                ui.label(content);
                            }
                            ui.end_row();
                        }
                    });
                })
            });
        });
    }
}
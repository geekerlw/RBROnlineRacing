use eframe::egui;
use egui::{ScrollArea, Grid};
use egui::{FontDefinitions, FontData};

#[derive(Default)]
pub struct RacingClient {
    pub page_index: u8,
    pub file_content: String,
}

impl RacingClient {
    pub fn configure_font(self, ctx: &egui::Context) -> Self {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert("msyh".to_owned(), FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\msyh.ttc")));
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "msyh".to_owned());
        ctx.set_fonts(fonts);
        self
    }

    pub fn draw_mainwindow(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("菜单", |ui| {
                    ui.vertical(|ui| {
                        if ui.button("Open").clicked() {
                            ui.close_menu();
                        }
                        if ui.button("Quit").clicked() {
                            frame.close();
                        }
                    });
                });
                
                ui.menu_button("Help", |ui| {
                    ui.vertical(|ui| {
                        if ui.button("About").clicked() {
                            ui.close_menu();
                        }
                    });
                });
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.page_index == 0 {
                Grid::new("race list").min_col_width(150.0)
                    .show(ui,|ui| {
                    let table_data = vec![
                        vec!["序号", "房间名称", "比赛赛道", "房主", "状态"],
                        vec!["1", "Test room 1", "semetin 2009", "ziye", "等待中"],
                        vec!["2", "Test room 2", "semetin 2009", "ziye", "等待中"],
                        vec!["3", "Test room 3", "semetin 2009", "ziye", "等待中"],
                        vec!["4", "Test room 4", "semetin 2009", "ziye", "等待中"],
                    ];

                    for row in table_data {
                        for cell in row {
                            ui.label(cell);
                        }
                        if ui.button("加入").clicked() {
                            self.page_index = 1;
                        }
                        ui.end_row();
                    }
                })
            } else {
                Grid::new("race list").min_col_width(150.0)
                    .show(ui,|ui| {
                    let table_data = vec![
                        vec!["序号", "房间名称", "比赛赛道", "房主", "状态"],
                        vec!["1", "Test room 1", "semetin 2009", "ziye", "等待中"],
                        vec!["2", "Test room 2", "semetin 2009", "ziye", "等待中"],
                        vec!["3", "Test room 3", "semetin 2009", "ziye", "等待中"],
                        vec!["4", "Test room 4", "semetin 2009", "ziye", "等待中"],
                    ];

                    for row in table_data {
                        for cell in row {
                            ui.label(cell);
                        }
                        if ui.button("退出").clicked() {
                            self.page_index = 0;
                        }
                        ui.end_row();
                    }
                })
            }
        });
        egui::TopBottomPanel::bottom("status bar").show(ctx, |ui| {
            ui.label("this is a status bar");
        });
    }
}

impl eframe::App for RacingClient {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.draw_mainwindow(ctx, frame);
    }
}
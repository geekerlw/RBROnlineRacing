use eframe::egui;
use egui::Grid;
use egui::RichText;
use rfd::FileDialog;
use super::UiPageState;
use super::{UiView, UiPageCtx};

#[derive(Default, Clone)]
pub struct UiSetting {
    pub gamepath: String,
    pub server_addr: String,
}

impl UiView for UiSetting {
    fn init(&mut self, page: &mut UiPageCtx) {
        self.gamepath = page.store.game_path.clone();
        self.server_addr = page.store.server_addr.clone();
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(120.0);
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("服务设置").size(18.0));
                        ui.add_space(20.0);
                        Grid::new("server setting table")
                        .min_col_width(80.0)
                        .min_row_height(24.0)
                        .show(ui, |ui| {
                            ui.label("服务器地址: ");
                            ui.add_sized([300.0, 24.0], egui::TextEdit::singleline(&mut self.server_addr));
                            ui.end_row();
                        });
                    });
                    ui.add_space(40.0);
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("游戏设置").size(18.0));
                        ui.add_space(20.0);
                        Grid::new("game setting table")
                        .min_col_width(80.0)
                        .min_row_height(24.0)
                        .show(ui, |ui| {
                            ui.label("RBR根目录: ");
                            ui.add_sized([300.0, 24.0], egui::TextEdit::singleline(&mut self.gamepath));
                            ui.add_space(10.0);
                            if ui.button("浏览").clicked() {
                                if let Some(path) = FileDialog::new().pick_folder() {
                                    if let Some(pathstr) = path.to_str() {
                                        self.gamepath = pathstr.to_string();
                                    }
                                }
                            }
                            ui.end_row();
                        });
                    });
                    
                    ui.add_space(60.0);
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(200.0);
                        if ui.button("取消").clicked() {
                            page.route.back_from_page(UiPageState::PageSetting);
                        }
                        if ui.button("确认").clicked() {
                            self.save_setting(page);
                            if page.store.user_token.is_empty() {
                                page.route.switch_to_page(UiPageState::PageLogin);
                            } else {
                                page.route.switch_to_page(UiPageState::PageLobby);
                            }
                        }
                    });
                });
            });
        });
    }
}

impl UiSetting {
    fn save_setting(&mut self, page: &mut UiPageCtx) {
        page.store.game_path = self.gamepath.clone();
        page.store.server_addr = self.server_addr.clone();
        page.store.save_config();
    }
}
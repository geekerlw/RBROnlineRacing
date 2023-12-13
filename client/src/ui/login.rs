use eframe::egui;
use egui::RichText;
use crate::components::store::RacingStore;
use crate::components::route::RacingRoute;
use crate::ui::UiPageState;
use super::UiView;

#[derive(Default, Clone)]
pub struct UiLogin {
}

impl UiView for UiLogin {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, route: &mut RacingRoute, store: &mut RacingStore) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    ui.label(RichText::new("致每一位热爱理查德伯恩斯拉力赛的小伙伴：").size(24.0));
                    ui.add_space(10.0);
                    ui.label(RichText::new("翻得开心，寄得愉快！").size(32.0));
                    ui.add_space(40.0);
                    ui.label(RichText::new("SimRallyCN 中国总群: 658110104").size(24.0));
                    ui.add_space(10.0);
                    ui.label(RichText::new("作者：子夜(Lw_Ziye), Copyright (c) 2023, 有疑问请进群@Lw_Ziye。").size(16.0));
                    ui.add_space(50.0);
                    if ui.button("知道了啦").clicked() {
                        route.switch_to_page(UiPageState::PageLobby);
                    }
                });
            });
        });
    }
}
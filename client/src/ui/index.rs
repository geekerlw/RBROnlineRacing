use eframe::egui;
use egui::{FontDefinitions, FontData};
use super::UiPageState;
use crate::components::store::RacingStore;
use crate::components::route::RacingRoute;
use crate::ui::UiView;

#[derive(Default)]
pub struct UiRacingApp {
    pub store: RacingStore,
    pub route: RacingRoute,

    pub login: super::login::UiLogin,
    pub finish: super::finish::UiFinish,
    pub loading: super::loading::UiLoading,
    pub lobby: super::lobby::UiLobby,
    pub racing: super::racing::UiRacing,
    pub setting: super::setting::UiSetting,
    pub create: super::create::UiCreateRace,
    pub inroom: super::inroom::UiInRoom,
}

impl UiRacingApp {
    pub fn configure_font(self, ctx: &egui::Context) -> Self {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert("msyh".to_owned(), FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\msyh.ttc")));
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "msyh".to_owned());
        ctx.set_fonts(fonts);
        self
    }

    pub fn switch_to_page(&mut self, page: UiPageState) {
        self.route.switch_to_page(page);
    }
}
 
impl eframe::App for UiRacingApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("主页").clicked() {
                    self.switch_to_page(UiPageState::PageLogin);
                }
                ui.menu_button("比赛大厅", |ui| {
                    ui.vertical(|ui| {
                        if ui.button("进入大厅").clicked() {
                            self.switch_to_page(UiPageState::PageLobby);
                            ui.close_menu();
                        }
                        if ui.button("创建比赛").clicked() {
                            self.switch_to_page(UiPageState::PageCreate);
                            ui.close_menu();
                        }
                    });
                });
                if ui.button("设置").clicked() {
                    self.switch_to_page(UiPageState::PageSetting);
                }
                if ui.button("帮助").clicked() {
                    self.switch_to_page(UiPageState::PageLogin);
                }
            })
        });

        egui::TopBottomPanel::bottom("status bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(String::from("用户："));
                ui.label(&self.store.user_name);
                ui.separator();
                ui.label("状态：");
                self.store.show_user_state(ui);
            });
        });

        match self.route.curr_page {
            UiPageState::PageLogin => self.login.update(ctx, frame, &mut self.route, &mut self.store),
            UiPageState::PageFinish => self.finish.update(ctx, frame, &mut self.route, &mut self.store),
            UiPageState::PageLoading => self.loading.update(ctx, frame, &mut self.route, &mut self.store),
            UiPageState::PageLobby => self.lobby.update(ctx, frame, &mut self.route, &mut self.store),
            UiPageState::PageRacing => self.racing.update(ctx, frame, &mut self.route, &mut self.store),
            UiPageState::PageSetting => self.setting.update(ctx, frame, &mut self.route, &mut self.store),
            UiPageState::PageCreate => self.create.update(ctx, frame, &mut self.route, &mut self.store),
            UiPageState::PageInRoom => self.inroom.update(ctx, frame, &mut self.route, &mut self.store),
        }
    }
}
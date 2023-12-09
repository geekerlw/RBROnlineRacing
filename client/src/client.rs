use eframe::egui;
use egui::{FontDefinitions, FontData};
use crate::{UiPageState, UiPages};
use crate::store::RacingStore;
use protocol::httpapi::RaceState;

#[derive(Default, Clone)]
pub struct RacingClient {
    pub store: RacingStore,
    pub ui: UiPages,
}

impl RacingClient {
    pub fn configure_font(self, ctx: &egui::Context) -> Self {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert("msyh".to_owned(), FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\msyh.ttc")));
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "msyh".to_owned());
        ctx.set_fonts(fonts);
        self
    }
}
 
impl eframe::App for RacingClient {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("主页").clicked() {
                    self.store.swich_page(UiPageState::PageLogin);
                }
                ui.menu_button("比赛大厅", |ui| {
                    ui.vertical(|ui| {
                        if ui.button("进入大厅").clicked() {
                            self.store.swich_page(UiPageState::PageLobby);
                            ui.close_menu();
                        }
                        if ui.button("创建比赛").clicked() {
                            self.store.swich_page(UiPageState::PageCreate);
                        }
                    });
                });
                if ui.button("设置").clicked() {
                    self.store.swich_page(UiPageState::PageSetting);
                }
            })
        });

        egui::TopBottomPanel::bottom("status bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(String::from("用户："));
                ui.label(self.store.user_name.clone());
                ui.separator();
                ui.label("状态：");
                match self.store.user_state {
                    RaceState::RaceDefault => ui.label("空闲"),
                    RaceState::RaceFinished => ui.label("比赛完成"),
                    RaceState::RaceInit => ui.label("初始化比赛"),
                    RaceState::RaceLoad => ui.label("比赛加载中"),
                    RaceState::RaceLoaded => ui.label("比赛加载完成"),
                    RaceState::RaceReady => ui.label("比赛就绪"),
                    RaceState::RaceRetired => ui.label("比赛已放弃"),
                    RaceState::RaceRunning => ui.label("比赛进行中"),
                    RaceState::RaceStart => ui.label("比赛开始"),
                };
            });
        });

        match self.store.curr_page {
            UiPageState::PageLogin => self.ui.login.update(ctx, frame, &mut self.store),
            UiPageState::PageFinish => self.ui.finish.update(ctx, frame),
            UiPageState::PageLoading => self.ui.loading.update(ctx, frame),
            UiPageState::PageLobby => self.ui.lobby.update(ctx, frame, &mut self.store),
            UiPageState::PageRacing => self.ui.racing.update(ctx, frame),
            UiPageState::PageSetting => self.ui.setting.update(ctx, frame),
            UiPageState::PageCreate => self.ui.create.update(ctx, frame),
            UiPageState::PageInRoom => self.ui.inroom.update(ctx, frame),
        }
    }
}
use eframe::egui;
use egui::{FontDefinitions, FontData};
use protocol::httpapi::UserAccess;
use crate::ui;
use crate::ui::{UiPageCtx, UiMsg, UiPageState};

#[derive(Default)]
pub struct RacingClient {
    pub ctx: UiPageCtx,
    pub curr_page: UiPageState,
    pub pages: Vec<Box<dyn ui::UiView>>,
}

impl RacingClient {
    pub fn init(mut self) -> Self {
        self.pages.insert(UiPageState::PageLogin as usize, Box::new(ui::login::UiLogin::default()));
        self.pages.insert(UiPageState::PageLobby as usize, Box::new(ui::lobby::UiLobby::default()));
        self.pages.insert(UiPageState::PageCreate as usize, Box::new(ui::create::UiCreateRace::default()));
        self.pages.insert(UiPageState::PageInRoom as usize, Box::new(ui::inroom::UiInRoom::default()));
        self.pages.insert(UiPageState::PageRacing as usize, Box::new(ui::racing::UiRacing::default()));
        self.pages.insert(UiPageState::PageSetting as usize, Box::new(ui::setting::UiSetting::default()));

        self.ctx.store.init();

        for (_, page) in self.pages.iter_mut().enumerate() {
            page.init(&mut self.ctx);
        }

        self
    }

    pub fn configure_font(self, ctx: &egui::Context) -> Self {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert("msyh".to_owned(), FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\msyh.ttc")));
        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "msyh".to_owned());
        ctx.set_fonts(fonts);
        self
    }

    pub fn switch_to_page(&mut self, page: UiPageState) {
        self.ctx.route.switch_to_page(page);
    }

    pub fn handle_async_uimsg(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Ok(msg) = self.ctx.rx.try_recv() {
            match msg {
                UiMsg::MsgGotoPage(state) => {
                    self.ctx.route.switch_to_page(state);
                },
                UiMsg::MsgUserLogined(token) => {
                    self.ctx.store.user_token = token;
                },
                UiMsg::MsgSetRoomInfo(room) => {
                    self.ctx.store.curr_room = room;
                },
                UiMsg::MsgSetErrState(err) => {
                    self.ctx.store.user_state = err;
                },
                UiMsg::MsgQuitApp => {
                    frame.close();
                }
            };
        }
        ctx.request_repaint();
    }
}
 
impl eframe::App for RacingClient {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if !self.ctx.store.user_token.is_empty() {
            let user: UserAccess = UserAccess{token: self.ctx.store.user_token.clone()};
            let url_leave = self.ctx.store.get_http_url("api/race/leave");
            let url_logout = self.ctx.store.get_http_url("api/user/logout");
            let is_inroom = !self.ctx.store.curr_room.is_empty();
            tokio::spawn(async move {
                if is_inroom {
                    let _res = reqwest::Client::new().post(url_leave).json(&user).send().await.unwrap();
                }
                let _res = reqwest::Client::new().post(url_logout).json(&user).send().await.unwrap();
            });
        }

        for (_, page) in self.pages.iter_mut().enumerate() {
            page.quit(&mut self.ctx);
        }
        self.ctx.store.save_config();
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.handle_async_uimsg(ctx, frame);

        egui::TopBottomPanel::top("menu bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("主页").clicked() {
                    self.switch_to_page(UiPageState::PageLogin);
                }
                if !self.ctx.store.user_token.is_empty() {
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
                }
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
                ui.label(&self.ctx.store.user_name);
                ui.separator();
                ui.label("状态：");
                self.ctx.store.show_user_state(ui);
            });
        });

        if self.curr_page != self.ctx.route.curr_page {
            self.pages[self.ctx.route.prev_page.clone() as usize].exit(ctx, frame, &mut self.ctx);
            self.pages[self.ctx.route.curr_page.clone() as usize].enter(ctx, frame, &mut self.ctx);
            self.curr_page = self.ctx.route.curr_page.clone();
        }
        self.pages[self.curr_page.clone() as usize].update(ctx, frame, &mut self.ctx);
    }
}
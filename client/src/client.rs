use eframe::egui;
use egui::{FontDefinitions, FontData};
use protocol::httpapi::UserAccess;
use crate::ui;
use crate::ui::{UiPageState, UiView};
use crate::ui::{UiPageCtx, UiMsg};
use reqwest::StatusCode;

#[derive(Default)]
pub struct RacingClient {
    pub ctx: UiPageCtx,

    pub login: ui::login::UiLogin,
    pub finish: ui::finish::UiFinish,
    pub loading: ui::loading::UiLoading,
    pub lobby: ui::lobby::UiLobby,
    pub racing: ui::racing::UiRacing,
    pub setting: ui::setting::UiSetting,
    pub create: ui::create::UiCreateRace,
    pub inroom: ui::inroom::UiInRoom,
}

impl RacingClient {
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

    pub fn handle_async_uimsg(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(msg) = self.ctx.rx.try_recv() {
            match msg {
                UiMsg::MsgGotoPage(state) => {
                    self.ctx.route.switch_to_page(state);
                },
                UiMsg::MsgUserLogined(token) => {
                    self.ctx.store.user_token = token;
                },
            };
        }
        ctx.request_repaint();
    }
}
 
impl eframe::App for RacingClient {
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        if !self.ctx.store.user_token.is_empty() {
            let user: UserAccess = UserAccess{token: self.ctx.store.user_token.clone()};
            let url = self.ctx.store.get_http_url("api/user/logout");
            tokio::spawn(async move {
                let _res = reqwest::Client::new().post(url).json(&user).send().await.unwrap();
            });
        }
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

        match self.ctx.route.curr_page {
            UiPageState::PageLogin => self.login.update(ctx, frame, &mut self.ctx),
            UiPageState::PageFinish => self.finish.update(ctx, frame, &mut self.ctx),
            UiPageState::PageLoading => self.loading.update(ctx, frame, &mut self.ctx),
            UiPageState::PageLobby => self.lobby.update(ctx, frame, &mut self.ctx),
            UiPageState::PageRacing => self.racing.update(ctx, frame, &mut self.ctx),
            UiPageState::PageSetting => self.setting.update(ctx, frame, &mut self.ctx),
            UiPageState::PageCreate => self.create.update(ctx, frame, &mut self.ctx),
            UiPageState::PageInRoom => self.inroom.update(ctx, frame, &mut self.ctx),
        }
    }
}
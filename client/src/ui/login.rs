use eframe::egui;
use egui::RichText;
use protocol::httpapi::{UserLogin, API_VERSION_STRING};
use reqwest::StatusCode;
use crate::ui::UiPageState;
use super::{UiMsg, UiView, UiPageCtx};

#[derive(Default, Clone)]
pub struct UiLogin {
}

impl UiLogin {
    fn login(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        if page.store.user_token.is_empty() {
            let user = UserLogin{name: page.store.user_name.clone(), passwd: page.store.user_passwd.clone()};
            let url_ver = page.store.get_http_url("api/version");
            let url_login = page.store.get_http_url("api/user/login");
            let tx = page.tx.clone();
            tokio::spawn(async move {
                let res = reqwest::get(url_ver).await.unwrap();
                if res.status() == StatusCode::OK {
                    let version = res.text().await.unwrap();
                    if version != API_VERSION_STRING {
                        tx.send(UiMsg::MsgSetErrState("客户端版本不匹配，请更新版本!".to_string())).await.unwrap();
                        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                        tx.send(UiMsg::MsgQuitApp).await.unwrap();
                    }
                }

                let res = reqwest::Client::new().post(url_login).json(&user).send().await.unwrap();
                if res.status() == StatusCode::OK {
                    let token = res.text().await.unwrap();
                    tx.send(UiMsg::MsgUserLogined(token)).await.unwrap();
                    tx.send(UiMsg::MsgGotoPage(UiPageState::PageLobby)).await.unwrap();
                }
            });
        } else {
            page.route.switch_to_page(UiPageState::PageLobby);
        }
    }
}

impl UiView for UiLogin {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, page: &mut UiPageCtx) {
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
                    if !page.store.user_name.is_empty() && !page.store.user_passwd.is_empty() {
                        if ui.button("知道了啦").clicked() {
                            self.login(ctx, frame, page);
                        }
                    }
                });
            });
        });
    }
}
use libc::c_char;
use log::info;
use protocol::httpapi::UserLogin;
use protocol::API_VERSION_STRING;
use reqwest::StatusCode;
use crate::game::plugin::IPlugin;
use crate::game::hacker::*;
use ini::Ini;
use tokio::runtime::Handle;
use crate::components::store::RacingStore;
use tokio::sync::mpsc::{channel, Receiver, Sender};

enum InnerMsg {
    MsgSetErrState(String),
    MsgUserLogined(String),
}

pub struct RBNHelper {
    tokio: Handle,
    rx: Receiver<InnerMsg>,
    tx: Sender<InnerMsg>,
    store: RacingStore,
    message: String,
    copyright: String,
}

impl Default for RBNHelper {
    fn default() -> Self {
        let (tx, rx) = channel::<InnerMsg>(64);
        Self {
            tokio: tokio::runtime::Builder::new_multi_thread().enable_all().build().expect("Failed to init tokio runtime.").handle().clone(),
            rx,
            tx,
            store: RacingStore::default(),
            message: String::new(),
            copyright: format!("Welcome to use RBN Helper [{}], Copyright Lw_Ziye 2023-2024.", std::env!("CARGO_PKG_VERSION")),
        }
    }
}

impl IPlugin for RBNHelper {
    fn GetName(&self) -> *const libc::c_char {
        let name = std::ffi::CString::new("RBN Helper").unwrap();
        name.into_raw()
    }
}

impl RBNHelper {
    pub fn init(&mut self) {
        self.store.init();
        self.load_dashboard_config();
        self.init_async_runtime();
        self.check_and_login();
    }

    fn init_async_runtime(&mut self) {
        let runtime = self.tokio.clone();
        std::thread::spawn(move || {
            runtime.block_on(async {
                info!("started tokio runtime success.");
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                };
            });
        });
    }

    fn check_and_login(&mut self) {
        let url_ver = self.store.get_http_url("api/version");
        let url_login = self.store.get_http_url("api/user/login");
        let user = UserLogin{name: self.store.user_name.clone(), passwd: self.store.user_passwd.clone()};
        let tx = self.tx.clone();
        self.tokio.spawn(async move {
            let res = reqwest::get(url_ver).await.unwrap();
            if res.status() == StatusCode::OK {
                let version = res.text().await.unwrap();
                if version != API_VERSION_STRING {
                    tx.send(InnerMsg::MsgSetErrState("Version is out of date, please upgrade!".to_string())).await.unwrap();
                } else {
                    let res = reqwest::Client::new().post(url_login).json(&user).send().await.unwrap();
                    if res.status() == StatusCode::OK {
                        let token = res.text().await.unwrap();
                        tx.send(InnerMsg::MsgUserLogined(token)).await.unwrap();
                    }
                }
            }
        });
    }

    fn load_dashboard_config(&mut self) {
        if let Some(game_path) = std::env::current_exe().unwrap().parent() {
            let conf_path = game_path.join("Plugins").join("RBNHelper").join("RBRDashBoard.ini");
            if let Ok(conf) = Ini::load_from_file(conf_path) {
                let enable_leader = conf.get_from_or(Some("Setting"), "LeaderEnable", "false");
                match enable_leader {
                    "true" => {
                        unsafe {
                            RBR_EnableLeaderBoard();
                            RBR_CfgLeaderBoardPos(
                            conf.get_from_or(Some("Pos"), "LeaderBoardPosX", "20").parse().unwrap(),
                            conf.get_from_or(Some("Pos"), "LeaderBoardPosY", "100").parse().unwrap()
                            );
                            RBR_CfgLeaderBoardStyle(
                                conf.get_from_or(Some("Color"), "LeaderBriefColor", "0xFFFF00FF").as_ptr() as *const c_char,
                                conf.get_from_or(Some("Color"), "LeaderBackGroundColor", "0xFFFFFF1F").as_ptr() as *const c_char,
                            );
                        };
                    },
                    _ => {},
                };
                let enable_progress = conf.get_from_or(Some("Setting"), "ProgressEnable", "false");
                match enable_progress {
                    "true" => {
                        unsafe {
                            RBR_EnableProgressBar();
                            RBR_CfgProgressBarPos(
                            conf.get_from_or(Some("Pos"), "ProgressBarPosX", "40").parse().unwrap(),
                            conf.get_from_or(Some("Pos"), "ProgressBarPosY", "300").parse().unwrap()
                            );
                            RBR_CfgProgressBarStyle(
                                conf.get_from_or(Some("Color"), "ProgressBarBackColor", "0xFFFFFFFF").as_ptr() as *const c_char,
                                conf.get_from_or(Some("Color"), "ProgressBarSplitColor", "0x00FF00FF").as_ptr() as *const c_char,
                                conf.get_from_or(Some("Color"), "ProgressBarPointerColor", "0x00FF00FF").as_ptr() as *const c_char,
                            );
                        };
                    }
                    _ => {},
                }

                if enable_leader.eq("true") || enable_progress.eq("true") {
                    unsafe {
                        RBR_CfgProfileStyle(
                            conf.get_from_or(Some("Color"), "UserColor1", "0xFF0000FF").as_ptr() as *const c_char,
                            conf.get_from_or(Some("Color"), "UserColor2", "0x00FF00FF").as_ptr() as *const c_char,
                        );
                    }
                }
            }
        }
    }

    pub fn async_message_handle(&mut self) {
        if let Ok(msg) = self.rx.try_recv() {
            match msg {
                InnerMsg::MsgSetErrState(err) => {
                    self.message = err;
                },
                InnerMsg::MsgUserLogined(token) => {
                    self.store.user_token = token;
                }
            }
        }
    }

    pub fn draw_on_end_frame(&mut self) {
        self.async_message_handle();
        //unsafe {RBR_ShowText(50.0, 200.0, self.copyright.as_ptr() as *const c_char)};
    }
}
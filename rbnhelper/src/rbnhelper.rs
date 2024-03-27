use libc::c_char;
use log::info;
use rbnproto::httpapi::{RaceInfo, RaceQuery, UserLogin};
use rbnproto::metaapi::{RaceJoin, RaceLeave};
use rbnproto::API_VERSION_STRING;
use reqwest::StatusCode;
use crate::backend::{RBNBackend, TaskMsg};
use crate::components::player::OggPlayer;
use crate::game::plugin::IPlugin;
use crate::game::hacker::*;
use crate::game::rbr::RBRGame;
use ini::Ini;
use crate::components::store::RacingStore;
use tokio::sync::mpsc::{channel, Receiver, Sender};

enum InnerMsg {
    MsgUserLogined(String),
    MsgMenuMessage(String),
    MsgInGameMessage(String),
    MsgErrorMessage(String),
    MsgSetRaceInfo(RaceInfo),
}

pub struct RBNHelper {
    backend: RBNBackend,
    rx: Receiver<InnerMsg>,
    tx: Sender<InnerMsg>,
    store: RacingStore,
    rsf_menu: i32,
    menu_message: String,
    ingame_message: String,
    error_message: String,
    copyright: String,
}

impl Default for RBNHelper {
    fn default() -> Self {
        let (tx, rx) = channel::<InnerMsg>(64);
        Self {
            backend: RBNBackend::default(),
            rx,
            tx,
            store: RacingStore::default(),
            rsf_menu: 0,
            menu_message: String::new(),
            ingame_message: String::new(),
            error_message: String::new(),
            copyright: format!("Welcome to use RBN Helper [{}], Copyright Lw_Ziye 2023-2024.", std::env!("CARGO_PKG_VERSION")),
        }
    }
}

impl IPlugin for RBNHelper {
    fn GetName(&mut self) -> *const libc::c_char {
        let name = std::ffi::CString::new("RBN Helper").unwrap();
        name.into_raw()
    }
}

impl RBNHelper {
    pub fn init(&mut self) {
        self.store.init();
        self.load_dashboard_config();
        self.check_and_login();
    }

    pub fn is_logined(&self) -> bool {
        !self.store.user_token.is_empty()
    }

    fn check_and_login(&mut self) {
        let url_ver = self.store.get_http_url("api/version");
        let url_login = self.store.get_http_url("api/user/login");
        let user = UserLogin{name: self.store.user_name.clone(), passwd: self.store.user_passwd.clone()};
        let tx = self.tx.clone();
        tokio::runtime::Runtime::new().unwrap().block_on(async move {
            let res = reqwest::get(url_ver).await.unwrap();
            if res.status() == StatusCode::OK {
                let version = res.text().await.unwrap();
                if version != API_VERSION_STRING {
                    tx.send(InnerMsg::MsgErrorMessage("Version is out of date, please upgrade!".to_string())).await.unwrap();
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
                InnerMsg::MsgUserLogined(token) => {
                    info!("User Logined RBN Server [{}] success.", self.store.server_addr);
                    self.store.user_token = token;
                    let (tx, rx) = channel::<TaskMsg>(16);
                    self.backend.init(&self.store);
                    self.backend.run(tx, rx);
                }
                InnerMsg::MsgMenuMessage(msg) => {
                    self.menu_message = msg;
                }
                InnerMsg::MsgInGameMessage(msg) => {
                    self.ingame_message = msg;
                }
                InnerMsg::MsgErrorMessage(msg) => {
                    self.error_message = msg;
                }
                InnerMsg::MsgSetRaceInfo(raceinfo) => {

                    //unsafe { RBR_SetPractice(RBRRaceSetting::from(&raceinfo, &RaceConfig::default())); }
                    // RBRGame::default().cfg_practice(&raceinfo);
                }
            }
        }
    }

    pub fn draw_on_end_frame(&mut self) {
        self.async_message_handle();
        
        
        if !self.menu_message.is_empty() {
            unsafe {RBR_DrawTextOverRsfMain(240,600, 0xFFFFFFFF, self.menu_message.as_ptr() as *const c_char)};
        }
        
        unsafe {RBR_DrawTextAnyway(240, 640, 0xFFFFFFFF, self.copyright.as_ptr() as *const c_char)};
    }

    pub fn on_rsf_menu_changed(&mut self, menu: i32) {
        if self.rsf_menu == 0 && menu == 2 {
            info!("Enter Hotlap Menu from Main Rsf Menu.");
        }

        if self.rsf_menu == 2 && menu == 0 {
            info!("Exit to main rsf menu from hotlap.");
        }

        if self.rsf_menu == 0 && menu == 3 {
            info!("Enter Practice Menu from Main Rsf Menu.");
            self.join_race(&"Daily Challenge".to_string());
            self.fetch_race_info(&"Daily Challenge".to_string());
            self.backend.trigger(TaskMsg::MsgStartStage("Daily Challenge".to_string()));
        }

        if self.rsf_menu == 3 && menu == 0 {
            info!("Exit to Main Rsf Menu from practice.");
            self.leave_race(&"Daily Challenge".to_string());
            self.backend.trigger(TaskMsg::MsgStopStage);
        }

        self.fetch_race_brief();
        self.rsf_menu = menu;
    }

    pub fn fetch_race_brief(&mut self) {
        if self.is_logined() {
            let url = self.store.get_http_url("api/race/brief");
            let tx = self.tx.clone();
            tokio::runtime::Runtime::new().unwrap().block_on(async move {
                let res = reqwest::Client::new().get(&url).send().await.unwrap();
                if res.status() == StatusCode::OK {
                    let brief = res.text().await.unwrap();
                    tx.send(InnerMsg::MsgMenuMessage(brief)).await.unwrap();
                }
            });
        }
    }

    // need to call by hooking hotlap and practice menu in.
    pub fn join_race(&mut self, race: &String) {
        if self.is_logined() {
            let race_join = RaceJoin {token: self.store.user_token.clone(), room: race.clone(), passwd: None};
            let url = self.store.get_http_url("api/race/join");
            let tx = self.tx.clone();
            tokio::runtime::Runtime::new().unwrap().block_on(async move {
                let res = reqwest::Client::new().post(url).json(&race_join).send().await.unwrap();
                match res.status() {
                    StatusCode::OK => {
                        tx.send(InnerMsg::MsgMenuMessage("Joined Race".to_string())).await.unwrap();
                        OggPlayer::open("join.ogg").play();
                    }
                    _ => {
                        tx.send(InnerMsg::MsgErrorMessage("Failed Join Race".to_string())).await.unwrap();
                    }
                }
            });
        }
    }

    // need to call by hooking exit hotlap and practice menu.
    pub fn leave_race(&mut self, race: &String) {
        if self.is_logined() {
            let user: RaceLeave = RaceLeave{ token: self.store.user_token.clone(), room: race.clone() };
            let url = self.store.get_http_url("api/race/leave");
            tokio::runtime::Runtime::new().unwrap().block_on(async move {
                let _res = reqwest::Client::new().post(url).json(&user).send().await.unwrap();
                OggPlayer::open("exit.ogg").play();
            });
        }
    }

    // need to call by hooking hotlap and practice menu in.
    pub fn fetch_race_info(&mut self, race: &String) {
        if self.is_logined() {
            let info_url = self.store.get_http_url("api/race/info");
            let info_tx = self.tx.clone();
            let info_query = RaceQuery {name: race.clone()};
            tokio::runtime::Runtime::new().unwrap().block_on(async move {
                let res = reqwest::Client::new().get(&info_url).json(&info_query).send().await.unwrap();
                if res.status() == StatusCode::OK {
                    let text = res.text().await.unwrap();
                    let raceinfo: RaceInfo = serde_json::from_str(text.as_str()).unwrap();
                    let mut rbr = RBRGame::default();
                    rbr.fast_set_race_stage(&raceinfo.stage_id);
                    info_tx.send(InnerMsg::MsgSetRaceInfo(raceinfo)).await.unwrap();
                }
            });
        }
    }
}
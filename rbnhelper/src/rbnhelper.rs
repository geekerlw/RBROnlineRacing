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
use crate::components::store::RacingStore;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub enum InnerMsg {
    MsgUserLogined(String),
    MsgUserJoined,
    MsgSetRaceInfo(RaceInfo),
    MsgLoadStage,
    MsgStartStage,
}

pub struct RBNHelper {
    backend: RBNBackend,
    rx: Receiver<InnerMsg>,
    tx: Sender<InnerMsg>,
    store: RacingStore,
    rsf_menu: i32,
    race_name: String,
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
            race_name: String::from("Daily Challenge"),
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
                if version == API_VERSION_STRING {
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
            RBRGame::default().cfg_dashboard_style(conf_path);
        }
    }

    pub fn async_message_handle(&mut self) {
        if let Ok(msg) = self.rx.try_recv() {
            match msg {
                InnerMsg::MsgUserLogined(token) => {
                    info!("User Logined RBN Server [{}] success.", self.store.server_addr);
                    self.store.user_token = token;
                    let (tx, rx) = channel::<TaskMsg>(16);
                    self.backend.init(&self.store, &self.tx);
                    self.backend.run(tx, rx);
                }
                InnerMsg::MsgUserJoined => {
                    self.fetch_race_info();
                }
                InnerMsg::MsgSetRaceInfo(raceinfo) => {
                    if self.rsf_menu == 2 {
                        RBRGame::default().cfg_hotlap(&raceinfo);
                    }
                    else if self.rsf_menu == 3 {
                        RBRGame::default().cfg_practice(&raceinfo);
                    }
                }
                InnerMsg::MsgLoadStage => {
                    RBRGame::default().load();
                }
                InnerMsg::MsgStartStage => {
                    RBRGame::default().start();
                }
            }
        }
    }

    pub fn draw_on_end_frame(&mut self) {
        self.async_message_handle();
        
        unsafe {RBR_DrawTextOverRsfMain(240, 640, 0xFFFFFFFF, self.copyright.as_ptr() as *const c_char)};
    }

    pub fn on_rsf_menu_changed(&mut self, menu: i32) {
        let last_menu = self.rsf_menu;
        self.rsf_menu = menu;
        self.fetch_race_brief();

        if last_menu == 0 && menu == 2 {
            info!("Enter Hotlap Menu from Main Rsf Menu.");
        }

        if last_menu == 2 && menu == 0 {
            info!("Exit to main rsf menu from hotlap.");
        }

        if last_menu == 0 && menu == 3 {
            info!("Enter Practice Menu from Main Rsf Menu.");
            self.join_race(&"Daily Challenge".to_string());
            self.backend.trigger(TaskMsg::MsgStartStage("Daily Challenge".to_string()));
        }

        if last_menu == 3 && menu == 0 {
            info!("Exit to Main Rsf Menu from practice.");
            self.leave_race(&"Daily Challenge".to_string());
            self.backend.trigger(TaskMsg::MsgStopStage);
        }
    }

    pub fn fetch_race_brief(&mut self) {
        if self.is_logined() {
            let url = self.store.get_http_url("api/race/brief");
            tokio::runtime::Runtime::new().unwrap().block_on(async move {
                let res = reqwest::Client::new().get(&url).send().await.unwrap();
                if res.status() == StatusCode::OK {
                    let _brief = res.text().await.unwrap();
                    //tx.send(InnerMsg::MsgMenuMessage(brief)).await.unwrap();
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
                if res.status() == StatusCode::OK {
                    tx.send(InnerMsg::MsgUserJoined).await.unwrap();
                    OggPlayer::open("join.ogg").play();
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
                reqwest::Client::new().post(url).json(&user).send().await.unwrap();
                OggPlayer::open("exit.ogg").play();
            });
        }
    }

    // need to call by hooking hotlap and practice menu in.
    pub fn fetch_race_info(&mut self) {
        if self.is_logined() {
            let info_url = self.store.get_http_url("api/race/info");
            let info_tx = self.tx.clone();
            let info_query = RaceQuery {name: self.race_name.clone()};
            tokio::runtime::Runtime::new().unwrap().block_on(async move {
                let res = reqwest::Client::new().get(&info_url).json(&info_query).send().await.unwrap();
                if res.status() == StatusCode::OK {
                    let text = res.text().await.unwrap();
                    let raceinfo: RaceInfo = serde_json::from_str(text.as_str()).unwrap();
                    RBRGame::default().fast_set_race_stage(&raceinfo.stage_id);
                    info_tx.send(InnerMsg::MsgSetRaceInfo(raceinfo)).await.unwrap();
                }
            });
        }
    }
}
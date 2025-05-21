use std::sync::{Arc, RwLock};
use std::vec;
use log::info;
use rbnproto::httpapi::{UserHeart, UserLogin, UserQuery, UserScore};
use rbnproto::metaapi::{MetaRaceProgress, MetaRaceResult, MetaRaceState};
use rbnproto::API_VERSION_STRING;
use rbrproxy::plugin::IPlugin;
use rbrproxy::rbrproxy_env_init;
use reqwest::StatusCode;
use simplelog::WriteLogger;
use tokio::time::Instant;
use crate::components::backend::{RBNBackend, TaskMsg};
use crate::components::store::RacingStore;
use crate::menu::Menu;
use crate::overlay::leaderboard::LeaderBoard;
use crate::overlay::progressbar::ProgressBar;
use crate::overlay::Overlay;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use crate::menu::loby::LobyMenu;

pub enum InnerMsg {
    MsgUserLogined(String),
    MsgTriggerBackendStart,
    MsgTriggerBackendend,
    MsgUpdateNews(String),
    MsgUpdateScore(UserScore),
    MsgUpdateNotice(String),
    MsgUpdateRaceState(Vec<MetaRaceState>),
    MsgUpdateRaceData(Vec<MetaRaceProgress>),
    MsgUpdateRaceResult(Vec<MetaRaceResult>),
}

pub struct RBNHelper {
    backend: RBNBackend,
    rx: Receiver<InnerMsg>,
    tx: Sender<InnerMsg>,
    store: Arc::<RwLock<RacingStore>>,
    overlays: Vec<Box<dyn Overlay + Send + Sync>>,
    menu: LobyMenu,
}

impl Default for RBNHelper {
    fn default() -> Self {
        let (tx, rx) = channel::<InnerMsg>(64);
        Self {
            backend: RBNBackend::default(),
            rx,
            tx,
            store: Arc::new(RwLock::new(RacingStore::default())),
            overlays: vec![],
            menu: LobyMenu::default(),
        }
    }
}

impl IPlugin for RBNHelper {
    fn plugin_init(&mut self) -> *const libc::c_char {
        self.init();
        let name = std::ffi::CString::new("RBNHelper").unwrap();
        name.into_raw()
    }

    fn plugin_draw_menu(&mut self) {
        self.menu.draw();
    }

    fn plugin_handle_input(&mut self, _txt: libc::c_char, up: bool, down: bool, left: bool, right: bool, select: bool) {
        if up {
            self.menu.up();
        }
        if down {
            self.menu.down();
        }
        if left {
            self.menu.left();
        }
        if right {
            self.menu.right();
        }
        if select {
            self.menu.select();
        }
    }

    fn plugin_on_end_scene(&mut self) {
        self.async_message_handle();
        self.draw_overlays();
    }

    fn plugin_on_stage_start(&mut self, _mapid: i32, _player: *const libc::c_char, _falsestart: bool) {
        self.overlays.push(Box::new(LeaderBoard::default()));
        self.overlays.push(Box::new(ProgressBar::default()));
        for overlay in self.overlays.iter_mut() {
            overlay.init();
        }
    }

    fn plugin_on_stage_end(&mut self, _checkpoint1: f32, _checkpoint2: f32, _finishtime: f32, _player: *const libc::c_char) {
        self.overlays.clear();
    }
}

impl RBNHelper {
    pub fn init(&mut self) {
        self.env_init();
        self.store_init();
        self.check_and_login();
        self.menu.init(Arc::clone(&self.store));
    }

    pub fn is_logined(&self) -> bool {
        let store = self.store.read().unwrap();
        !store.user_token.is_empty()
    }

    pub fn draw_overlays(&mut self) {
        self.overlays.iter_mut().for_each(|x| {
            let store = self.store.read().unwrap();
            x.draw(&store);
        });
    }

    fn env_init(&mut self) {
        rbrproxy_env_init();
        if let Some(game_path) = std::env::current_exe().unwrap().parent() {
            let log_file = game_path.join("Plugins")
            .join("RBNHelper")
            .join("RBNHelper.log");
            WriteLogger::init(log::LevelFilter::Info, 
                simplelog::Config::default(), std::fs::File::create(log_file).unwrap()).unwrap();
        }
    }

    fn store_init(&mut self) {
        let mut store = self.store.write().unwrap();
        store.init();
    }

    fn check_and_login(&mut self) {
        let store = self.store.read().unwrap();

        let url_ver = store.get_http_url("api/version");
        let url_login = store.get_http_url("api/user/login");
        let user = UserLogin{name: store.user_name.clone(), passwd: store.user_passwd.clone()};
        let tx = self.tx.clone();
        tokio::runtime::Runtime::new().unwrap().block_on(async move {
            let res = reqwest::get(url_ver).await;
            if let Ok(res) = res {
                if res.status() == StatusCode::OK {
                    let version = res.text().await.unwrap();
                    if version == API_VERSION_STRING {
                        let res = reqwest::Client::new().post(url_login).json(&user).send().await.unwrap();
                        if res.status() == StatusCode::OK {
                            let token = res.text().await.unwrap();
                            tx.send(InnerMsg::MsgUserLogined(token)).await.unwrap();
                        }
                    } else {
                        tx.send(InnerMsg::MsgUpdateNews("Version is out of date, need to update.".to_string())).await.unwrap();
                    }
                }
            }
        });
    }

    pub fn async_message_handle(&mut self) {
        if let Ok(msg) = self.rx.try_recv() {
            let mut store = self.store.write().unwrap();

            match msg {
                InnerMsg::MsgUserLogined(token) => {
                    info!("User Logined RBN Server [{}] success.", store.get_http_uri());
                    store.user_token = token;
                    let (tx, rx) = channel::<TaskMsg>(16);
                    self.backend.init(&store);
                    self.backend.run(tx, rx, &self.tx);
                    self.keep_alive();
                }
                InnerMsg::MsgTriggerBackendStart => {
                    self.backend.trigger(TaskMsg::MsgStartStage(store.room_name.clone()));
                }
                InnerMsg::MsgTriggerBackendend => {
                    self.backend.trigger(TaskMsg::MsgStopStage);
                }
                InnerMsg::MsgUpdateNews(news) => {
                    store.brief_news = news;
                }
                InnerMsg::MsgUpdateScore(score) => {
                    store.scoreinfo = score;
                }
                InnerMsg::MsgUpdateNotice(notice) => {
                    store.noticeinfo = notice;
                }
                InnerMsg::MsgUpdateRaceState(state) => {
                    store.racestate = state;
                }
                InnerMsg::MsgUpdateRaceData(progress) => {
                    store.racedata = progress;
                }
                InnerMsg::MsgUpdateRaceResult(result) => {
                    store.raceresult = result;
                }
            }
        }
    }


    pub fn fetch_race_news(&self) {
        if !self.is_logined() {
            return;
        }
        let store = self.store.read().unwrap();

        let url = store.get_http_url("api/race/news");
        let tx = self.tx.clone();
        tokio::runtime::Runtime::new().unwrap().block_on(async move {
            let res = reqwest::Client::new().get(&url).send().await;
            if let Ok(res) = res {
                if res.status() == StatusCode::OK {
                    let news = res.text().await.unwrap();
                    tx.send(InnerMsg::MsgUpdateNews(news)).await.unwrap();
                }
            }
        });
    }

    pub fn fetch_user_score(&self) {
        if !self.is_logined() {
            return;
        }
        let store = self.store.read().unwrap();
        let url = store.get_http_url("api/user/score");
        let query = UserQuery { token: store.user_token.clone() };
        let tx = self.tx.clone();
        tokio::runtime::Runtime::new().unwrap().block_on(async move {
            let res = reqwest::Client::new().get(&url).json(&query).send().await;
            if let Ok(res) = res {
                if res.status() == StatusCode::OK {
                    let text = res.text().await.unwrap();
                    let userscore: UserScore = serde_json::from_str(&text).unwrap();
                    tx.send(InnerMsg::MsgUpdateScore(userscore)).await.unwrap();
                }
            }
        });
    }

    pub fn keep_alive(&self) {
        if !self.is_logined() {
            return;
        }
        let store = self.store.read().unwrap();
        let url = store.get_http_url("api/user/heartbeat");
        let user = UserHeart { token: store.user_token.clone() };
        std::thread::spawn(move || {
            tokio::runtime::Runtime::new().unwrap().block_on(async move {
                loop {
                    let _res = reqwest::Client::new().post(&url).json(&user).send().await;
                    tokio::time::sleep_until(Instant::now() + tokio::time::Duration::from_secs(10)).await;
                }
            });
        });
    }
}
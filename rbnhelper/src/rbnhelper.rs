use std::vec;
use log::info;
use rbnproto::httpapi::{UserHeart, UserLogin, UserQuery, UserScore};
use rbnproto::metaapi::{MetaRaceProgress, MetaRaceResult, MetaRaceState, RaceJoin, RaceLeave};
use rbnproto::API_VERSION_STRING;
use rbrproxy::plugin::IPlugin;
use rbrproxy::rbrproxy_env_init;
use reqwest::StatusCode;
use simplelog::WriteLogger;
use tokio::time::Instant;
use crate::backend::{RBNBackend, TaskMsg};
use crate::components::player::AudioPlayer;
use crate::components::store::RacingStore;
use crate::menu::Menu;
use crate::overlay::leaderboard::LeaderBoard;
use crate::overlay::progressbar::ProgressBar;
use crate::overlay::Overlay;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use crate::menu::loby::LobyMenu;

pub enum InnerMsg {
    MsgUserLogined(String),
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
    store: RacingStore,
    race_name: String,
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
            store: RacingStore::default(),
            race_name: String::from("Daily Challenge"),
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

    fn plugin_handle_input(&mut self, txt: libc::c_char, up: bool, down: bool, left: bool, right: bool, select: bool) {
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
        self.store.init();
        self.check_and_login();
        self.menu.init();
    }

    pub fn is_logined(&self) -> bool {
        !self.store.user_token.is_empty()
    }

    pub fn draw_overlays(&mut self) {
        self.overlays.iter_mut().for_each(|x| {
            x.draw(&self.store);
        });
    }

    fn env_init(&mut self) {
        rbrproxy_env_init();
        if let Some(game_path) = std::env::current_exe().unwrap().parent() {
            let log_file = game_path.join("SimrallyCN").join("rbnhelper.log");
            WriteLogger::init(log::LevelFilter::Info, 
                simplelog::Config::default(), std::fs::File::create(log_file).unwrap()).unwrap();
        }
    }

    fn check_and_login(&mut self) {
        let url_ver = self.store.get_http_url("api/version");
        let url_login = self.store.get_http_url("api/user/login");
        let user = UserLogin{name: self.store.user_name.clone(), passwd: self.store.user_passwd.clone()};
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
            match msg {
                InnerMsg::MsgUserLogined(token) => {
                    info!("User Logined RBN Server [{}] success.", self.store.get_http_uri());
                    self.store.user_token = token;
                    let (tx, rx) = channel::<TaskMsg>(16);
                    self.backend.init(&self.store);
                    self.backend.run(tx, rx, &self.tx);
                    self.keep_alive();
                }
                InnerMsg::MsgUpdateNews(news) => {
                    self.store.brief_news = news;
                }
                InnerMsg::MsgUpdateScore(score) => {
                    self.store.scoreinfo = score;
                }
                InnerMsg::MsgUpdateNotice(notice) => {
                    self.store.noticeinfo = notice;
                }
                InnerMsg::MsgUpdateRaceState(state) => {
                    self.store.racestate = state;
                }
                InnerMsg::MsgUpdateRaceData(progress) => {
                    self.store.racedata = progress;
                }
                InnerMsg::MsgUpdateRaceResult(result) => {
                    self.store.raceresult = result;
                }
            }
        }
    }

    // need to call by hooking hotlap and practice menu in.
    pub fn join_race(&mut self, race: &String) -> bool {
        if self.is_logined() {
            let race_join = RaceJoin {token: self.store.user_token.clone(), room: race.clone(), passwd: None};
            let join_url = self.store.get_http_url("api/race/join");
            return tokio::runtime::Runtime::new().unwrap().block_on(async move {
                let res = reqwest::Client::new().post(join_url).json(&race_join).send().await;
                if let Ok(res) = res {
                    match res.status() {
                        StatusCode::OK => {
                            AudioPlayer::notification("join.wav").play();
                            return true;
                        }
                        _ => {
                            AudioPlayer::notification("join_failed.wav").play();
                            return false;
                        }
                    }
                }
                return false;
            });
        }
        false
    }

    // need to call by hooking exit hotlap and practice menu.
    pub fn leave_race(&mut self, race: &String) -> bool {
        if self.is_logined() {
            let user: RaceLeave = RaceLeave{ token: self.store.user_token.clone(), room: race.clone() };
            let url = self.store.get_http_url("api/race/leave");
            tokio::runtime::Runtime::new().unwrap().block_on(async move {
                let res = reqwest::Client::new().post(url).json(&user).send().await;
                if let Ok(res) = res {
                    if res.status() == StatusCode::OK {
                        AudioPlayer::notification("exit.wav").play();
                        return true;
                    }
                }
                return false;
            });
        }
        false
    }

    pub fn fetch_race_news(&mut self) {
        if self.is_logined() {
            let url = self.store.get_http_url("api/race/news");
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
    }

    pub fn fetch_user_score(&mut self) {
        if self.is_logined() {
            let url = self.store.get_http_url("api/user/score");
            let query = UserQuery { token: self.store.user_token.clone() };
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
    }

    pub fn keep_alive(&mut self) {
        if self.is_logined() {
            let url = self.store.get_http_url("api/user/heartbeat");
            let user = UserHeart { token: self.store.user_token.clone() };
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
}
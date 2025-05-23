use std::vec;
use log::info;
use rbnproto::httpapi::{RaceInfo, RaceQuery, UserHeart, UserLogin, UserQuery, UserScore};
use rbnproto::metaapi::{RaceJoin, RaceLeave};
use rbnproto::API_VERSION_STRING;
use reqwest::StatusCode;
use tokio::time::Instant;
use crate::backend::{RBNBackend, TaskMsg};
use crate::components::player::AudioPlayer;
use crate::game::plugin::IPlugin;
use crate::game::hacker::*;
use crate::game::rbr::RBRGame;
use crate::components::store::RacingStore;
use crate::overlay::copyright::CopyRight;
use crate::overlay::news::RaceNews;
use crate::overlay::notice::RaceNotice;
use crate::overlay::scoreboard::ScoreBoard;
use crate::overlay::Overlay;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub enum InnerMsg {
    MsgUserLogined(String),
    MsgUpdateNews(String),
    MsgUpdateScore(UserScore),
    MsgUpdateNotice(String),
}

pub struct RBNHelper {
    backend: RBNBackend,
    rx: Receiver<InnerMsg>,
    tx: Sender<InnerMsg>,
    store: RacingStore,
    rsf_menu: i32,
    race_name: String,
    overlays: Vec<Box<dyn Overlay + Send + Sync>>,
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
            overlays: vec![],
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
        self.init_overlays();
    }

    pub fn is_logined(&self) -> bool {
        !self.store.user_token.is_empty()
    }

    pub fn is_autojoin(&self) -> bool {
        self.store.autojoin
    }

    pub fn init_overlays(&mut self) {
        self.overlays.push(Box::new(CopyRight::default()));
        self.overlays.push(Box::new(ScoreBoard::default()));
        self.overlays.push(Box::new(RaceNews::default()));
        self.overlays.push(Box::new(RaceNotice::default()));

        let window_width = unsafe { RBR_GetD3dWindowWidth() };
        let window_height = unsafe { RBR_GetD3dWindowHeight() };
        self.overlays.iter_mut().for_each(|x| x.init(window_width, window_height));
    }

    pub fn draw_overlays(&mut self) {
        self.overlays.iter_mut().for_each(|x| {
            x.draw_ui(&self.store);
        });
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

    fn load_dashboard_config(&mut self) {
        if let Some(game_path) = std::env::current_exe().unwrap().parent() {
            let conf_path = game_path.join("Plugins").join("RBNHelper").join("RBNHelper.ini");
            RBRGame::default().cfg_dashboard_style(conf_path);
        }
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
            }
        }
    }

    pub fn draw_on_end_frame(&mut self) {
        self.async_message_handle();
        self.draw_overlays();
    }

    pub fn on_game_mode_changed(&mut self) {
        let mut rbr = RBRGame::default();

        if rbr.game_mode() == 0x05 { // clear notice info when game start load.
            self.store.noticeinfo.clear();
        }
    }

    pub fn on_rsf_menu_changed(&mut self, menu: i32) {
        let last_menu = self.rsf_menu;
        self.rsf_menu = menu;
        self.fetch_race_news();
        self.fetch_user_score();

        if last_menu == 0 && menu == 2 {
            self.race_name = "Time Trial".to_string();
            if self.join_race(&self.race_name.clone()) {
                self.backend.trigger(TaskMsg::MsgStartStage(self.race_name.clone()));
            }
        }

        if last_menu == 0 && menu == 3 {
            self.race_name = "Practice".to_string();
            if self.join_race(&self.race_name.clone()) {
                self.backend.trigger(TaskMsg::MsgStartStage(self.race_name.clone()));
            }
        }

        if (last_menu == 2 || last_menu == 3) && menu == 0 {
            if self.leave_race(&self.race_name.clone()) {
                self.backend.trigger(TaskMsg::MsgStopStage);
            }
        }
    }

    // need to call by hooking hotlap and practice menu in.
    pub fn join_race(&mut self, race: &String) -> bool {
        if self.is_logined() && self.is_autojoin() {
            let race_join = RaceJoin {token: self.store.user_token.clone(), room: race.clone(), passwd: None};
            let join_url = self.store.get_http_url("api/race/join");
            let info_url = self.store.get_http_url("api/race/info");
            let info_query = RaceQuery {name: race.clone()};
            return tokio::runtime::Runtime::new().unwrap().block_on(async move {
                let res = reqwest::Client::new().post(join_url).json(&race_join).send().await;
                if let Ok(res) = res {
                    match res.status() {
                        StatusCode::OK => {
                            let res = reqwest::Client::new().get(&info_url).json(&info_query).send().await.unwrap();
                            if res.status() == StatusCode::OK {
                                let text = res.text().await.unwrap();
                                let raceinfo: RaceInfo = serde_json::from_str(text.as_str()).unwrap();
                                RBRGame::default().fast_set_race_stage(&raceinfo.stage_id);
                                RBRGame::default().fast_set_race_car_damage(&raceinfo.damage);
                                AudioPlayer::notification("join.wav").play();
                                return true;
                            };
                            return false;
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
        if self.is_logined() && self.is_autojoin() {
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
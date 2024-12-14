use ini::Ini;
use log::info;
use rbnproto::httpapi::UserScore;

use crate::game::rbr::RBRGame;

#[derive(Default, Clone)]
pub struct RacingStore {
    server_addr: String,
    server_port: u16,
    meta_port: u16,
    pub autojoin: bool,
    pub user_name: String,
    pub user_passwd: String,
    pub user_token: String,
    pub brief_news: String,
    pub noticeinfo: String,
    pub scoreinfo: UserScore,
}

impl RacingStore {
    pub fn init(&mut self) {
        if let Some(game_root) = std::env::current_exe().unwrap().parent() {
            let conf_path = game_root.join("Plugins").join("RBNHelper").as_path().to_owned();
            if !conf_path.exists() {
                std::fs::create_dir(conf_path).unwrap();
            }
        }
        self.load_config();
    }

    pub fn load_config(&mut self) {
        self.user_name = RBRGame::default().get_user().to_string();
        self.user_passwd = String::from("simrallycn");
        info!("Parsed game user [{}] success", self.user_name);

        self.scoreinfo.license = "Rookie".to_string();
        self.scoreinfo.score = 0;

        if let Some(game_path) = std::env::current_exe().unwrap().parent() {
            let conf_file = game_path.join("Plugins").join("RBNHelper").join("RBNHelper.ini");
            if let Ok(conf) = Ini::load_from_file(&conf_file) {
                self.autojoin = conf.get_from_or(Some("Setting"), "AutoJoinRace", "true").parse().unwrap();
                self.server_addr = conf.get_from_or(Some("Server"), "Host", "127.0.0.1").parse().unwrap();
                self.server_port = conf.get_from_or(Some("Server"), "HttpPort", 23555).parse().unwrap();
                self.meta_port = conf.get_from_or(Some("Server"), "DataPort", 23556).parse().unwrap();
            }
        }

        if cfg!(debug_assertions) {
            self.server_addr = String::from("127.0.0.1");
        }
    }

    #[allow(dead_code)]
    pub fn save_config(&mut self) {
        if let Some(game_path) = std::env::current_exe().unwrap().parent() {
            let conf_file = game_path.join("Plugins").join("RBNHelper").join("RBNHelper.ini");
            if let Ok(mut conf) = Ini::load_from_file(&conf_file) {
                conf.with_section(Some("Setting")).set("AutoJoinRace", self.autojoin.to_string());
                conf.write_to_file(conf_file).unwrap();
            }
        }
    }

    pub fn get_http_uri(&self) -> String {
        let uri = "http://".to_string()
            + self.server_addr.as_str()
            + ":"
            + self.server_port.to_string().as_str()
            + "/";
        uri
    }    

    pub fn get_http_url(&self, uri: &str) -> String {
        let url = "http://".to_string()
            + self.server_addr.as_str()
            + ":"
            + self.server_port.to_string().as_str()
            + "/"
            + uri;
        url
    }

    pub fn get_meta_url(&self) -> String {
        let addr = String::from(&self.server_addr) + ":" + self.meta_port.to_string().as_str();
        addr
    }
}
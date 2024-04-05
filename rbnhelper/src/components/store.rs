use std::env;
use log::info;

use crate::game::rbr::RBRGame;

#[derive(Default, Clone)]
pub struct RacingStore {
    server_addr: String,
    server_port: u16,
    meta_port: u16,
    pub user_name: String,
    pub user_passwd: String,
    pub user_token: String,
    pub brief_news: String,
}

impl RacingStore {
    pub fn init(&mut self) {
        if let Some(game_root) = env::current_exe().unwrap().parent() {
            let conf_path = game_root.join("Plugins").join("RBNHelper").as_path().to_owned();
            if !conf_path.exists() {
                std::fs::create_dir(conf_path).unwrap();
            }
        }
        self.load_config();
    }

    pub fn load_config(&mut self) {
        if cfg!(debug_assertions) {
            self.server_addr = String::from("127.0.0.1");
        } else {
            self.server_addr = String::from("8.137.36.254");
        }
        self.server_port = 23555;
        self.meta_port = 23556;

        self.user_name = RBRGame::default().get_user().to_string();
        self.user_passwd = String::from("simrallycn");

        info!("Parsed game user [{}] success", self.user_name);
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
use std::env;
use ini::Ini;
use log::info;

use crate::game::rbr::RBRGame;

#[derive(Default, Clone)]
pub struct RacingStore {
    pub server_addr: String,
    pub server_port: u16,
    pub meta_port: u16,
    pub user_name: String,
    pub user_passwd: String,
    pub user_token: String,
    pub user_state: String,
    pub curr_room: String,
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
        if let Some(game_root) = env::current_exe().unwrap().parent() {
            let conf_file = game_root.join("Plugins").join("RBNHelper").join("rbnhelper.ini");
            if let Ok(conf) = Ini::load_from_file(conf_file) {
                self.server_addr = conf.get_from_or(Some("server"), "address", "127.0.0.1").to_string();
                self.server_port = conf.get_from_or(Some("server"), "http_port", "23555").parse::<u16>().unwrap();
                self.meta_port = conf.get_from_or(Some("server"), "data_port", "23556").parse::<u16>().unwrap();

                self.user_name = RBRGame::default().get_user().to_string();
                info!("Parsed game user [{}] success", self.user_name);
                self.user_passwd = String::from("simrallycn");
            }
        }
    }

    pub fn save_config(&mut self) {
        if let Some(game_root) = env::current_exe().unwrap().parent() {
            let mut conf = Ini::new();
            conf.with_section(Some("server"))
                .set("address", &self.server_addr)
                .set("http_port", self.server_port.to_string())
                .set("data_port", self.meta_port.to_string());


            let conf_file = game_root.join("Plugins").join("RBNHelper").join("rbnhelper.ini");
            conf.write_to_file(conf_file).unwrap();
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
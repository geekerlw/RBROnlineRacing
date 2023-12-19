use std::env;
use egui::Ui;
use protocol::httpapi::RaceState;
use ini::Ini;
use std::path::Path;

#[derive(Default, Clone)]
pub struct RacingStore {
    pub server_addr: String,
    pub server_port: u16,
    pub meta_port: u16,
    pub user_name: String,
    pub user_passwd: String,
    pub user_token: String,
    pub user_state: RaceState,
    pub curr_room: String,
    pub game_path: String,
}

impl RacingStore {
    pub fn init(mut self) -> Self {
        if let Ok(appdata) = env::var("AppData") {
            let conf_path = appdata + r"\RBROnlineRacing";
            let path = Path::new(&conf_path);
            if !path.exists() {
                std::fs::create_dir(path).unwrap();
            }
        }
        self.user_name = String::from("Lw_Ziye");
        self.user_passwd = String::from("simrallycn");
        self.user_state = RaceState::RaceInit;
        self
    }

    pub fn load_config(mut self) -> Self {
        if let Ok(appdata) = env::var("AppData") {
            let conf_file = appdata + r"\RBROnlineRacing\Config.ini";
            if let Ok(conf) = Ini::load_from_file(conf_file) {
                self.server_addr = conf.get_from_or(Some("server"), "address", "127.0.0.1").to_string();
                self.server_port = conf.get_from_or(Some("server"), "http_port", "8080").parse::<u16>().unwrap();
                self.meta_port = conf.get_from_or(Some("server"), "data_port", "9493").parse::<u16>().unwrap();

                self.game_path = conf.get_from_or(Some("game"), "path", r".\").to_string();
            }
        }
        self
    }

    pub fn save_config(&mut self) {
        if let Ok(appdata) = env::var("AppData") {
            let mut conf = Ini::new();
            conf.with_section(Some("server"))
                .set("address", &self.server_addr)
                .set("http_port", self.server_port.to_string())
                .set("data_port", self.meta_port.to_string());

            conf.with_section(Some("game"))
                .set("path", &self.game_path);

            let conf_file = appdata + r"\RBROnlineRacing\Config.ini";
            conf.write_to_file(conf_file).unwrap();
        }
    }

    pub fn show_user_state(&mut self, ui: &mut Ui) {
        match &self.user_state {
            RaceState::RaceDefault => ui.label("空闲"),
            RaceState::RaceFinished => ui.label("比赛完成"),
            RaceState::RaceInit => ui.label("初始化比赛"),
            RaceState::RaceLoad => ui.label("比赛加载中"),
            RaceState::RaceLoaded => ui.label("比赛加载完成"),
            RaceState::RaceReady => ui.label("比赛就绪"),
            RaceState::RaceRetired => ui.label("比赛已放弃"),
            RaceState::RaceRunning => ui.label("比赛进行中"),
            RaceState::RaceStart => ui.label("比赛开始"),
            RaceState::RaceError(err) => ui.label(err),
        };
    }

    pub fn get_http_url(&self, uri: &str) -> String {
        let url = "http://".to_string()
            + self.server_addr.as_str()
            + ":"
            + self.server_port.to_string().as_str()
            + "/"
            + uri;
        return url;
    }

    pub fn get_meta_url(&self) -> String {
        let addr = String::from(&self.server_addr) + ":" + self.meta_port.to_string().as_str();
        return addr;
    }
}
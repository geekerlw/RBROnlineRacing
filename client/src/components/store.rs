use egui::Ui;
use protocol::httpapi::RaceState;

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
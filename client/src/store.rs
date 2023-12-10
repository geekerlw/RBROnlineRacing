use crate::UiPageState;
use protocol::httpapi::RaceState;

#[derive(Default, Clone)]
pub struct RacingStore {
    pub user_name: String,
    pub user_passwd: String,
    pub user_token: String,
    pub user_state: RaceState,
    pub prev_page: UiPageState,
    pub curr_page: UiPageState,
}

impl RacingStore {
    pub fn switch_to_page(&mut self, page: UiPageState) {
        self.prev_page = self.curr_page.clone();
        self.curr_page = page;
    }

    pub fn back_from_page(&mut self, page: UiPageState) {
        self.curr_page = self.prev_page.clone();
        self.prev_page = page;
    }
}
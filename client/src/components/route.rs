use crate::ui::UiPageState;

#[derive(Clone)]
pub struct RacingRoute {
    pub prev_page: UiPageState,
    pub curr_page: UiPageState,
}

impl Default for RacingRoute {
    fn default() -> Self {
        Self { prev_page: UiPageState::PageLogin, curr_page: UiPageState::PageLogin }
    }
}

impl RacingRoute {
    pub fn switch_to_page(&mut self, page: UiPageState) {
        self.prev_page = self.curr_page.clone();
        self.curr_page = page;
    }

    pub fn back_from_page(&mut self, page: UiPageState) {
        self.curr_page = self.prev_page.clone();
        self.prev_page = page;
    }
}
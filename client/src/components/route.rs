use crate::ui::UiPageState;

#[derive(Default, Clone)]
pub struct RacingRoute {
    pub prev_page: UiPageState,
    pub curr_page: UiPageState,
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
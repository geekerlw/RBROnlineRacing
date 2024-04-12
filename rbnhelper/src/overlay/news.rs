use std::ffi::CString;

use crate::{components::store::RacingStore, RBR_DrawTextOverRsfMain};
use super::Overlay;

pub struct RaceNews {
    posx: i16,
    posy: i16,
}

impl Default for RaceNews {
    fn default() -> Self {
        Self {
            posx: (1920 - 600) / 2,
            posy: 54,
        }
    }
}

impl Overlay for RaceNews {
    fn init(&mut self, width: i16, _height: i16) {
        self.posx = (width - 600) / 2;
        self.posy = 54;
    }

    fn draw_ui(&mut self, store: &RacingStore) {
        if store.brief_news.is_empty() {
            return;
        }

        let text = CString::new(store.brief_news.clone()).unwrap_or_default();
        unsafe { RBR_DrawTextOverRsfMain(self.posx, self.posy, 0xFFFFFFFF, text.as_ptr()) };
    }
}
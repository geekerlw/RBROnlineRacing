use std::ffi::CString;
use crate::{components::store::RacingStore, RBR_DrawTextOverRsfMain};
use super::Overlay;

pub struct RaceNews {
    posx: i16,
    posy: i16,
    content: CString,
}

impl Default for RaceNews {
    fn default() -> Self {
        Self {
            posx: (1920 - 600) / 2,
            posy: 45,
            content: CString::new("Connect Server Failed.").expect("Failed to init CString."),
        }
    }
}

impl Overlay for RaceNews {
    fn init(&mut self, width: i16, _height: i16) {
        self.posx = (width - 600) / 2;
        self.posy = 45;
    }

    fn update(&mut self, store: &RacingStore) {
        self.content = CString::new(store.brief_news.clone()).expect("Failed to init CString.");
    }

    fn draw_ui(&mut self) {
        unsafe { RBR_DrawTextOverRsfMain(self.posx, self.posy, 0xFFFFFFFF, self.content.as_ptr()) };
    }
}
use std::ffi::CString;

use crate::{components::store::RacingStore, RBR_DrawTextOverRsfHotlap, RBR_DrawTextOverRsfPractice};
use super::Overlay;

pub struct RaceNotice {
    posx: i16,
    posy: i16,
}

impl Default for RaceNotice {
    fn default() -> Self {
        Self {
            posx: (1920 - 700) / 2,
            posy: 54,
        }
    }
}

impl Overlay for RaceNotice {
    fn init(&mut self, width: i16, _height: i16) {
        self.posx = (width - 700) / 2;
        self.posy = 54;
    }

    fn draw_ui(&mut self, store: &RacingStore) {
        if store.noticeinfo.is_empty() {
            return;
        }

        let text = CString::new(store.noticeinfo.clone()).unwrap_or_default();
        unsafe { 
            RBR_DrawTextOverRsfHotlap(self.posx, self.posy, 0xFFFFFFFF, text.as_ptr());
            RBR_DrawTextOverRsfPractice(self.posx, self.posy, 0xFFFFFFFF, text.as_ptr());
        };
    }
}
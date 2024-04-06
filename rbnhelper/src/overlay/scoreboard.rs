use std::ffi::CString;
use crate::{components::store::RacingStore, RBR_DrawTextOverRsf};
use super::Overlay;

pub struct ScoreBoard {
    posx: i16,
    posy: i16,
}

impl Default for ScoreBoard {
    fn default() -> Self {
        Self {
            posx: 1780,
            posy: 5,
        }
    }
}

impl Overlay for ScoreBoard {
    fn init(&mut self, width: i16, _height: i16) {
        self.posx = width - 140;
        self.posy = 5;
    }

    fn draw_ui(&mut self, store: &RacingStore) {
        let text = CString::new(format!("{} {}", store.scoreinfo.license, store.scoreinfo.score)).unwrap_or_default();
        unsafe { RBR_DrawTextOverRsf(self.posx, self.posy, 0xFFFFFFFF, text.as_ptr()) };
    }
}
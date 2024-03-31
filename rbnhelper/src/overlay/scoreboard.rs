use std::ffi::CString;
use crate::{components::store::RacingStore, RBR_DrawTextOverRsf};
use super::Overlay;

pub struct ScoreBoard {
    posx: i16,
    posy: i16,
    content: CString,
}

impl Default for ScoreBoard {
    fn default() -> Self {
        Self {
            posx: 1800,
            posy: 5,
            content: CString::new("Rookie 666").expect("Failed to init copyright."),
        }
    }
}

impl Overlay for ScoreBoard {
    fn init(&mut self, width: i16, _height: i16) {
        self.posx = width - 120;
        self.posy = 5;
    }

    fn update(&mut self, _store: &RacingStore) {

    }

    fn draw_ui(&mut self) {
        unsafe { RBR_DrawTextOverRsf(self.posx, self.posy, 0xFFFFFFFF, self.content.as_ptr()) };
    }
}
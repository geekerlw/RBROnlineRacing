use std::ffi::CString;
use crate::RBR_DrawTextOverRsfMain;
use super::Overlay;

pub struct RaceNews {
    posx: i16,
    posy: i16,
    content: CString,
}

impl Default for RaceNews {
    fn default() -> Self {
        Self {
            posx: (1920 - 320) / 2,
            posy: 40,
            content: CString::new("Connect Server Failed.").expect("Failed to init copyright."),
        }
    }
}

impl Overlay for RaceNews {
    fn init(&mut self, width: i16, _height: i16) {
        self.posx = (width - 320) / 2;
        self.posy = 40;
    }

    fn draw_ui(&mut self) {
        unsafe { RBR_DrawTextOverRsfMain(self.posx, self.posy, 0xFFFFFFFF, self.content.as_ptr()) };
    }
}
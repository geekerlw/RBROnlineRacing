use std::ffi::CString;
use crate::{components::store::RacingStore, RBR_DrawTextOverRsfMain};
use super::Overlay;

pub struct CopyRight {
    posx: i16,
    posy: i16,
    content: CString,
}

impl Default for CopyRight {
    fn default() -> Self {
        Self {
            posx: (1920 - 680) / 2,
            posy: 1000,
            content: CString::new(format!("RBN Helper [{}], https:://www.rbrlover.cn, Copyright (C) Lw_Ziye 2023-2024.", std::env!("CARGO_PKG_VERSION"))).expect("Failed to init copyright."),
        }
    }
}

impl Overlay for CopyRight {
    fn init(&mut self, width: i16, height: i16) {
        self.posx = (width - 680) / 2;
        self.posy = height - 60;
    }

    fn draw_ui(&mut self, _store: &RacingStore) {
        unsafe { RBR_DrawTextOverRsfMain(self.posx, self.posy, 0xFFFFFFFF, self.content.as_ptr()) };
    }
}
use std::{cell::RefCell, ffi::CString, rc::Rc};
use rbrproxy::game::{RBRGame, RBRGrapher, RBRMemReader};
use crate::components::store::RacingStore;

use super::Overlay;

#[derive(Default)]
pub struct ProgressBar {
    height: f32,
    stagelen: f32,
    pos: [f32; 2],
    player_name: String,
    bkground_color: u32,
    split_color: u32,
    own_color: u32,
    other_color: u32,
    grapher: RBRGrapher,
}

impl Overlay for ProgressBar {
    fn init(&mut self) {
        self.height = 400.0;
        self.stagelen = RBRMemReader::default().read_stage_len();
        self.pos = [30.0, 300.0];
        self.player_name = RBRGame::default().get_user_name();
        self.bkground_color = 0x5FFFFFFF;
        self.split_color = 0xDFFFFFFF;
        self.own_color = 0xBFFB4B01;
        self.other_color = 0x2FFFFFFF;
    }

    fn draw(&self, store: &RacingStore) {
        let grapher = &self.grapher;
        let posx = self.pos[0];
        let posy = self.pos[1];

        grapher.begin_draw();
        grapher.draw_filled_box(posx, posy, 10.0, self.height, self.bkground_color);
        for i in 0..9 {
            grapher.draw_filled_box(posx - 1.0, posy + i as f32 * self.height / 8.0, 12.0, 3.0, self.split_color);
        }

        for player in &store.racedata {
            let left = 1f32 - (player.progress / self.stagelen);
            grapher.draw_filled_box(posx + 12.0, posy + self.height * left, 20.0 , 2.0, 0xFF00FF00);
            let name = CString::new(player.profile_name.as_str()).expect("failed");
            if player.profile_name == self.player_name {
                grapher.draw_string(posx + 45.0, posy + self.height * left - 10.0, self.own_color, name.as_ptr());
            } else {
                grapher.draw_string(posx + 45.0, posy + self.height * left - 10.0, 0xFFFFFFFF, name.as_ptr());
            }
        }
        grapher.end_draw();
    }
}
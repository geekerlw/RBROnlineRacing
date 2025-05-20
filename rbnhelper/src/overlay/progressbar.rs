use std::ffi::CString;
use rbrproxy::game::{RBRGame, RBRGrapher, RBRMemReader};
use super::Overlay;

#[derive(Default)]
pub struct ProgressBar {
    height: i16,
    stagelen: f32,
    pos: [i16; 2],
    player_name: String,
    bkground_color: u32,
    split_color: u32,
    own_color: u32,
    other_color: u32,
    grapher: RBRGrapher,
}

impl Overlay for ProgressBar {
    fn init(&mut self) {
        self.height = 400;
        self.stagelen = RBRMemReader::default().read_stage_len();
        self.pos = [30, 100];
        self.player_name = RBRGame::default().get_user_name();
        self.bkground_color = 0xFFFFFFFF;
        self.split_color = 0xFF00FF00;
        self.own_color = 0xFFFF0000;
        self.other_color = 0xFF00FF00;
    }

    fn draw(&self, store: &crate::components::store::RacingStore) {
        let posx = self.pos[0];
        let posy = self.pos[1];

        self.grapher.begin_draw();
        self.grapher.draw_filled_box(posx, posy, 10, self.height, self.bkground_color);
        for i in 0..8 {
            self.grapher.draw_filled_box(posx - 1, posy + i * self.height / 8, 12, 3, self.split_color);
        }

        for player in &store.racedata {
            let left = 1f32 - (player.progress / self.stagelen);
            self.grapher.draw_filled_box(posx + 12, posy + (self.height as f32 * left) as i16, 20 , 2, 0xFFFFFFFF);
            let name = CString::new(player.profile_name.as_str()).expect("failed");
            if player.profile_name == self.player_name {
                self.grapher.draw_string(posx + 45, posy + (self.height as f32 * left) as i16 - 10, self.own_color, name.as_ptr());
            } else {
                self.grapher.draw_string(posx + 45, posy + (self.height as f32 * left) as i16 - 10, self.other_color, name.as_ptr());
            }
        }
        self.grapher.end_draw();
    }
}
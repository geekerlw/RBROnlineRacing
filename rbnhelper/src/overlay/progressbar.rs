use std::ffi::{c_void, CString};
use rbrproxy::RBRProxy;
use super::Overlay;

pub struct ProgressBar {
    height: i16,
    stagelen: f32,
    pos: [i16; 2],
    player_name: String,
    bkground_color: u32,
    split_color: u32,
    own_color: u32,
    other_color: u32,
    rbrproxy: RBRProxy,
    render: *mut c_void,
}

impl Overlay for ProgressBar {
    fn init(&mut self) {
        self.pos = [30, 100];
    }

    fn draw(&self, store: &crate::components::store::RacingStore) {
        let posx = self.pos[0];
        let posy = self.pos[1];

        self.rbrproxy.graph_begin_draw(self.render);
        self.rbrproxy.graph_draw_filled_box(self.render, posx, posy, 10, self.height, self.bkground_color);
        for i in 0..8 {
            self.rbrproxy.graph_draw_filled_box(self.render, posx - 1, posy + i * self.height / 8, 12, 3, self.split_color);
        }

        for player in &store.racedata {
            let left = 1f32 - (player.progress / self.stagelen);
            self.rbrproxy.graph_draw_filled_box(self.render, posx + 12, posy + (self.height as f32 * left) as i16, 20 , 2, 0xFFFFFFFF);
            let name = CString::new(player.profile_name.as_str()).expect("failed");
            if player.profile_name == self.player_name {
                self.rbrproxy.graph_draw_string(self.render, posx + 45, posy + (self.height as f32 * left) as i16 - 10, self.own_color, name.as_ptr());
            } else {
                self.rbrproxy.graph_draw_string(self.render, posx + 45, posy + (self.height as f32 * left) as i16 - 10, self.other_color, name.as_ptr());
            }
        }
        self.rbrproxy.graph_end_draw(self.render);
    }
}
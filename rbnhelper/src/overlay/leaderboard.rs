use std::{cell::RefCell, ffi::CString, rc::Rc};
use rbrproxy::game::{RBRGame, RBRGrapher};
use crate::components::{store::RacingStore, utils::format_seconds};
use super::Overlay;

#[derive(Default)]
pub struct LeaderBoard {
    pos: [f32; 2],
    player_colors: [u32; 8],
    player_name: String,
    own_color: u32,
    other_color: u32,
    grapher: RBRGrapher,
}

impl Overlay for LeaderBoard {
    fn init(&mut self) {
        self.pos = [20.0, 20.0];
        self.player_colors = [
            0xEF00873B, 0xEF0950B4, 0xEFB91F86, 0xEFBC1A15,
            0xEF37F6FF, 0xEFCEBA52, 0xEF413E25, 0xEF9C353B,
        ];
        self.player_name = RBRGame::default().get_user_name();
        self.own_color = 0xBFFB4B01;
        self.other_color = 0x2FFFFFFF;
    }

    fn draw(&self, store: &RacingStore) {
        let grapher = &self.grapher;

        let posx = self.pos[0];
        let mut posy = self.pos[1];
        grapher.begin_draw();
        for (i, player) in store.racedata.iter().enumerate() {
            if i > 8 {
                break; // only show front 8 players.
            }

            // rank background
            let itsme = player.profile_name == self.player_name;
            if itsme {
                grapher.draw_filled_box(posx, posy, 28.0, 28.0, self.own_color);
            } else {
                grapher.draw_filled_box(posx, posy, 28.0, 28.0, self.other_color);
            }

            // rank
            let rank = CString::new((i + 1).to_string()).expect("failed");
            grapher.draw_string(posx + 6.0, posy + 2.0, 0xFFFFFFFF, rank.as_ptr());

            // player name background
            grapher.draw_filled_box(posx + 36.0, posy, 6.0, 28.0, self.player_colors[i]);
            if itsme {
                grapher.draw_filled_box(posx + 45.0, posy, 240.0, 28.0, self.own_color);
            } else {
                grapher.draw_filled_box(posx + 45.0, posy, 240.0, 28.0, self.other_color);
            }
            // player name
            let name = CString::new(player.profile_name.as_str()).expect("failed");
            grapher.draw_string(posx + 50.0, posy + 2.0, 0xFFFFFFFF, name.as_ptr());

            // diff time
            let diffstr = CString::new(format!("+{}", format_seconds(player.difffirst))).expect("failed");
            self.grapher.draw_string(posx + 180.0, posy + 2.0, 0xFFFFFFFF, diffstr.as_ptr());

            posy += 30.0;
        }
        grapher.end_draw();
    }
}
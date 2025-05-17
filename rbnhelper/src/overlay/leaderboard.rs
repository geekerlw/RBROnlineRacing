use std::ffi::CString;
use rbrproxy::game::RBRGrapher;
use crate::components::utils::format_seconds;
use super::Overlay;

pub struct LeaderBoard {
    pos: [i16; 2],
    player_colors: [u32; 8],
    player_name: String,
    own_color: u32,
    other_color: u32,
    grapher: RBRGrapher,
}

impl Overlay for LeaderBoard {
    fn init(&mut self) {
        self.pos = [20, 20];
    }

    fn draw(&self, store: &crate::components::store::RacingStore) {
        let posx = self.pos[0];
        let mut posy = self.pos[1];
        self.grapher.begin_draw();
        for (i, player) in store.racedata.iter().enumerate() {
            if i > 8 {
                break; // only show front 8 players.
            }

            // rank background
            let itsme = player.profile_name == self.player_name;
            if itsme {
                self.grapher.draw_filled_box(posx, posy, 28, 28, self.own_color);
            } else {
                self.grapher.draw_filled_box(posx, posy, 28, 28, self.other_color);
            }

            // rank
            let rank = CString::new((i + 1).to_string()).expect("failed");
            self.grapher.draw_string(posx + 6, posy, 0xFFFFFFFF, rank.as_ptr());

            // player name background
            self.grapher.draw_filled_box(posx + 36, posy, 6, 28, self.player_colors[i]);
            if itsme {
                self.grapher.draw_filled_box(posx + 45, posy, 240, 28, self.own_color);
            } else {
                self.grapher.draw_filled_box(posx + 45, posy, 240, 28, self.other_color);
            }
            // player name
            let name = CString::new(player.profile_name.as_str()).expect("failed");
            self.grapher.draw_string(posx + 50, posy, 0xFFFFFFFF, name.as_ptr());

            // diff time
            let diffstr = CString::new(format_seconds(player.difffirst)).expect("failed");
            self.grapher.draw_string(posx + 180, posy, 0xFFFFFFFF, diffstr.as_ptr());

            posy += 30;
        }
        self.grapher.end_draw();
    }
}
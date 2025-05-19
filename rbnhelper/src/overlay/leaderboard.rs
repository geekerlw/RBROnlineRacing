use std::ffi::CString;
use rbrproxy::game::RBRGrapher;
use crate::components::utils::format_seconds;
use super::Overlay;
use std::cell::OnceCell;

static RBRGRAPHER: OnceCell<RBRGrapher> = OnceCell::new();

fn get_grapher_or_init() -> &'static RBRGrapher {
    RBRGRAPHER.get_or_init(|| {
        let mut grapher = RBRGrapher::default();
        grapher.init(16, false);
        grapher
    })
}

#[derive(Default)]
pub struct LeaderBoard {
    pos: [i16; 2],
    player_colors: [u32; 8],
    player_name: String,
    own_color: u32,
    other_color: u32,
}

impl Overlay for LeaderBoard {
    fn init(mut self) -> Self {
        self.pos = [20, 20];
        self
    }

    fn draw(&self, store: &crate::components::store::RacingStore) {
        let grapher = get_grapher_or_init();

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
                grapher.draw_filled_box(posx, posy, 28, 28, self.own_color);
            } else {
                grapher.draw_filled_box(posx, posy, 28, 28, self.other_color);
            }

            // rank
            let rank = CString::new((i + 1).to_string()).expect("failed");
            grapher.draw_string(posx + 6, posy, 0xFFFFFFFF, rank.as_ptr());

            // player name background
            grapher.draw_filled_box(posx + 36, posy, 6, 28, self.player_colors[i]);
            if itsme {
                grapher.draw_filled_box(posx + 45, posy, 240, 28, self.own_color);
            } else {
                grapher.draw_filled_box(posx + 45, posy, 240, 28, self.other_color);
            }
            // player name
            let name = CString::new(player.profile_name.as_str()).expect("failed");
            grapher.draw_string(posx + 50, posy, 0xFFFFFFFF, name.as_ptr());

            // diff time
            let diffstr = CString::new(format_seconds(player.difffirst)).expect("failed");
            grapher.draw_string(posx + 180, posy, 0xFFFFFFFF, diffstr.as_ptr());

            posy += 30;
        }
        grapher.end_draw();
    }
}
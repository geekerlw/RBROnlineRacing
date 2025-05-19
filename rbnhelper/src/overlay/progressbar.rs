use std::ffi::CString;
use rbrproxy::game::RBRGrapher;
use super::Overlay;
use std::cell::OnceCell;

static RBRGRAPHER: OnceCell<RBRGrapher> = OnceCell::new();

fn get_grapher_or_init() -> &'static RBRGrapher {
    RBRGRAPHER.get_or_init(|| {
        let mut grapher = RBRGrapher::default();
        grapher.init(14, false);
        grapher
    })
}

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
}

impl Overlay for ProgressBar {
    fn init(mut self) -> Self {
        self.pos = [30, 100];
        self
    }

    fn draw(&self, store: &crate::components::store::RacingStore) {
        let grapher = get_grapher_or_init();
        let posx = self.pos[0];
        let posy = self.pos[1];

        grapher.begin_draw();
        grapher.draw_filled_box(posx, posy, 10, self.height, self.bkground_color);
        for i in 0..8 {
            grapher.draw_filled_box(posx - 1, posy + i * self.height / 8, 12, 3, self.split_color);
        }

        for player in &store.racedata {
            let left = 1f32 - (player.progress / self.stagelen);
            grapher.draw_filled_box(posx + 12, posy + (self.height as f32 * left) as i16, 20 , 2, 0xFFFFFFFF);
            let name = CString::new(player.profile_name.as_str()).expect("failed");
            if player.profile_name == self.player_name {
                grapher.draw_string(posx + 45, posy + (self.height as f32 * left) as i16 - 10, self.own_color, name.as_ptr());
            } else {
                grapher.draw_string(posx + 45, posy + (self.height as f32 * left) as i16 - 10, self.other_color, name.as_ptr());
            }
        }
        grapher.end_draw();
    }
}
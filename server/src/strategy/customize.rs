use protocol::httpapi::RoomState;
use crate::room::RaceRoom;

use super::RaceStrategy;

#[derive(Default)]
pub struct Customize;

impl RaceStrategy for Customize {
    fn auto_start(&mut self) -> bool {
        false
    }

    fn allow_empty(&mut self) -> bool {
        false
    }
}
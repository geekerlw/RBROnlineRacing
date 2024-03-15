use std::{future::Future, pin::Pin};

use crate::room::RaceRoom;

pub mod customize;
pub mod daily;

pub trait RaceStrategy {
    fn auto_start(&mut self) -> bool;

    fn allow_empty(&mut self) -> bool;


    fn update_room_state(&mut self, _room: &mut RaceRoom) -> Pin<Box<dyn Future<Output = ()>>> {
        Box::pin(async {})
    }

    fn update_race_state(&mut self, _room: &mut RaceRoom) -> Pin<Box<dyn Future<Output = ()>>> {
        Box::pin(async {})
    }
}
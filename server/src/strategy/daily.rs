use super::RaceStrategy;

#[derive(Default)]
pub struct Daily;

impl RaceStrategy for Daily {
    fn auto_start(&mut self) -> bool {
        true
    }

    fn allow_empty(&mut self) -> bool {
        true
    }
}
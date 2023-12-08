use protocol::httpapi::{RaceState, MetaRaceData};

#[derive(Clone)]
pub struct RacePlayer {
    pub profile_name: String,
    pub room_name: String,
    pub state: RaceState,
    pub race_data: MetaRaceData,
}

impl Default for RacePlayer {
    fn default() -> Self {
        Self {
            profile_name: String::from("anonymous"),
            room_name: String::new(),
            state: RaceState::default(),
            race_data: MetaRaceData::default(),
        }
    }
}

impl RacePlayer {
    pub fn new(username: String) -> Self {
        Self {
            profile_name: username,
            room_name: String::new(),
            state: RaceState::default(),
            race_data: MetaRaceData::default(),
        }
    }

    pub fn sort_by_time(&self, player: &RacePlayer) -> std::cmp::Ordering {
        if self.race_data.process > player.race_data.process {
            return std::cmp::Ordering::Greater;
        } else if self.race_data.process == player.race_data.process {
            return std::cmp::Ordering::Equal;
        } else {
            return std::cmp::Ordering::Less;
        }
    }
}
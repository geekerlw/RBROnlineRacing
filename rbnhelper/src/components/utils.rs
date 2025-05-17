use rbnproto::{httpapi::RaceState, metaapi::MetaRaceData};
use rbrproxy::game::RBRMemReader;

pub fn format_seconds(second: f32) -> String {
    let minutes = (second / 60.0) as u32;
    let seconds = (second % 60.0) as u32;
    let milliseconds = ((second % 1.0) * 1000.0) as u32;

    format!("{:02}:{:02}:{:03}", minutes, seconds, milliseconds)
}

pub fn get_race_state(reader: &RBRMemReader) -> RaceState {
    let mut state = RaceState::RaceDefault;
    let game_mode = reader.read_game_mode();
    let start_count = reader.read_race_start_count();
    let track_load_state = reader.read_track_load_state();
    if game_mode == 0x01 && start_count < 0f32 {
        state = RaceState::RaceRunning;
    } else if game_mode == 0x05 {
        state = RaceState::RaceLoading;
    } else if game_mode == 0x0A && track_load_state == 0x08 && start_count == 7f32 {
        state = RaceState::RaceLoaded;
    } else if game_mode == 0x09 {
        state = RaceState::RaceFinished;
    } else if game_mode == 0x0C {
        state = RaceState::RaceExitMenu;
    }
    state
}

pub fn get_race_data(reader: &RBRMemReader) -> MetaRaceData {
    let mut data = MetaRaceData::default();
    data.speed = reader.read_car_speed();
    data.racetime = reader.read_car_race_time();
    data.progress = reader.read_car_stage_progress();
    data.stagelen = reader.read_stage_len();
    data.splittime1 = reader.read_split1_time();
    data.splittime2 = reader.read_split2_time();
    data.finishtime = reader.read_finish_time();
    data.carlook = reader.read_car_look();
    data.carpos = reader.read_car_pos();
    data
}
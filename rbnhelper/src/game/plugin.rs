use libc::c_char;

#[allow(non_snake_case)]
pub trait IPlugin {
    fn GetName(&self) -> *const c_char;
    fn DrawFrontEndPage(&self) {}
    fn DrawResultUI(&self) {}
    fn HandleFrontEndEvents(&self, _txt: char, _up: bool, _down: bool, _left: bool, _right: bool, _select: bool) {}
    fn TickFrontEndPage(&self, _time: f32) {}
    fn StageStarted(&self, _mapid: i32, _player: *const c_char, _falsestart: bool) {}
    fn HandleResults(&self, _checkpoint1: f32, _checkpoint2: f32, _finishtime: f32, _player: *const c_char) {}
    fn CheckPoint(&self, _checkpointtime: f32, _checkpointid: i32, _player: *const c_char) {}
}
use libc::c_char;

#[allow(non_snake_case, dead_code)]
pub trait IPlugin {
    fn GetName(&mut self) -> *const c_char;
    fn DrawFrontEndPage(&mut self) {}
    fn DrawResultUI(&mut self) {}
    fn HandleFrontEndEvents(&mut self, _txt: c_char, _up: bool, _down: bool, _left: bool, _right: bool, _select: bool) {}
    fn TickFrontEndPage(&mut self, _time: f32) {}
    fn StageStarted(&mut self, _mapid: i32, _player: *const c_char, _falsestart: bool) {}
    fn HandleResults(&mut self, _checkpoint1: f32, _checkpoint2: f32, _finishtime: f32, _player: *const c_char) {}
    fn CheckPoint(&mut self, _checkpointtime: f32, _checkpointid: i32, _player: *const c_char) {}
}
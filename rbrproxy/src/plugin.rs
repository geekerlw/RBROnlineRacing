use libc::c_char;

pub trait IPlugin {
    fn get_name(&mut self) -> *const c_char;
    fn draw_menu(&mut self) {}
    fn draw_result(&mut self) {}
    fn handle_input(&mut self, _txt: c_char, _up: bool, _down: bool, _left: bool, _right: bool, _select: bool) {}
    fn tick_render(&mut self, _time: f32) {}
    fn on_stage_start(&mut self, _mapid: i32, _player: *const c_char, _falsestart: bool) {}
    fn on_stage_end(&mut self, _checkpoint1: f32, _checkpoint2: f32, _finishtime: f32, _player: *const c_char) {}
    fn on_stage_checkpoint(&mut self, _checkpointtime: f32, _checkpointid: i32, _player: *const c_char) {}
}
use libc::c_char;

pub trait IPlugin {
    fn plugin_init(&mut self) -> *const c_char;
    fn plugin_draw_menu(&mut self) {}
    fn plugin_draw_result(&mut self) {}
    fn plugin_handle_input(&mut self, _txt: c_char, _up: bool, _down: bool, _left: bool, _right: bool, _select: bool) {}
    fn plugin_tick_render(&mut self, _time: f32) {}
    fn plugin_on_stage_start(&mut self, _mapid: i32, _player: *const c_char, _falsestart: bool) {}
    fn plugin_on_stage_end(&mut self, _checkpoint1: f32, _checkpoint2: f32, _finishtime: f32, _player: *const c_char) {}
    fn plugin_on_stage_checkpoint(&mut self, _checkpointtime: f32, _checkpointid: i32, _player: *const c_char) {}

    fn plugin_on_gamemode_changed(&mut self) {}
    fn plugin_on_begin_scene(&mut self) {}
    fn plugin_on_frame(&mut self) {}
    fn plugin_on_end_scene(&mut self) {}
    fn plugin_on_ghost_movement(&mut self) {}
}
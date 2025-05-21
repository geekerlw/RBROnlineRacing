use libc::c_char;
use rbnhelper::RBNHelper;
use rbrproxy::plugin::IPlugin;
use lazy_static::lazy_static;
use std::sync::Mutex;

mod components;
mod overlay;
mod backend;
mod rbnhelper;
mod menu;

lazy_static! {
    static ref RBNHELPER: Mutex<RBNHelper> = Mutex::new(RBNHelper::default());
}

#[no_mangle]
extern fn plugin_init() -> *const c_char {
    let mut plugin = RBNHELPER.lock().unwrap();
    plugin.plugin_init()
}

#[no_mangle]
extern fn plugin_draw_menu() {
    let mut plugin = RBNHELPER.lock().unwrap();
    plugin.plugin_draw_menu()
}

#[no_mangle]
extern fn plugin_draw_result() {
    let mut plugin = RBNHELPER.lock().unwrap();
    plugin.plugin_draw_result()
}

#[no_mangle]
extern fn plugin_handle_input(keycode: c_char, up: bool, down: bool, left: bool, right: bool, select: bool) {
    let mut plugin = RBNHELPER.lock().unwrap();
    plugin.plugin_handle_input(keycode, up, down, left, right, select);
}

#[no_mangle]
extern fn plugin_on_end_scene() {
    let mut plugin = RBNHELPER.lock().unwrap();
    plugin.plugin_on_end_scene();
}

#[no_mangle]
extern fn plugin_on_stage_start(mapid: i32, player: *const c_char, falsestart: bool) {
    let mut plugin = RBNHELPER.lock().unwrap();
    plugin.plugin_on_stage_start(mapid, player, falsestart);
}

#[no_mangle]
extern fn plugin_on_stage_end(checkpoint1: f32, checkpoint2: f32, finishtime: f32, player: *const c_char) {
    let mut plugin = RBNHELPER.lock().unwrap();
    plugin.plugin_on_stage_end(checkpoint1, checkpoint2, finishtime, player);
}
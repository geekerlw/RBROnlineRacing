use libc::{c_char, c_void};
use game::plugin::IPlugin;
use rbnhelper::RBNHelper;
use log::info;
use simplelog::WriteLogger;
use game::hacker::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

mod components;
mod game;
mod rbnhelper;

lazy_static! {
    static ref RBNHELPER: Mutex<RBNHelper> = Mutex::new(RBNHelper::default());
}

#[no_mangle]
extern fn rbn_init() -> *const c_char {
    let mut plugin = RBNHELPER.lock().unwrap();
    plugin.init();
    plugin.GetName()
}

#[no_mangle]
extern fn rbn_on_end_frame() {
    let mut plugin = RBNHELPER.lock().unwrap();
    plugin.draw_on_end_frame();
}

#[no_mangle]
extern fn rbn_on_rsf_menu_changed(menu: i32) {
    let mut plugin = RBNHELPER.lock().unwrap();
    plugin.on_rsf_menu_changed(menu);
}

#[no_mangle]
extern "stdcall" fn DllMain(_hinst: usize, _reason: u32, _reserved: *mut ()) -> bool {
    true
}

#[no_mangle]
extern "cdecl" fn RBR_CreatePlugin(rbrgame: *mut c_void) -> *mut c_void {
    if let Some(game_path) = std::env::current_exe().unwrap().parent() {
        let plugin_path = game_path.join("Plugins").join("RBNHelper");
        let log_file = plugin_path.join("rbnhelper.log");
        if !plugin_path.exists() {
            std::fs::create_dir(plugin_path).unwrap();
        }
        WriteLogger::init(log::LevelFilter::Info, 
            simplelog::Config::default(), std::fs::File::create(log_file).unwrap()).unwrap();
    }

    info!("Create Plugin RBN Helper [{}] with arg: {:?}", std::env!("CARGO_PKG_VERSION"), rbrgame);

    unsafe {
        let plugin = RBR_InitPlugin(rbrgame);
        RBR_SetInitialize(rbn_init);
        RBR_SetOnEndScene(rbn_on_end_frame);
        RBR_SetOnRsfMenuChanged(rbn_on_rsf_menu_changed);

        return plugin;
    };
}
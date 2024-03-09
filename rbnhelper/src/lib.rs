use libc::{c_char, c_void};
use plugin::IPlugin;
use rbnhelper::RBNHelper;
use log::info;
use simplelog::WriteLogger;
use crate::hacker::*;

mod hacker;
mod plugin;
mod rbnhelper;

#[no_mangle]
extern fn rbn_init(ptr: *mut c_void) -> *const c_char {
    let mut plugin = RBNHelper::from(ptr);
    plugin.init();
    plugin.GetName()
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

    let rust_plugin = Box::into_raw(Box::new(RBNHelper::default())) as *mut c_void;
    unsafe {
        let plugin = RBR_InitPlugin(rbrgame, rust_plugin);
        RBR_SetInitialize(rbn_init);

        return plugin;
    };
}
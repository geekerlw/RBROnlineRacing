use std::os::raw::c_void;
use plugin::{IPlugin, RBRGame};
use log::info;
use simplelog::WriteLogger;

pub mod plugin;

#[link(name = "RBRHacker", kind = "static")]
extern "C" {
    fn RBR_InitPlugin(arg: *mut c_void) -> *mut c_void;
    fn RBR_SetInitialize(func: extern "C" fn());
    fn RBR_SetDrawFrontEndPage(func: extern "C" fn());
}

#[repr(C)]
#[derive(Default)]
struct RBNHelper{
}

impl IPlugin for RBNHelper {
    #[allow(non_snake_case)]
    fn GetName(&self) -> &str {
        "RBR Helper"
    }
}

#[no_mangle]
extern fn rbn_init() {
    info!("call rbn plugin init");
}

#[no_mangle]
extern fn rbn_draw_frontend_page() {
    info!("call draw frontend page");
}

#[no_mangle]
extern "stdcall" fn DllMain(_hinst: usize, _reason: u32, _reserved: *mut ()) -> bool {
    true
}

#[no_mangle]
extern "cdecl" fn RBR_CreatePlugin(rbrgame: *mut c_void) -> *mut c_void {
    let log_file = std::env::current_dir().unwrap().join("rbnhelper.log");
    WriteLogger::init(log::LevelFilter::Info, 
        simplelog::Config::default(), std::fs::File::create(log_file).unwrap()).unwrap();

    info!("Create Plugin RBN Helper [{}] with arg: {:?}", std::env!("CARGO_PKG_VERSION"), rbrgame);

    unsafe {
        let plugin = RBR_InitPlugin(rbrgame);
        RBR_SetInitialize(rbn_init);
        RBR_SetDrawFrontEndPage(rbn_draw_frontend_page);

        return plugin;
    };
}
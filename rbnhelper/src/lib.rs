use plugin::{IPlugin, RBRGame};
use log::info;
use simplelog::WriteLogger;

pub mod plugin;

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
extern "stdcall" fn DllMain(_hinst: usize, _reason: u32, _reserved: *mut ()) -> bool {
    true
}

#[no_mangle]
extern "cdecl" fn RBR_CreatePlugin(rbrgame: *mut RBRGame) -> *mut RBNHelper {
    let log_file = std::env::current_dir().unwrap().join("rbnhelper.log");
    WriteLogger::init(log::LevelFilter::Info, 
        simplelog::Config::default(), std::fs::File::create(log_file).unwrap()).unwrap();

    info!("Create Plugin RBN Helper [{}] with arg: {:?}", std::env!("CARGO_PKG_VERSION"), rbrgame);

    Box::into_raw(Box::new(RBNHelper::default()))
}
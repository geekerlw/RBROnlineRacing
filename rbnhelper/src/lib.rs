use plugin::IPlugin;
pub mod plugin;

#[repr(C)]
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
extern "cdecl" fn RBR_CreatePlugin(_rbrgame: i32) -> *mut dyn IPlugin {
    return Box::into_raw(Box::new(RBNHelper{}));
}
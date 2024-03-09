use libc::{c_char, c_void};

#[link(name = "RBRHacker", kind = "static")]
extern "C" {
    pub fn RBR_InitPlugin(rbrgame: *mut c_void, plug_inst: *mut c_void) -> *mut c_void;
    pub fn RBR_SetInitialize(func: extern "C" fn(*mut c_void) -> *const c_char);
}
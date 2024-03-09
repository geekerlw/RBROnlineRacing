use libc::c_void;
use crate::plugin::IPlugin;

#[derive(Default, Clone, Copy)]
pub struct RBNHelper {
}

impl IPlugin for RBNHelper {
    fn GetName(&self) -> *const libc::c_char {
        let name = std::ffi::CString::new("RBN Helper").unwrap();
        name.into_raw()
    }
}

impl From<*mut c_void> for RBNHelper {
    fn from(value: *mut c_void) -> Self {
        let ptr = value as *mut RBNHelper;
        unsafe {*ptr}
    }
}
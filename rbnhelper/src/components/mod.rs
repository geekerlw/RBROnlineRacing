pub mod input;
pub mod store;
pub mod time;
pub mod player;

#[cfg(target_os = "windows")]
fn get_system_scaling_factor() -> f32 {
    use winapi::um::wingdi::{LOGPIXELSY, GetDeviceCaps};

    let hdc = unsafe { winapi::um::winuser::GetDC(std::ptr::null_mut()) };
    if hdc.is_null() {
        return 1.0; // 默认值
    }
    let dpi = unsafe { GetDeviceCaps(hdc, LOGPIXELSY) as f32 };
    unsafe { winapi::um::winuser::ReleaseDC(std::ptr::null_mut(), hdc) };
    dpi / 96.0 // 计算缩放比例
}

#[cfg(not(target_os = "windows"))]
fn get_system_scaling_factor() -> f32 {
    1.0
}
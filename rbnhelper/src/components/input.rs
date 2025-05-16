#[cfg(target_os = "windows")]
pub fn is_horn_pressed() -> bool {
    use winapi::um::winuser::GetAsyncKeyState;

    let state = unsafe { GetAsyncKeyState(0x48) }; // special key VK_H

    0 != state & 0x8000u16 as i16
}

#[cfg(not(target_os = "windows"))]
pub fn is_horn_pressed() -> bool {
    false
}
use std::ffi::{c_char, c_int};
use rbnproto::D3DQuaternion;
use super::hacker::*;

#[derive(Default)]
pub struct RBRWindow;

impl RBRWindow {
    pub fn width(&self) -> i16 {
        #[cfg(target_os = "windows")]
        return unsafe { RBR_GetD3dWindowWidth() };

        #[cfg(not(target_os = "windows"))]
        return 1366i16;
    }

    pub fn height(&self) -> i16 {
        #[cfg(target_os = "windows")]
        return unsafe { RBR_GetD3dWindowHeight() };

        #[cfg(not(target_os = "windows"))]
        return 768i16;
    }

    pub fn fps(&self) -> i16 {
        #[cfg(target_os = "windows")]
        return unsafe { RBR_GetD3dWindowFps() };

        #[cfg(not(target_os = "windows"))]
        return 60i16;
    }

    #[cfg(target_os = "windows")]
    pub fn scall() -> f32 {
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
    pub fn scall() -> f32 {
        1.0
    }
}

#[derive(Default)]
pub struct RBRMenu;

impl RBRMenu {
    pub fn show_game_message(&self, text: *const c_char, time_to_display: f32, x: f32, y: f32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_ShowGameMessage(text, time_to_display, x, y);}
    }
    
    pub fn draw_text(&self, x: f32, y: f32, text: *const c_char) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_DrawText(x, y, text); }
    }

    pub fn draw_box(&self, i_box: u32, x: f32, y: f32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_DrawBox(i_box, x, y); }
    }

    pub fn draw_blackout(&self, x: f32, y: f32, w: f32, h: f32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_DrawBlackOut(x, y, w, h); }
    }

    pub fn draw_flatbox(&self, x: f32, y: f32, w: f32, h: f32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_DrawFlatBox(x, y, w, h); }
    }

    pub fn draw_selection(&self, x: f32, y: f32, w: f32, h: f32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_DrawSelection(x, y, w, h); }
    }

    pub fn set_draw_mode(&self, mode: i32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_SetDrawMode(mode); }
    }

    pub fn reset_draw_mode(&self, mode: i32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_ReSetDrawMode(mode); }
    }

    pub fn set_font_size(&self, font: i32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_SetFontSize(font);}
    }

    pub fn set_menu_color(&self, color: i32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_SetMenuColor(color);}
    }

    pub fn set_color(&self, r: f32, g: f32, b: f32, a: f32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_SetColor(r, g, b, a);}
    } 
}

#[derive(Default, Sync, Copy)]
pub struct RBRGrapher;

impl RBRGrapher {
    pub fn begin_draw(&self) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_GraphBeginDraw() }
    }

    pub fn end_draw(&self) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_GraphEndDraw() }
    }

    pub fn draw_string(&self, x: i16, y: i16, color: u32, text: *const c_char) {
        #[cfg(target_os = "windows")]
        unsafe {RBR_GraphDrawString(x, y, color, text)}
    }

    pub fn draw_line(&self, x1: i16, y1: i16, x2: i16, y2: i16, color: u32) {
        #[cfg(target_os = "windows")]
        unsafe {RBR_GraphDrawLine(x1, y1, x2, y2, color)}
    }

    pub fn draw_filled_box(&self, x: i16, y: i16, width: i16, height: i16, color: u32) {
        #[cfg(target_os = "windows")]
        unsafe {RBR_GraphDrawFilledBox(x, y, width, height, color)}
    }
}

#[derive(Default)]
pub struct RBRMemReader;

impl RBRMemReader {
    pub fn read_game_mode(&self) -> i32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadGameMode()
        };

        #[cfg(not(target_os = "windows"))]
        return 0
    }

    pub fn read_track_load_state(&self) -> i32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadTrackLoadState()
        };

        #[cfg(not(target_os = "windows"))]
        return 0
    }

    pub fn read_race_start_count(&self) -> f32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadRaceStartCount()
        };

        #[cfg(not(target_os = "windows"))]
        return 0f32
    }

    pub fn read_car_speed(&self) -> f32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadCarSpeed()
        };

        #[cfg(not(target_os = "windows"))]
        return 0f32
    }

    pub fn read_car_race_time(&self) -> f32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadCarRaceTime()
        };

        #[cfg(not(target_os = "windows"))]
        return 0f32
    }

    pub fn read_stage_len(&self) -> f32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadStageLen()
        };

        #[cfg(not(target_os = "windows"))]
        return 0f32
    }

    pub fn read_car_stage_progress(&self) -> f32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadCarStageProgress()
        };

        #[cfg(not(target_os = "windows"))]
        return 0f32
    }

    pub fn read_split1_time(&self) -> f32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadSplitTime1()
        };

        #[cfg(not(target_os = "windows"))]
        return 0f32
    }

    pub fn read_split2_time(&self) -> f32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadSplitTime2()
        };

        #[cfg(not(target_os = "windows"))]
        return 0f32
    }

    pub fn read_finish_time(&self) -> f32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadFinishTime()
        };

        #[cfg(not(target_os = "windows"))]
        return 0f32
    }

    pub fn read_car_finished(&self) -> bool {
                #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadCarFinished()
        };

        #[cfg(not(target_os = "windows"))]
        return false
    }

    pub fn read_car_look(&self) -> D3DQuaternion {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadCarLook()
        };

        #[cfg(not(target_os = "windows"))]
        return D3DQuaternion::default();
    }

    pub fn read_car_pos(&self) -> D3DQuaternion {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadCarPos()
        };

        #[cfg(not(target_os = "windows"))]
        return D3DQuaternion::default();
    }
}

#[derive(Default)]
pub struct RBRInput;

impl RBRInput {
    #[cfg(target_os = "windows")]
    pub fn is_key_pressed(&self, keycode: c_int) -> bool {
        use winapi::um::winuser::GetAsyncKeyState;
        let state = unsafe { GetAsyncKeyState(keycode) };
        0 != state & 0x8000u16 as i16
    }

    #[cfg(not(target_os = "windows"))]
    pub fn is_horn_pressed(&self, keycode: c_int) -> bool {
        false
    }
}


#[derive(Default)]
pub struct RBRGame;

impl RBRGame {
    pub fn get_user_name(&self) -> String {
        "Lw_Ziye".to_string()
    }

    pub fn prepare_stage(&self, map: u32, timeofday: u32, skycloudtype: u32, timeofday2: u32, skytype: u32, surface: u32) {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_PrepareStage(map, timeofday, skycloudtype, timeofday2, skytype, surface);
        };
    }

    pub fn prepare_car(&self, carid: u32, tyre: u32, setup: *const c_char) {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_PrepareCar(carid, tyre, setup);
        };
    }

    pub fn load(&self) {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_LoadGame();
        };
    }

    pub fn start(&self) {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_StartGame();
        };
    }
}
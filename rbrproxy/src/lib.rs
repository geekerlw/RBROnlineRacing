use hacker::*;
use libc::c_char;
use rbnproto::D3DQuaternion;

pub mod hacker;

pub struct RBRProxy;

impl RBRProxy {
    pub fn init(&self) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_ProxyInit(); }
    }

    pub fn start_game(&self, imap: i32, icar: i32, weather: i32, tyre: i32, setup: *const c_char) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_StartGame(imap, icar, weather, tyre, setup); }
    }

    pub fn show_game_message(&self, text: *const c_char, time_to_display: f32, x: f32, y: f32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_ShowGameMessage(text, time_to_display, x, y);}
    }
    
    pub fn draw_text(x: f32, y: f32, text: *const c_char) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_DrawText(x, y, text); }
    }

    pub fn draw_box(i_box: u32, x: f32, y: f32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_DrawBox(i_box, x, y); }
    }

    pub fn draw_blackout(x: f32, y: f32, w: f32, h: f32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_DrawBlackOut(x, y, w, h); }
    }

    pub fn draw_flatbox(x: f32, y: f32, w: f32, h: f32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_DrawFlatBox(x, y, w, h); }
    }

    pub fn draw_selection(x: f32, y: f32, w: f32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_DrawSelection(x, y, w); }
    }

    pub fn set_draw_mode(mode: i32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_SetDrawMode(mode); }
    }

    pub fn reset_draw_mode(mode: i32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_ReSetDrawMode(mode); }
    }

    pub fn set_font_size(font: i32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_SetFontSize(font);}
    }

    pub fn set_menu_color(color: i32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_SetMenuColor(color);}
    }

    pub fn set_color(r: f32, g: f32, b: f32, a: f32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_SetColor(r, g, b, a);}
    }

    /*
     * Reading memory functions.
     */
    pub fn read_game_mode() -> i32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadGameMode()
        };

        #[cfg(not(target_os = "windows"))]
        return 0
    }

    pub fn read_track_load_state() -> i32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadTrackLoadState()
        };

        #[cfg(not(target_os = "windows"))]
        return 0
    }

    pub fn read_race_start_count() -> f32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadRaceStartCount()
        };

        #[cfg(not(target_os = "windows"))]
        return 0f32
    }

    pub fn read_car_speed() -> f32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadCarSpeed()
        };

        #[cfg(not(target_os = "windows"))]
        return 0f32
    }

    pub fn read_car_race_time() -> f32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadCarRaceTime()
        };

        #[cfg(not(target_os = "windows"))]
        return 0f32
    }

    pub fn read_stage_len() -> f32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadStageLen()
        };

        #[cfg(not(target_os = "windows"))]
        return 0f32
    }

    pub fn read_car_stage_progress() -> f32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadCarStageProgress()
        };

        #[cfg(not(target_os = "windows"))]
        return 0f32
    }

    pub fn read_split1_time() -> f32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadSplitTime1()
        };

        #[cfg(not(target_os = "windows"))]
        return 0f32
    }

    pub fn read_split2_time() -> f32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadSplitTime2()
        };

        #[cfg(not(target_os = "windows"))]
        return 0f32
    }

    pub fn read_finish_time() -> f32 {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadFinishTime()
        };

        #[cfg(not(target_os = "windows"))]
        return 0f32
    }

    pub fn read_car_finished() -> bool {
                #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadCarFinished()
        };

        #[cfg(not(target_os = "windows"))]
        return 0f32
    }

    pub fn read_car_look() -> D3DQuaternion {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadCarLook()
        };

        #[cfg(not(target_os = "windows"))]
        return D3DQuaternion::default();
    }

    pub fn read_car_pos() -> D3DQuaternion {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_ReadCarPos()
        };

        #[cfg(not(target_os = "windows"))]
        return D3DQuaternion::default();
    }
}
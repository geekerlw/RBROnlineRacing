#[allow(unused_imports)]
use hacker::*;
use libc::{c_char, c_void};
use rbnproto::D3DQuaternion;

mod hacker;
pub mod game;
pub mod plugin;

pub struct RBRProxy;

impl Default for RBRProxy {
    fn default() -> Self {
        let proxy = RBRProxy;
        proxy.init();
        proxy
    }
}

#[allow(unused_variables)]
impl RBRProxy {
    fn init(&self) {
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

    pub fn draw_selection(&self, x: f32, y: f32, w: f32) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_DrawSelection(x, y, w); }
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

    /**
     * Functions to draw in game overlay or graph.
     */
    pub fn create_graph_render(&self, fontsize: i32, bold: bool) -> *mut c_void {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_CreateGraphRender(fontsize, bold)
        };

        #[cfg(not(target_os = "windows"))]
        return std::ptr::null_mut()
    }

    pub fn destroy_graph_render(&self, render: *mut c_void) {
        #[cfg(target_os = "windows")]
        return unsafe {
            RBR_DestroyGraphRender(render)
        };
    }

    pub fn graph_begin_draw(&self, render: *mut c_void) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_GraphBeginDraw(render) }
    }

    pub fn graph_end_draw(&self, render: *mut c_void) {
        #[cfg(target_os = "windows")]
        unsafe { RBR_GraphEndDraw(render) }
    }

    pub fn graph_draw_string(&self, render: *mut c_void, x: i16, y: i16, color: i32, text: *const c_char) {
        #[cfg(target_os = "windows")]
        unsafe {RBR_GraphDrawString(render, x, y, color, text)}
    }

    pub fn graph_draw_line(&self, render: *mut c_void, x1: i16, y1: i16, x2: i16, y2: i16, color: i32) {
        #[cfg(target_os = "windows")]
        unsafe {RBR_GraphDrawLine(render, x1, y1, x2, y2, color)}
    }

    pub fn graph_draw_filled_box(&self, render: *mut c_void, x: i16, y: i16, width: i16, height: i16, color: i32) {
        #[cfg(target_os = "windows")]
        unsafe {RBR_GraphDrawFilledBox(render, x, y, width, height, color)}
    }

    /*
     * Reading memory functions.
     */
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
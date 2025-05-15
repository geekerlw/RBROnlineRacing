use libc::c_char;
use rbnproto::D3DQuaternion;

#[cfg(target_os = "windows")]
#[link(name = "RBRProxy", kind = "static")]
unsafe extern "C" {
    /*
     * plugin hooking functions.
     */
    pub fn RBR_ProxyInit();

    /**
     * Functions to control game flow.
     */
    pub fn RBR_StartGame(imap: i32, icar: i32, weather: i32, tyre: i32, setup: *const c_char);

    /*
     * ui functions.
     */
    pub fn RBR_ShowGameMessage(text: *const c_char, time_to_display: f32, x: f32, y: f32);
    pub fn RBR_DrawText(x: f32, y: f32, text: *const c_char);
    pub fn RBR_DrawBox(iBox: u32, x: f32, y: f32);
    pub fn RBR_DrawBlackOut(x: f32, y: f32, w: f32, h: f32);
    pub fn RBR_DrawFlatBox(x: f32, y: f32, w: f32, h: f32);
    pub fn RBR_DrawSelection(x: f32, y: f32, w: f32);
    pub fn RBR_SetDrawMode(mode: i32);
    pub fn RBR_ReSetDrawMode(mode: i32);
    pub fn RBR_SetFontSize(font: i32);
    pub fn RBR_SetMenuColor(color: i32);
    pub fn RBR_SetColor(r: f32, g: f32, b: f32, a: f32);

    /*
     * Reading memory functions.
     */
    pub fn RBR_ReadGameMode() -> i32;
    pub fn RBR_ReadTrackLoadState() -> i32;
    pub fn RBR_ReadRaceStartCount() -> f32;
    pub fn RBR_ReadCarSpeed() -> f32;
    pub fn RBR_ReadCarRaceTime() -> f32;
    pub fn RBR_ReadStageLen() -> f32;
    pub fn RBR_ReadCarStageProgress() -> f32;
    pub fn RBR_ReadSplitTime1() -> f32;
    pub fn RBR_ReadSplitTime2() -> f32;
    pub fn RBR_ReadFinishTime() -> f32;
    pub fn RBR_ReadCarFinished() -> bool;
    pub fn RBR_ReadCarLook() -> D3DQuaternion;
    pub fn RBR_ReadCarPos() -> D3DQuaternion;
}
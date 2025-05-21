use std::ffi::c_char;
use rbnproto::D3DQuaternion;

#[link(name = "RBRProxy", kind = "static")]
unsafe extern "C" {
    /*
     * plugin hooking functions.
     */
    pub fn RBR_ProxyInit();

    /**
     * Funtions to access DirectX9.
     */
    pub fn RBR_GetD3dWindowWidth() -> i16;

    pub fn RBR_GetD3dWindowHeight() -> i16;

    pub fn RBR_GetD3dWindowFps() -> i16;

    /**
     * Functions to control game flow.
     */
    pub fn RBR_PrepareStage(iMap: u32, timeofday: u32, skycloudtype: u32, timeofday2: u32, skytype: u32, surfacetype: u32);

    pub fn RBR_PrepareCar(carid: u32, tyre: u32, setup: *const c_char);

    pub fn RBR_LoadGame();

    pub fn RBR_StartGame();

    /*
     * ui functions.
     */
    pub fn RBR_ShowGameMessage(text: *const c_char, time_to_display: f32, x: f32, y: f32);
    pub fn RBR_DrawText(x: f32, y: f32, text: *const c_char);
    pub fn RBR_DrawBox(iBox: u32, x: f32, y: f32);
    pub fn RBR_DrawBlackOut(x: f32, y: f32, w: f32, h: f32);
    pub fn RBR_DrawFlatBox(x: f32, y: f32, w: f32, h: f32);
    pub fn RBR_DrawSelection(x: f32, y: f32, w: f32, h: f32);
    pub fn RBR_SetDrawMode(mode: i32);
    pub fn RBR_ReSetDrawMode(mode: i32);
    pub fn RBR_SetFontSize(font: i32);
    pub fn RBR_SetMenuColor(color: i32);
    pub fn RBR_SetColor(r: f32, g: f32, b: f32, a: f32);

    /**
     * Functions to draw in game overlay or graph.
     */
    pub fn RBR_GraphBeginDraw();
    pub fn RBR_GraphEndDraw();
    pub fn RBR_GraphDrawString(x: f32, y: f32, color: u32, text: *const c_char);
    pub fn RBR_GraphDrawLine(x1: f32, y1: f32, x2: f32, y2: f32, color: u32);
    pub fn RBR_GraphDrawFilledBox(x: f32, y: f32, width: f32, height: f32, color: u32);

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
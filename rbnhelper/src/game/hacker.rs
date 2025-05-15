use libc::{c_char, c_uchar, c_void};
use rbnproto::{rsfdata::{RBRRaceData, RBRRaceResult, RBRRaceSetting, RBRRaceState}, D3DQuaternion};

#[link(name = "RBRHacker", kind = "static")]
extern "C" {
    /*
     * plugin hooking functions.
     */
    pub fn RBR_InitPlugin(rbrgame: *mut c_void) -> *mut c_void;
    pub fn RBR_InitLogger(logfile: *const c_char);
    pub fn RBR_GetD3dWindowWidth() -> i16;
    pub fn RBR_GetD3dWindowHeight() -> i16;
    pub fn RBR_SetInitialize(func: extern "C" fn() -> *const c_char);
    pub fn RBR_SetOnEndScene(func: extern "C" fn());
    pub fn RBR_SetOnRsfMenuChanged(func: extern "C" fn(menu: i32));
    pub fn RBR_SetOnGameModeChanged(func: extern "C" fn());
    pub fn RBR_SetDrawFrontEndPage(func: extern "C" fn());

    /*
     * ui functions.
     */
    pub fn RBR_EnableLeaderBoard();
    pub fn RBR_CfgLeaderBoardPos(posx: u16, posy: u16);
    pub fn RBR_CfgLeaderBoardStyle(LeaderBriefColor: *const c_char, LeaderBackGroundColor: *const c_char);
    pub fn RBR_EnableProgressBar();
    pub fn RBR_CfgProgressBarPos(posx: u16, posy: u16);
    pub fn RBR_CfgProgressBarStyle(ProgressBarBackColor: *const c_char, ProgressBarSplitColor: *const c_char, ProgressBarPointerColor: *const c_char);
    pub fn RBR_CfgProfileStyle(UserColor1: *const c_char, UserColor2: *const c_char);
    pub fn RBR_CfgFontSize(dashFontSize: i32, textFontSize: i32);
    pub fn RBR_DrawTextOverRsf(x: i16, y: i16, color: u32, text: *const c_uchar);
    pub fn RBR_DrawTextOverRsfMain(x: i16, y: i16, color: u32, text: *const c_char);
    pub fn RBR_DrawTextOverRsfHotlap(x: i16, y: i16, color: u32, text: *const c_char);
    pub fn RBR_DrawTextOverRsfPractice(x: i16, y: i16, color: u32, text: *const c_char);
    pub fn RBR_DrawMenuText(x: i16, y: i16, text: *const c_uchar);

    /*
     * game flow control functions.
     */
    pub fn RBR_CfgRace(racesetting: RBRRaceSetting);
    pub fn RBR_LoadRace();
    pub fn RBR_StartRace();
    pub fn RBR_FeedRaceState(racestate: RBRRaceState);
    pub fn RBR_FeedRaceData(racedata: RBRRaceData);
    pub fn RBR_FeedRaceResult(raceresult: RBRRaceResult);

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
    pub fn RBR_ReadCarLook() -> D3DQuaternion;
    pub fn RBR_ReadCarPos() -> D3DQuaternion;
}
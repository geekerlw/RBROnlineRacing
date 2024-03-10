use libc::{c_char, c_void};

#[link(name = "RBRHacker", kind = "static")]
extern "C" {
    pub fn RBR_InitPlugin(rbrgame: *mut c_void) -> *mut c_void;
    pub fn RBR_SetInitialize(func: extern "C" fn() -> *const c_char);
    //pub fn RBR_SetOnEndScene(func: extern "C" fn());
    pub fn RBR_EnableLeaderBoard();
    pub fn RBR_CfgLeaderBoardPos(posx: u16, posy: u16);
    pub fn RBR_CfgLeaderBoardStyle(LeaderBriefColor: *const c_char, LeaderBackGroundColor: *const c_char);
    pub fn RBR_EnableProgressBar();
    pub fn RBR_CfgProgressBarPos(posx: u16, posy: u16);
    pub fn RBR_CfgProgressBarStyle(ProgressBarBackColor: *const c_char, ProgressBarSplitColor: *const c_char, ProgressBarPointerColor: *const c_char);
    pub fn RBR_CfgProfileStyle(UserColor1: *const c_char, UserColor2: *const c_char);
    //pub fn RBR_ShowMessage(x: f32, y: f32, ptxtMessage: *const c_char, fTimeToDisplay: f32);
    //pub fn RBR_ShowText(x: f32, y: f32, ptxtText: *const c_char);
}
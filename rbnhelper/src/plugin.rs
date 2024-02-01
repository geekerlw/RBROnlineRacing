use libc::c_char;

#[repr(C)]
pub enum ERBRTyreTypes {
    TypeTarmacDry           = 0,
    TypeTarmacInterMediate  = 1,
    TypeTarmacWet           = 2,
    TypeGravelDry           = 3,
    TypeGravelInterMediate  = 4,
    TypeGravelWet           = 5,
    TypeSnow                = 6,
}

#[repr(C)]
pub enum ERBRWeatherType {
    WeatherGood     = 0,
    WeatherRandom   = 1,
    WeatherBad      = 2,
}

#[repr(C)]
pub enum ERBRGfxDrawFlags {
    GfxDrawCenterX      = 0x00000001,
    GfxDrawCenterY      = 0x00000002,
    GfxDrawAlignRight   = 0x00000004,
    GfxDrawAlignBottom  = 0x00000008,
    GfxDrawFlipX        = 0x00000010,
    GfxDrawFlipY        = 0x00000020,
    GfxDrawTextShadow   = 0x00000040,
}

#[repr(C)]
pub enum ERBRFontSize {
    FontSmall   = 0,
    FontBig     = 1,
    FontDebug   = 2,
    FontHeading = 3,
}

#[repr(C)]
pub enum ERBRMenuColors {
    MenuBkground    = 0,
    MenuSelection   = 1,
    MenuIcon        = 2,
    MenuText        = 3,
    MenuHeading     = 4,
}

#[repr(C)]
pub enum ERBRGameLanguage {
    GameLangEnglish     = 0,
    GameLangFrench      = 1,
    GameLangGerman      = 2,
    GameLangSpanish     = 3,
    GameLangItalian     = 4,
    GameLangCzech       = 5,
    GameLangPolish      = 6,
}

#[allow(non_snake_case)]
pub trait IPlugin {
    fn GetName(&self) -> &str;
    fn DrawFrontEndPage(&self) {}
    fn DrawResultUI(&self) {}
    fn HandleFrontEndEvents(&self, _txt: char, _up: bool, _down: bool, _left: bool, _right: bool, _select: bool) {}
    fn TickFrontEndPage(&self, _time: f32) {}
    fn StageStarted(&self, _mapid: i32, _player: *const c_char, _falsestart: bool) {}
    fn HandleResults(&self, _checkpoint1: f32, _checkpoint2: f32, _finishtime: f32, _player: *const c_char) {}
    fn CheckPoint(&self, _checkpointtime: f32, _checkpointid: i32, _player: *const c_char) {}
}

#[allow(non_snake_case)]
#[repr(C)]
pub struct RBRGame {
    StartGame: extern "C" fn(mapid: i32, carid: i32, weather: ERBRWeatherType, tyre: ERBRTyreTypes, setup: *const c_char) -> bool,
}
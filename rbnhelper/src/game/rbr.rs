use std::path::PathBuf;
use std::ffi::CString;
use rbnproto::httpapi::{RaceConfig, RaceInfo, RaceState};
use rbnproto::metaapi::{MetaRaceData, MetaRaceProgress, MetaRaceResult, MetaRaceState};
use ini::Ini;
use rbnproto::rsfdata::{RBRRaceData, RBRRaceResult, RBRRaceSetting, RBRRaceState};
use super::hacker::*;

#[derive(Debug, Default)]
pub struct RBRGame;

impl RBRGame {
    pub fn cfg_dashboard_style(&mut self, conf: PathBuf) {
        if let Ok(conf) = Ini::load_from_file(conf) {
            let enable_leader = conf.get_from_or(Some("Setting"), "LeaderEnable", "false");
            match enable_leader {
                "true" => {
                    unsafe {
                        RBR_EnableLeaderBoard();
                        let posx = conf.get_from_or(Some("Pos"), "LeaderBoardPosX", "20").parse().unwrap();
                        let posy = conf.get_from_or(Some("Pos"), "LeaderBoardPosY", "100").parse().unwrap();
                        RBR_CfgLeaderBoardPos(posx, posy);

                        let briefcolor = CString::new(conf.get_from_or(Some("Color"), "LeaderBriefColor", "0xFFFF00FF")).unwrap();
                        let groundcolor = CString::new(conf.get_from_or(Some("Color"), "LeaderBackGroundColor", "0xFFFFFF1F")).unwrap();
                        RBR_CfgLeaderBoardStyle(briefcolor.as_ptr(), groundcolor.as_ptr());
                    };
                },
                _ => {},
            };
            let enable_progress = conf.get_from_or(Some("Setting"), "ProgressEnable", "false");
            match enable_progress {
                "true" => {
                    unsafe {
                        RBR_EnableProgressBar();
                        let posx = conf.get_from_or(Some("Pos"), "ProgressBarPosX", "40").parse().unwrap();
                        let posy = conf.get_from_or(Some("Pos"), "ProgressBarPosY", "300").parse().unwrap();
                        RBR_CfgProgressBarPos(posx, posy);
                        let backcolor = CString::new(conf.get_from_or(Some("Color"), "ProgressBarBackColor", "0xFFFFFFFF")).unwrap();
                        let splitcolor = CString::new(conf.get_from_or(Some("Color"), "ProgressBarSplitColor", "0x00FF00FF")).unwrap();
                        let pointercolor = CString::new(conf.get_from_or(Some("Color"), "ProgressBarPointerColor", "0x00FF00FF")).unwrap();
                        RBR_CfgProgressBarStyle(backcolor.as_ptr(), splitcolor.as_ptr(), pointercolor.as_ptr());
                    };
                }
                _ => {},
            }

            if enable_leader.eq("true") || enable_progress.eq("true") {
                unsafe {
                    let color1 = CString::new(conf.get_from_or(Some("Color"), "UserColor1", "0xFF0000FF")).unwrap();
                    let color2 = CString::new(conf.get_from_or(Some("Color"), "UserColor2", "0x00FF00FF")).unwrap();
                    RBR_CfgProfileStyle(color1.as_ptr(), color2.as_ptr());
                    let dashfontsize: i32 = conf.get_from_or(Some("Font"), "DashFontSize", "16").parse().unwrap();
                    let textfontsize: i32 = conf.get_from_or(Some("Font"), "TextFontSize", "14").parse().unwrap();

                    let scale_factor = self.get_system_scaling_factor();
                    RBR_CfgFontSize(dashfontsize * scale_factor as i32, textfontsize * scale_factor as i32);
                }
            }
        }
    }

    pub fn config(&mut self, info: &RaceInfo) {
        unsafe { RBR_CfgRace(RBRRaceSetting::from(info, &RaceConfig::default())); }
    }

    pub fn load(&mut self) {
        unsafe { RBR_LoadRace(); };
    }

    pub fn start(&mut self) {
        unsafe { RBR_StartRace(); };
    }

    pub fn game_mode(&mut self) -> i32 {
        unsafe { RBR_ReadGameMode() }
    }

    pub fn get_user(&mut self) -> String {
        let default_user = "anonymous".to_string();
        if let Some(game_path) = std::env::current_exe().unwrap().parent() {
            let conf_path = game_path.join("rallysimfans.ini");
            if let Ok(conf) = Ini::load_from_file(conf_path) {
                return conf.get_from_or(Some("login"), "name", default_user.as_str()).to_string();
            }
        }
        default_user
    }

    pub fn get_race_state(&mut self) -> RaceState {
        let mut state = RaceState::RaceDefault;
        unsafe {
            let game_mode = RBR_ReadGameMode();
            let start_count = RBR_ReadRaceStartCount();
            let track_load_state = RBR_ReadTrackLoadState();
            if game_mode == 0x01 && start_count < 0f32 {
                state = RaceState::RaceRunning;
            } else if game_mode == 0x05 {
                state = RaceState::RaceLoading;
            } else if game_mode == 0x0A && track_load_state == 0x08 && start_count == 7f32 {
                state = RaceState::RaceLoaded;
            } else if game_mode == 0x09 {
                state = RaceState::RaceFinished;
            } else if game_mode == 0x0C {
                state = RaceState::RaceExitMenu;
            }
        };
        state
    }

    pub fn get_race_data(&mut self) -> MetaRaceData {
        let mut data = MetaRaceData::default();
        unsafe {
            data.speed = RBR_ReadCarSpeed();
            data.racetime = RBR_ReadCarRaceTime();
            data.progress = RBR_ReadCarStageProgress();
            data.stagelen = RBR_ReadStageLen();
            data.splittime1 = RBR_ReadSplitTime1();
            data.splittime2 = RBR_ReadSplitTime2();
            data.finishtime = RBR_ReadFinishTime();
            data.carlook = RBR_ReadCarLook();
            data.carpos = RBR_ReadCarPos();
        }
        data
    }

    pub fn feed_race_state(&mut self, result: &Vec<MetaRaceState>) {
        let racestate = RBRRaceState::from_result(result);
        unsafe { RBR_FeedRaceState(racestate) };
    }

    pub fn feed_race_data(&mut self, result: &Vec<MetaRaceProgress>) {
        let racedata = RBRRaceData::from_result(result);
        unsafe { RBR_FeedRaceData(racedata) };
    }

    pub fn feed_race_result(&mut self, result: &Vec<MetaRaceResult>) {
        let raceresult = RBRRaceResult::from_result(result);
        unsafe { RBR_FeedRaceResult(raceresult) };
    }

    pub fn fast_set_race_stage(&mut self, stage_id: &u32) {
        if let Some(game_path) = std::env::current_exe().unwrap().parent() {
            let recent_filepath = game_path.join("rsfdata").join("cache").join("recent.ini");
            if let Ok(mut conf) = Ini::load_from_file(&recent_filepath) {
                conf.with_section(Some("PracticeStage")).set("id", stage_id.to_string());
                conf.write_to_file(recent_filepath).unwrap();
            }
        }
    }

    #[allow(dead_code)]
    pub fn fast_set_race_car(&mut self, car_id: &u32) {
        if let Some(game_path) = std::env::current_exe().unwrap().parent() {
            let recent_filepath = game_path.join("rsfdata").join("cache").join("recent.ini");
            if let Ok(mut conf) = Ini::load_from_file(&recent_filepath) {
                conf.with_section(Some("PracticeCar")).set("id", car_id.to_string());
                conf.write_to_file(recent_filepath).unwrap();
            }
        }
    }

    pub fn fast_set_race_car_damage(&mut self, damage: &u32) {
        if let Some(game_path) = std::env::current_exe().unwrap().parent() {
            let conf_path = game_path.join("\rallysimfans.ini");
            if let Ok(mut conf) = Ini::load_from_file(&conf_path) {
                conf.with_section(Some("drive")).set("practice_damage", damage.to_string());
                conf.write_to_file(conf_path).unwrap();
            }
        }
    }

    #[cfg(target_os = "windows")]
    fn get_system_scaling_factor(&self) -> f32 {
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
    fn get_system_scaling_factor(&self) -> f32 {
        // 在非Windows平台上返回默认值
        1.0
    }
}
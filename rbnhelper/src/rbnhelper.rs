use libc::{c_char, c_void};
use log::info;
use crate::plugin::IPlugin;
use crate::hacker::*;
use ini::Ini;

#[derive(Default, Clone, Copy)]
pub struct RBNHelper {
}

impl IPlugin for RBNHelper {
    fn GetName(&self) -> *const libc::c_char {
        let name = std::ffi::CString::new("RBN Helper").unwrap();
        name.into_raw()
    }
}

impl From<*mut c_void> for RBNHelper {
    fn from(value: *mut c_void) -> Self {
        let ptr = value as *mut RBNHelper;
        unsafe {*ptr}
    }
}

impl RBNHelper {
    pub fn init(&mut self) {
        self.load_dashboard_config();
    }

    fn load_dashboard_config(&mut self) {
        if let Some(game_path) = std::env::current_exe().unwrap().parent() {
            let conf_path = game_path.join("Plugins").join("RBNHelper").join("RBRDashBoard.ini");
            if let Ok(conf) = Ini::load_from_file(conf_path) {
                let enable_leader = conf.get_from_or(Some("Setting"), "LeaderEnable", "false");
                match enable_leader {
                    "true" => {
                        unsafe {
                            RBR_EnableLeaderBoard();
                            RBR_CfgLeaderBoardPos(
                            conf.get_from_or(Some("Pos"), "LeaderBoardPosX", "20").parse().unwrap(),
                            conf.get_from_or(Some("Pos"), "LeaderBoardPosY", "100").parse().unwrap()
                            );
                            RBR_CfgLeaderBoardStyle(
                                conf.get_from_or(Some("Color"), "LeaderBriefColor", "0xFFFF00FF").as_ptr() as *const c_char,
                                conf.get_from_or(Some("Color"), "LeaderBackGroundColor", "0xFFFFFF1F").as_ptr() as *const c_char,
                            );
                        };
                    },
                    _ => {},
                };
                let enable_progress = conf.get_from_or(Some("Setting"), "ProgressEnable", "false");
                match enable_progress {
                    "true" => {
                        unsafe {
                            RBR_EnableProgressBar();
                            RBR_CfgProgressBarPos(
                            conf.get_from_or(Some("Pos"), "ProgressBarPosX", "40").parse().unwrap(),
                            conf.get_from_or(Some("Pos"), "ProgressBarPosY", "300").parse().unwrap()
                            );
                            RBR_CfgProgressBarStyle(
                                conf.get_from_or(Some("Color"), "ProgressBarBackColor", "0xFFFFFFFF").as_ptr() as *const c_char,
                                conf.get_from_or(Some("Color"), "ProgressBarSplitColor", "0x00FF00FF").as_ptr() as *const c_char,
                                conf.get_from_or(Some("Color"), "ProgressBarPointerColor", "0x00FF00FF").as_ptr() as *const c_char,
                            );
                        };
                    }
                    _ => {},
                }

                if enable_leader.eq("true") || enable_progress.eq("true") {
                    unsafe {
                        RBR_CfgProfileStyle(
                            conf.get_from_or(Some("Color"), "UserColor1", "0xFF0000FF").as_ptr() as *const c_char,
                            conf.get_from_or(Some("Color"), "UserColor2", "0x00FF00FF").as_ptr() as *const c_char,
                        );
                    }
                }
            }
        }
    }
}
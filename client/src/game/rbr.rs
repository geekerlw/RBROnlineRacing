use std::io::Read;
use unicode_normalization::UnicodeNormalization;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio::io::SeekFrom;
use core::time::Duration;
use std::thread::sleep;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::process::Command;
extern crate winapi;
use winapi::um::winuser::{FindWindowW, SetForegroundWindow, SendMessageW, WM_KEYDOWN, WM_KEYUP, VK_DOWN, VK_RETURN, VK_ESCAPE};
use protocol::httpapi::{RaceState, MetaRaceData, RaceInfo, MetaRaceResult};
use ini::Ini;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone)]
pub struct RBRGame {
    pub root_path: String,
    pub test_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RBRStageData {
    pub id: String,
    pub name: String,
    pub deftime: String,
    pub length: String,
    pub surface_id: String,
    pub stage_id: String,
    pub short_country: String,
    pub author: String,
    pub tarmac: String,
    pub gravel: String,
    pub snow: String,
    pub new_update: String,
    pub author_web: String,
    pub author_note: String,
    pub fattrib: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RBRCarData {
    pub id: String,
    pub name: String,
    pub path: String,
    pub hash: String,
    pub carmodel_id: String,
    pub user_id: String,
    pub base_group_id: String,
    pub test: String,
    pub ngp: String,
    pub custom_setups: String,
    pub rev: String,
    pub audio: Option<String>,
    pub audio_hash: String,
}

impl RBRGame {
    pub fn new(path: &String) -> Self {
        Self {
            root_path: path.clone(),
            test_count: 0,
        }
    }

    pub async fn launch(self) -> Self {
        let rbr_sse = self.root_path.clone() + r"\RichardBurnsRally_SSE.exe";
        let _process = Command::new(rbr_sse).current_dir(&self.root_path)
            .spawn().expect("failed to execute command");

        let target = "Automatic menu navigation completed\r\n";
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

            if let Ok(mut file) = tokio::fs::File::open(self.root_path.clone() + r"\Plugins\NGPCarMenu\NGPCarMenu.log").await {
                if file.metadata().await.unwrap().len() < 40 {
                    continue;
                }
                file.seek(SeekFrom::End(-37)).await.unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).await.unwrap();
                if target == content.as_str() {
                    println!("RBR Game automic login complete.");
                    break;
                }
            }            
        };
        self
    }

    pub fn load(&mut self) {
        unsafe {
            // Find the handle of the target window
            let window_title = "Richard Burns Rally - DirectX9\0";
            let wide_title: Vec<u16> = OsStr::new(window_title).encode_wide().chain(once(0)).collect();
            let window_handle = FindWindowW(std::ptr::null_mut(), wide_title.as_ptr());
    
            sleep(Duration::from_secs(1));

            // Simulate a special keyboard event (Down key)
            SetForegroundWindow(window_handle);
            SendMessageW(window_handle, WM_KEYDOWN, VK_DOWN as usize, 0);
            SendMessageW(window_handle, WM_KEYUP, VK_DOWN as usize, 0);
    
            sleep(Duration::from_secs(1));
    
            // Simulate a special keyboard event (Down key)
            SetForegroundWindow(window_handle);
            SendMessageW(window_handle, WM_KEYDOWN, VK_DOWN as usize, 0);
            SendMessageW(window_handle, WM_KEYUP, VK_DOWN as usize, 0);
    
            sleep(Duration::from_secs(1));
    
            // Simulate a special keyboard event (Enter key)
            SetForegroundWindow(window_handle);
            SendMessageW(window_handle, WM_KEYDOWN, VK_RETURN as usize, 0);
            SendMessageW(window_handle, WM_KEYUP, VK_RETURN as usize, 0);
    
            sleep(Duration::from_secs(1));
    
            // Simulate a special keyboard event (Enter key)
            SetForegroundWindow(window_handle);
            SendMessageW(window_handle, WM_KEYDOWN, VK_RETURN as usize, 0);
            SendMessageW(window_handle, WM_KEYUP, VK_RETURN as usize, 0);

        }
    }

    pub fn start(&mut self) {
        unsafe {
            // Find the handle of the target window
            let window_title = "Richard Burns Rally - DirectX9\0";
            let wide_title: Vec<u16> = OsStr::new(window_title).encode_wide().chain(once(0)).collect();
            let window_handle = FindWindowW(std::ptr::null_mut(), wide_title.as_ptr());
    
            // Simulate a special keyboard event (ESC key)
            SetForegroundWindow(window_handle);
            SendMessageW(window_handle, WM_KEYDOWN, VK_ESCAPE as usize, 0);
            SendMessageW(window_handle, WM_KEYUP, VK_ESCAPE as usize, 0);
        }
    }

    pub fn get_user(&mut self) -> String {
        let default_user = "anonymous".to_string();
        let conf_path = self.root_path.clone() + r"\rallysimfans.ini";
        if let Ok(conf) = Ini::load_from_file(conf_path) {
            return conf.get_from_or(Some("login"), "name", default_user.as_str()).to_string();
        }
        default_user
    }

    pub fn get_race_state(&mut self) -> RaceState {
        let mut state = RaceState::default();
        if self.test_count <= 10 {
            state = RaceState::RaceRunning;
        } else if self.test_count > 10 {
            state = RaceState::RaceFinished;
        } else {
            state = RaceState::default();
        }

        self.test_count += 1;
        return state;
    }

    pub fn get_race_data(&mut self) -> MetaRaceData {
        MetaRaceData::default()
    }

    pub fn set_race_data(&mut self, result: &MetaRaceResult) {
        
    }

    pub fn set_race_result(&mut self, result: &MetaRaceResult) {
        
    }

    pub fn set_race_info(&mut self, info: &RaceInfo) {
        let recent_filepath = self.root_path.clone() + r"\rsfdata\cache\recent.ini";
        if let Ok(mut conf) = Ini::load_from_file(&recent_filepath) {
            if let Some(car_id) = &info.car_id {
                conf.with_section(Some("PracticeCar")).set("id", car_id.to_string());
            }
            conf.with_section(Some("PracticeStage")).set("id", info.stage_id.to_string());
            conf.write_to_file(recent_filepath).unwrap();
        }
        let conf_path = self.root_path.clone() + r"\rallysimfans.ini";
        if let Ok(mut conf) = Ini::load_from_file(&conf_path) {
            conf.with_section(Some("drive")).set("practice_damage", info.damage.to_string());
            conf.write_to_file(conf_path).unwrap();
        }
    }

    pub fn load_game_stages(&mut self) -> Option<Vec<RBRStageData>> {
        let filepath = self.root_path.clone() + r"\rsfdata\cache\stages_data.json";
        if let Ok(mut file) = std::fs::File::open(filepath) {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            let bufstr = String::from_utf8_lossy(&buf).to_string().nfc().collect::<String>();
            if let Ok(mut stages) = serde_json::from_str::<Vec<RBRStageData>>(&bufstr) {
                stages.sort_by(|a, b| a.name.cmp(&b.name));
                return Some(stages);
            }
        }
        return None;
    }

    pub fn load_game_cars(&mut self) -> Option<Vec<RBRCarData>> {
        let filepath = self.root_path.clone() + r"\rsfdata\cache\cars.json";
        if let Ok(file) = std::fs::File::open(filepath) {
            if let Ok(mut cars) = serde_json::from_reader::<std::fs::File, Vec<RBRCarData>>(file) {
                cars.sort_by(|a, b| a.name.cmp(&b.name));
                return Some(cars);
            }
        }
        return None;
    }
}
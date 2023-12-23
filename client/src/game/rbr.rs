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
use winapi::um::winuser::{FindWindowW, SetForegroundWindow, SendMessageW, WM_KEYDOWN, WM_KEYUP, VK_DOWN, VK_RETURN, VK_ESCAPE};
use protocol::httpapi::{RaceState, MetaRaceData, RaceInfo, MetaRaceResult};
use ini::Ini;
use serde::{Serialize, Deserialize};
use process_memory::{Architecture, Memory, DataMember, Pid, ProcessHandleExt, TryIntoProcessHandle};

#[derive(Debug, Default, Clone)]
pub struct RBRGame {
    pub root_path: String,
    pub pid: u32,
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


// Offset 0x007EAC48 + 0x728 The current game mode (state machine of RBR)
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
struct RBRGameMode {
    reversed: [u8; 0x728],
    // gameMode
    //		00 = (not available)
    //		01 = driving (after 5secs or less left in start clock or already driving after GO! command)
    //		02 = pause (when a menu is shown while stage or replay is running, not the main menu)
    //		03 = main menu or plugin menu (stage not running)
    //		04 = ? (black out)
    //		05 = loading track (race or replay. When the track model is loaded the status goes briefly to 0x0D and when the countdown starts the status goes to 0x0A)
    //		06 = exiting to menu from a race or replay (after this the mode goes to 12 for a few secs and finally to 3 when the game is showing the main or plugin menu)
    //		07 = quit the application ?
    //		08 = replay
    //		09 = end lesson / finish race / retiring / end replay
    //      0A = Before starting a race or replay (camera is spinning around the car. At this point map and car model has been loaded and is ready to rock)
    //      0B = ? (black out)
    //      0C = Game is starting or racing ended and going back to main menu (loading the initial "Load Profile" screen or "RBR menu system". Status goes to 0x03 when the "Load Profile" menu is ready and shown)
    //      0D = (not available) (0x0D status after 0x05 map loading step is completed. After few secs the status goes to 0x0A and camera starts to spin around the car)
    //      0E = (not available) (status goes to 0x0F and then RBR crashes)
    //      0F = ? Doesnt work anymore. Goes to menu? Pause racing and replaying and hide all on-screen instruments and timers (supported only after the race or replay has started, ie 0x0A and 0x10 status has changed to 0x08 or 0x01)
    //		10-0xFF = ?
    game_mode: i32, // 0x728
}

// Offset 0x0165FC68 RBRCarInfo
#[repr(C, packed)]
struct RBRCarInfo {
    hud_position_x: i32,     // 0x00
    hud_position_y: i32,     // 0x04
    race_started: i32,       // 0x08 (1=Race started. Start countdown less than 5 secs, so false start possible, 0=Race not yet started or start countdown still more than 5 secs and gas pedal doesn't work yet)
    speed: f32, 			 // 0x0C
    rpm: f32,				 // 0x10
    temp: f32,				 // 0x14 (water temp in celsius?)
    turbo: f32, 			 // 0x18. (pressure, in Pascals?)
    unknown2: i32,  		 // 0x1C (always 0?)
    distance_from_start: f32, // 0x20
    distance_from_travelled: f32, // 0x24
    distance_to_finish: f32, // 0x28
    pad1: [u8; 0x110],
    stage_process: f32,      // 0x13C  (meters, hundred meters, some map unit?. See RBRMapInfo.stageLength also)
    race_time: f32,			 // 0x140  Total race time (includes time penalties)  (or if gameMode=8 then the time is taken from replay video)
    race_finished: i32,      // 0x144  (0=Racing after GO! command, 1=Racing completed/retired or not yet started
    unknown4: i32,          // 0x148
    unknown5: i32,          // 0x14C
    driving_direction: i32,  // 0x150. 0=Correct direction, 1=Car driving to wrong direction
    fade_wrongway_msg: f32,   // 0x154. 1 when "wrong way" msg is shown
    pad3: [u8; 0x18],
    gear: i32,  		     // 0x170. 0=Reverse,1=Neutral,2..6=Gear-1 (ie. value 3 means gear 2) (note! the current value only, gear is not set via this value)
    pad4: [u8; 0xD0],
    stage_start_countdown: f32, // 0x244 (7=Countdown not yet started, 6.999-0.1 Countdown running, 0=GO!, <0=Racing time since GO! command)
    false_start: i32,		   // 0x248 (0=No false start, 1=False start)
    pad5: [u8; 8],
    split_reached_num: i32,      // 0x254 0=Start line passed if race is on, 1=Split#1 passed, 2=Split#2 passed
    split1_time: f32,        // 0x258 Total elapsed time in secs up to split1
    split2_time: f32,        // 0x25C Total elapsed time in secs up to split2  (split2-split1 would be the time between split1 and split2)
}

// Fixed Offset 0x1660800 RBRMapSettings. Configuration of the next stage (Note! Need to set these values before the stage begins)
#[repr(C, packed)]
struct RBRMapSettings {
    unknown1: i32,   // 0x00
    track_id: i32,	// 0x04   (xx trackID)
    car_id: i32,		// 0x08   (0..7 carID)
    unknown2: i32,   // 0x0C
    unknown3: i32,   // 0x10
    transmission_type: i32, // 0x14  (0=Manual, 1=Automatic)
    pad1: [u8; 0x18],
    race_paused: i32, // 0x30 (0=Normal mode, 1=Racing in paused state)
    pad2: i32,
    tyre_type: i32,	// 0x38 (0=Dry tarmac, 1=Intermediate tarmac, 2=Wet tarmac, 3=Dry gravel, 4=Inter gravel, 5=Wet gravel, 6=Snow)
    pad3: [u8; 0xC],
    weather_type: i32,	// 0x48   (0=Good, 1=Random, 2=Bad)
    unknown4: i32,       // 0x4C
    damage_type: i32,		// 0x50   (0=No damage, 1=Safe, 2=Reduced, 3=Realistic)
    pacecar_enabled: i32 // 0x54   (0=Pacecar disabled, 1=Pacecar enabled)
}
    
// Fixed offset 0x8938F8. Additional map settings 
#[repr(C, packed)]
struct RBRMapSettingsEx {
    unknown1: i32,		// 0x00
    unknown2: i32,		// 0x04
    track_id: i32,		// 0x08
    unknown3: i32,		// 0x0C
    sky_cloud_type: i32,	// 0x10 (0=Clear, 1=PartCloud, 2=LightCloud, 3=HeavyCloud)
    surface_wetness: i32, // 0x14 (0=Dry, 1=Damp, 2=Wet)
    surface_age: i32,     // 0x18 (0=New, 1=Normal, 2=Worn)
    pad1: [u8; 0x1C],
    timeofday: i32,		// 0x38 (0=Morning, 1=Noon, 2=Evening)
    sky_type: i32,		// 0x3C (0=Crisp, 1=Hazy, 2=NoRain, 3=LightRain, 4=HeavyRain, 5=NoSnow, 6=LightSnow, 7=HeavySnow, 8=LightFog, 9=HeavyFog)
}


impl RBRGame {
    pub fn new(path: &String) -> Self {
        Self {
            root_path: path.clone(),
            pid: 0,
        }
    }

    pub async fn launch(&mut self) {
        let rbr_sse = self.root_path.clone() + r"\RichardBurnsRally_SSE.exe";
        let process = Command::new(rbr_sse).current_dir(&self.root_path)
            .spawn().expect("failed to execute command");
        self.pid = process.id();

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
        let handle = (self.pid as Pid).try_into_process_handle().unwrap().set_arch(Architecture::Arch32Bit);
        let game_mode_addr = DataMember::<i32>::new_offset(handle, vec![0x7EAC48, 0x728]);
        let loading_mode_addr = DataMember::<i32>::new_offset(handle, vec![0x7EA678, 0x70, 0x10]);
        let game_mode: i32 = unsafe {game_mode_addr.read().unwrap()};
        let loading_mode: i32 = unsafe {loading_mode_addr.read().unwrap()};
        if game_mode == 0x01 {
            return RaceState::RaceRunning;
        } else if game_mode == 0x0A && loading_mode == 0x08 {
            return RaceState::RaceLoaded;
        } else if game_mode == 0x0C {
            return RaceState::RaceFinished;
        }
        return RaceState::RaceDefault;
    }

    pub fn get_race_data(&mut self) -> MetaRaceData {
        let mut data = MetaRaceData::default();
        let handle = (self.pid as Pid).try_into_process_handle().unwrap().set_arch(Architecture::Arch32Bit);
        let racetime_addr = DataMember::<f32>::new_offset(handle, vec![0x165FC68, 0x140]);
        let process_addr = DataMember::<f32>::new_offset(handle, vec![0x165FC68, 0x13C]);
        let split1_addr = DataMember::<f32>::new_offset(handle, vec![0x165FC68, 0x258]);
        let split2_addr = DataMember::<f32>::new_offset(handle, vec![0x165FC68, 0x25C]);
        let finished_addr = DataMember::<i32>::new_offset(handle, vec![0x165FC68, 0x2C4]);

        unsafe {
            data.racetime = racetime_addr.read().unwrap();
            data.process = process_addr.read().unwrap();
            data.splittime1 = split1_addr.read().unwrap();
            data.splittime2 = split2_addr.read().unwrap();
            if finished_addr.read().unwrap() == 1 {
                data.finishtime = racetime_addr.read().unwrap();
            } else {
                data.finishtime = 9999.0;
            }
        }

        return data;
    }

    pub fn set_race_data(&mut self, result: &Vec<MetaRaceResult>) {
        
    }

    pub fn set_race_result(&mut self, result: &Vec<MetaRaceResult>) {
        
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
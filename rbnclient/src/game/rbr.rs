use std::io::Read;
use std::os::windows::process::CommandExt;
use rbnproto::rsfdata::{RBRCarData, RBRRaceData, RBRRaceSetting, RBRStageData, RBRStageWeather};
use unicode_normalization::UnicodeNormalization;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio::io::SeekFrom;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::process::Command;
use winapi::um::winuser::{FindWindowW, SetForegroundWindow, SendMessageW, WM_KEYDOWN, WM_KEYUP, VK_ESCAPE};
use rbnproto::httpapi::{RaceState, RaceInfo, RaceConfig};
use rbnproto::metaapi::{MetaRaceData, MetaRaceProgress};
use ini::Ini;
use process_memory::{Architecture, Memory, DataMember, Pid, ProcessHandleExt, TryIntoProcessHandle};
use tokio::net::UdpSocket;
use log::info;

#[derive(Debug, Default)]
pub struct RBRGame {
    pub root_path: String,
    pub pid: u32,
    pub udp: Option<UdpSocket>,
}

impl RBRGame {
    pub fn new(path: &String) -> Self {
        Self {
            root_path: path.clone(),
            pid: 0,
            udp: None,
        }
    }

    pub async fn open_udp(mut self) -> Self {
        let sock = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let remote_addr = "127.0.0.1:5555";
        sock.connect(remote_addr).await.unwrap();
        self.udp = Some(sock);
        self
    }

    pub fn attach(&mut self) -> bool {
        let process_name = r"RichardBurnsRally_SSE.exe";
        let output = Command::new("tasklist")
        .args(&["/FO", "CSV", "/NH"])
        .creation_flags(0x08000000)
        .output()
        .expect("Failed to execute command");

        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            let fields: Vec<&str> = line.split(',').collect();
            if let Some(name) = fields.get(0) {
                if name.trim_matches('"') == process_name {
                    self.pid = fields.get(1).unwrap().trim().trim_matches('"').parse::<u32>().unwrap();
                    return true;
                }
            }
        }

        return false;
    }

    pub async fn launch(&mut self) {
        if self.attach() {
            return;
        }

        let rbr_sse = self.root_path.clone() + r"\RichardBurnsRally_SSE.exe";
        let process = Command::new(rbr_sse).current_dir(&self.root_path)
            .creation_flags(0x08000000)
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
                    info!("RBR Game automic login complete.");
                    break;
                }
            }
        };
    }

    pub async fn enter_practice(&mut self, raceinfo: &RaceInfo, racecfg: &RaceConfig) {
        info!("enter practice with raceinfo {:?}", raceinfo);
        info!("enter practice with racecfg {:?}", racecfg);
        self.set_race_config(raceinfo, racecfg).await;
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
        let startcount_addr = DataMember::<f32>::new_offset(handle, vec![0x165FC68, 0x244]);
        let game_mode: i32 = unsafe {game_mode_addr.read().unwrap()};
        let loading_mode: i32 = unsafe {loading_mode_addr.read().unwrap()};
        let start_count: f32 = unsafe {startcount_addr.read().unwrap()};
        if game_mode == 0x01 && start_count < 0f32 {
            return RaceState::RaceRunning;
        } else if game_mode == 0x0A && loading_mode == 0x08 && start_count == 7f32 {
            return RaceState::RaceLoaded;
        } else if game_mode == 0x0C {
            return RaceState::RaceFinished;
        }
        return RaceState::RaceDefault;
    }

    pub fn get_race_data(&mut self) -> MetaRaceData {
        let mut data = MetaRaceData::default();
        let handle = (self.pid as Pid).try_into_process_handle().unwrap().set_arch(Architecture::Arch32Bit);
        let speed_addr = DataMember::<f32>::new_offset(handle, vec![0x165FC68, 0x0C]);
        let racetime_addr = DataMember::<f32>::new_offset(handle, vec![0x165FC68, 0x140]);
        let progress_addr = DataMember::<f32>::new_offset(handle, vec![0x165FC68, 0x13C]);
        let stagelen_addr = DataMember::<i32>::new_offset(handle, vec![0x1659184, 0x75310]);
        let split1_addr = DataMember::<f32>::new_offset(handle, vec![0x165FC68, 0x258]);
        let split2_addr = DataMember::<f32>::new_offset(handle, vec![0x165FC68, 0x25C]);
        let line_finished_addr = DataMember::<i32>::new_offset(handle, vec![0x165FC68, 0x2C4]);

        unsafe {
            data.speed = speed_addr.read().unwrap();
            data.racetime = racetime_addr.read().unwrap();
            data.progress = progress_addr.read().unwrap();
            data.stagelen = stagelen_addr.read().unwrap() as f32;
            data.splittime1 = split1_addr.read().unwrap();
            data.splittime2 = split2_addr.read().unwrap();
            if line_finished_addr.read().unwrap() == 1 {
                data.finishtime = racetime_addr.read().unwrap();
            } else {
                data.finishtime = 3600.0;
            }
        }

        return data;
    }

    pub fn prepare_game_env(&mut self, info: &RaceInfo, cfg: &RaceConfig) {
        self.set_race_stage(&info.stage_id);
        self.set_race_car_damage(&info.damage);
        let default_setup = info.car_id.to_string() + "_d_" + info.stage_type.to_lowercase().as_str();
        if info.car_fixed {
            self.set_race_car(&info.car_id);
            self.set_race_car_setup(&info.car_id, &info.stage_type.to_lowercase().as_str(), &default_setup);
        } else {
            self.set_race_car(&cfg.car_id);
            if cfg.setup == "default" {
                self.set_race_car_setup(&cfg.car_id, &info.stage_type.to_lowercase().as_str(), &default_setup);
            } else {
                self.set_race_car_setup(&cfg.car_id, &info.stage_type.to_lowercase().as_str(), &cfg.setup);
            }
        }
    }

    pub async fn set_race_config(&mut self, info: &RaceInfo, cfg: &RaceConfig) {
        if let Some(udp) = &self.udp {
            let buf = RBRRaceSetting::from(info, cfg).as_bytes();
            udp.send(&buf).await.unwrap();
        }
    }

    pub async fn set_race_data(&mut self, result: &Vec<MetaRaceProgress>) {
        if let Some(udp) = &self.udp {
            let buf = RBRRaceData::from_result(result).as_bytes();
            udp.send(&buf).await.unwrap();
        }
    }

    pub fn set_race_stage(&mut self, stage_id: &u32) {
        let recent_filepath = self.root_path.clone() + r"\rsfdata\cache\recent.ini";
        if let Ok(mut conf) = Ini::load_from_file(&recent_filepath) {
            conf.with_section(Some("PracticeStage")).set("id", stage_id.to_string());
            conf.write_to_file(recent_filepath).unwrap();
        }
    }

    pub fn set_race_car(&mut self, car_id: &u32) {
        let recent_filepath = self.root_path.clone() + r"\rsfdata\cache\recent.ini";
        if let Ok(mut conf) = Ini::load_from_file(&recent_filepath) {
            conf.with_section(Some("PracticeCar")).set("id", car_id.to_string());
            conf.write_to_file(recent_filepath).unwrap();
        }
    }

    pub fn set_race_car_damage(&mut self, damage: &u32) {
        let conf_path = self.root_path.clone() + r"\rallysimfans.ini";
        if let Ok(mut conf) = Ini::load_from_file(&conf_path) {
            conf.with_section(Some("drive")).set("practice_damage", damage.to_string());
            conf.write_to_file(conf_path).unwrap();
        }
    }

    pub fn set_race_car_setup(&mut self, car_id: &u32, surface: &str, setup: &String) {
        let personal_filepath = self.root_path.clone() + r"\rallysimfans_personal.ini";
        if let Ok(mut file) = std::fs::File::open(&personal_filepath) {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            let bufstr = String::from_utf8_lossy(&buf).to_string().nfc().collect::<String>();
        
            if let Ok(mut conf) = Ini::load_from_str(&bufstr) {
                let section = "car".to_string() + car_id.to_string().as_str();
                conf.with_section(Some(&section)).set("setup".to_owned() + surface, setup);
                conf.write_to_file(personal_filepath).unwrap();
            }
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

    pub fn load_game_stage_weathers(&mut self, stage_id: &u32) -> Option<Vec<RBRStageWeather>> {
        let filepath = self.root_path.clone() + r"\rsfdata\cache\stages_tracksettings.json";
        if let Ok(file) = std::fs::File::open(filepath) {
            if let Ok(mut weathers) = serde_json::from_reader::<std::fs::File, Vec<RBRStageWeather>>(file) {
                weathers.retain(|x| &x.stage_id.parse().unwrap_or(0u32) == stage_id);
                weathers.sort_by(|a, b| a.get_weight().cmp(&b.get_weight()));
                if !weathers.is_empty() {
                    return Some(weathers);
                }
            }
        }
        None
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

    pub fn load_game_car_setups(&mut self, path: &String) -> Option<Vec<String>> {
        let folder_path = self.root_path.clone() + r"\SavedGames\" + path;
        if let Ok(entrys) = std::fs::read_dir(folder_path) {
            let mut setups = vec![];
            for entry in entrys {
                if let Ok(entry) = entry {
                    if entry.metadata().unwrap().is_file() && entry.file_name().to_string_lossy().ends_with(".lsp") {
                        setups.push(entry.file_name().to_string_lossy().trim_end_matches(".lsp").to_string());
                    }
                }
            }
            return Some(setups);
        }

        None
    }
}
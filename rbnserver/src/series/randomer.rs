use std::{io::Read, path::PathBuf};
use unicode_normalization::UnicodeNormalization;
use rand::{thread_rng, Rng};
use rbnproto::{httpapi::RaceInfo, rsfdata::{RBRCarData, RBRStageData, RBRStageWeather}};


pub struct RaceRandomer {
    rsfdata_path: PathBuf,
    pub stages: Vec<RBRStageData>,
    pub wetness: Vec<&'static str>,
    pub weathers: Vec<&'static str>,
    pub skytypes: Vec<RBRStageWeather>,
    pub cars: Vec<RBRCarData>,
    pub damages: Vec<&'static str>,
    fixed_stage: bool,
    fixed_weather: bool,
    fixed_car: bool,
    fixed_damage: bool,
    raceinfo: RaceInfo,
}

impl Default for RaceRandomer {
    fn default() -> Self {
        Self { 
            rsfdata_path: std::env::current_exe().unwrap().parent().unwrap().join("rsfdata"),
            stages: vec![],
            wetness: vec!["Dry", "Damp", "Wet"],
            weathers: vec!["Good", "Random", "Bad"],
            skytypes: vec![],
            cars: vec![],
            damages: vec!["Off", "Safe", "Reduced", "Realistic"],
            fixed_stage: false,
            fixed_weather: false,
            fixed_car: false,
            fixed_damage: false,
            raceinfo: RaceInfo::default(),
        }
    }
}

#[allow(dead_code)]
impl RaceRandomer {
    pub fn build() -> Self {
        let mut randomer = Self::default();
        randomer.load_game_stages();
        randomer.load_game_cars();
        randomer
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.raceinfo.name = name;
        self
    }

    pub fn with_owner(mut self, owner: String) -> Self {
        self.raceinfo.owner = owner;
        self
    }

    pub fn fixed_stage(mut self, stage: String) -> Self {
        let mut select_stage = 0;
        for (i, item) in self.stages.iter().enumerate() {
            if item.name == stage {
                select_stage = i;
                break;
            }
        }
        self.raceinfo.stage = self.stages[select_stage].name.clone();
        self.raceinfo.stage_id = self.stages[select_stage].stage_id.parse().unwrap();
        self.raceinfo.stage_type = self.stages[select_stage].get_surface();
        self.raceinfo.stage_len = self.stages[select_stage].length.parse().unwrap();
        self.fixed_stage = true;
        self
    }

    pub fn fixed_weather(mut self) -> Self {
        self.raceinfo.weather = 0u32;
        self.raceinfo.wetness = 0u32;
        self.raceinfo.skytype = "Default".to_string();
        self.raceinfo.skytype_id = 0u32;
        self.fixed_weather = true;
        self
    }

    pub fn fixed_car(mut self, car: String) -> Self {
        let mut select_car = 0;
        for (i, item) in self.cars.iter().enumerate() {
            if item.name == car {
                select_car = i;
                break;
            }
        }
        self.raceinfo.car_fixed = true;
        self.raceinfo.car = self.cars[select_car].name.clone();
        self.raceinfo.car_id = self.cars[select_car].id.parse().unwrap();
        self.fixed_car = true;
        self
    }

    pub fn fixed_damage(mut self, damage: u32) -> Self {
        self.raceinfo.damage = damage;
        self.fixed_damage = true;
        self
    }

    fn load_game_stages(&mut self) {
        let filepath = self.rsfdata_path.clone().join("stages_data.json");
        if let Ok(mut file) = std::fs::File::open(filepath) {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            let bufstr = String::from_utf8_lossy(&buf).to_string().nfc().collect::<String>();
            if let Ok(mut stages) = serde_json::from_str::<Vec<RBRStageData>>(&bufstr) {
                stages.sort_by(|a, b| a.name.cmp(&b.name));
                self.stages = stages;
            }
        }
    }

    fn load_game_stage_weathers(&mut self, stage_id: &u32) -> Option<Vec<RBRStageWeather>> {
        let filepath = self.rsfdata_path.clone().join("stages_tracksettings.json");
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

    fn load_game_cars(&mut self) {
        let filepath = self.rsfdata_path.clone().join("cars.json");
        if let Ok(file) = std::fs::File::open(filepath) {
            if let Ok(mut cars) = serde_json::from_reader::<std::fs::File, Vec<RBRCarData>>(file) {
                cars.sort_by(|a, b| a.name.cmp(&b.name));
                self.cars = cars;
            }
        }
    }

    pub fn random(&mut self) -> RaceInfo {
        let select_stage = thread_rng().gen_range(0..self.stages.len());
        let select_wetness = thread_rng().gen_range(0..self.wetness.len());
        let select_weather = thread_rng().gen_range(0..self.weathers.len());
        let select_car = thread_rng().gen_range(0..self.cars.len());
        let select_damage = thread_rng().gen_range(0..self.damages.len());
        let mut skytype = "Default".to_string();
        let mut select_skytype = 0 as usize;
        let stage_id: u32 = self.stages[select_stage].stage_id.parse().unwrap();
        if let Some(weathers) = self.load_game_stage_weathers(&stage_id) {
            self.skytypes = weathers;
            select_skytype = thread_rng().gen_range(0..self.skytypes.len());
            skytype = self.skytypes[select_skytype].get_weather_string();
        }

        if !self.fixed_stage {
            self.raceinfo.stage = self.stages[select_stage].name.clone();
            self.raceinfo.stage_id = self.stages[select_stage].stage_id.parse().unwrap();
            self.raceinfo.stage_type = self.stages[select_stage].get_surface();
            self.raceinfo.stage_len = self.stages[select_stage].length.parse().unwrap();
        }

        if !self.fixed_weather {
            self.raceinfo.weather = select_weather as u32;
            self.raceinfo.wetness = select_wetness as u32;
            self.raceinfo.skytype = skytype;
            self.raceinfo.skytype_id = select_skytype as u32;
        }

        if !self.fixed_car {
            self.raceinfo.car_fixed = false;
            self.raceinfo.car = self.cars[select_car].name.clone();
            self.raceinfo.car_id = self.cars[select_car].id.parse().unwrap();
        }
        
        if !self.fixed_damage {
            self.raceinfo.damage = select_damage as u32;
        }

        self.raceinfo.clone()
    }
}
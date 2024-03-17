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
        }
    }
}

impl RaceRandomer {
    pub fn build() -> Self {
        let mut randomer = Self::default();
        randomer.load_game_stages();
        randomer.load_game_cars();
        randomer
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
        let mut raceinfo = RaceInfo::default();
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

        raceinfo.stage = self.stages[select_stage].name.clone();
        raceinfo.stage_id = self.stages[select_stage].stage_id.parse().unwrap();
        raceinfo.stage_type = self.stages[select_stage].get_surface();
        raceinfo.stage_len = self.stages[select_stage].length.parse().unwrap();
        raceinfo.car_fixed = false;
        raceinfo.car = self.cars[select_car].name.clone();
        raceinfo.car_id = self.cars[select_car].id.parse().unwrap();
        raceinfo.damage = select_damage as u32;
        raceinfo.weather = select_weather as u32;
        raceinfo.wetness = select_wetness as u32;
        raceinfo.skytype = skytype;
        raceinfo.skytype_id = select_skytype as u32;

        raceinfo
    }
}
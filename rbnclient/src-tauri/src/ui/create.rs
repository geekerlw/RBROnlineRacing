use eframe::egui;
use egui::Grid;
use egui::ComboBox;
use egui::RichText;
use egui::containers::popup::popup_below_widget;
use protocol::httpapi::RaceCreate;
use protocol::httpapi::RaceInfo;
use reqwest::StatusCode;
use super::{UiView, UiPageCtx, UiMsg};
use crate::game::rbr::RBRStageWeather;
use crate::game::rbr::{RBRGame, RBRStageData, RBRCarData};
use crate::ui::UiPageState;
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct UiCreateRace {
    pub room_name: String,
    pub room_passwd: String,
    pub stages: Vec<RBRStageData>,
    pub select_stage: usize,
    pub filter_stage: String,
    pub fixed_car: bool,
    pub cars: Vec<RBRCarData>,
    pub select_car: usize,
    pub filter_car: String,
    pub damages: Vec<&'static str>,
    pub select_damage: usize,
    pub wetness: Vec<&'static str>,
    pub select_wetness: usize,
    pub weathers: Vec<&'static str>,
    pub select_weather: usize,
    pub skytypes: Vec<RBRStageWeather>,
    pub select_skytype: usize,
}

impl Default for UiCreateRace {
    fn default() -> Self {
        Self { 
            room_name: "请输入房间名称".to_string(),
            room_passwd: String::new(),
            stages: vec![],
            select_stage: 246,
            filter_stage: String::from("Lyon - Gerland"),
            fixed_car: true,
            cars: vec![],
            select_car: 36,
            filter_car: String::from("Ford Fiesta WRC 2019"),
            damages: vec!["Off", "Safe", "Reduced", "Realistic"],
            select_damage: 3,
            wetness: vec!["Dry", "Damp", "Wet"],
            select_wetness: 0,
            weathers: vec!["Good", "Random", "Bad"],
            select_weather: 0,
            skytypes: vec![],
            select_skytype: 0,
        }
    }
}

impl UiView for UiCreateRace {
    fn init(&mut self, page: &mut UiPageCtx) {
        let mut rbr = RBRGame::new(&page.store.game_path);
        if let Some(stages) = rbr.load_game_stages() {
            self.stages = stages;
            let stage_id: u32 = self.stages[self.select_stage].stage_id.parse().unwrap();
            if let Some(skytypes) = rbr.load_game_stage_weathers(&stage_id) {
                self.skytypes = skytypes;
            }
        }
        if let Some(cars) = rbr.load_game_cars() {
            self.cars = cars;
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(300.0);
                ui.vertical(|ui| {
                    Grid::new("race create table")
                    .min_col_width(80.0)
                    .min_row_height(24.0)
                    .show(ui, |ui| {
                        ui.add_space(-20.0);
                        ui.label(RichText::new("房间设定").size(14.0));
                        ui.end_row();

                        ui.label("房间名称：");
                        ui.add_sized([200.0, 25.0], egui::TextEdit::singleline(&mut self.room_name));
                        ui.end_row();

                        ui.label("房间密码: ");
                        ui.add_sized([200.0, 25.0], egui::TextEdit::singleline(&mut self.room_passwd));

                        ui.end_row();

                        ui.add_space(-20.0);
                        ui.label(RichText::new("比赛设定").size(14.0));
                        ui.end_row();

                        ui.label("比赛赛道：");
                        ui.horizontal(|ui| {
                            let filter_stage = ui.add_sized([150.0, 25.0], egui::TextEdit::singleline(&mut self.filter_stage));
                            let popup_stage = ui.make_persistent_id("filter stage");
                            if filter_stage.changed() || filter_stage.clicked() {
                                ui.memory_mut(|mem| mem.open_popup(popup_stage));
                            }
                            popup_below_widget(ui, popup_stage, &filter_stage, |ui| {
                                let patten = self.filter_stage.clone().to_lowercase();
                                egui::ScrollArea::new([false, true]).max_height(240.0).show(ui, |ui| {
                                    for (index, stage) in self.stages.clone().iter().enumerate() {
                                        if stage.name.to_lowercase().contains(patten.as_str()) {
                                            if ui.selectable_label(self.select_stage == index, &stage.name).clicked() {
                                                self.filter_stage = stage.name.clone();
                                                self.select_stage = index;
                                                self.update_stage_weathers(page);
                                            }
                                        }
                                    }
                                });
                            });
                            ui.add_space(12.0);
                            if ui.button("随机").clicked() {
                                self.select_stage = thread_rng().gen_range(0..self.stages.len());
                                self.filter_stage = self.stages[self.select_stage].name.clone();
                                self.update_stage_weathers(page);
                            };
                        });
                        ui.end_row();

                        ui.label("车辆损坏：");
                        ComboBox::from_id_source("select damage").selected_text(self.damages[self.select_damage])
                        .show_ui(ui, |ui| {
                            for (index, damage) in self.damages.iter().enumerate() {
                                if ui.selectable_label(self.select_car == index, damage.to_string()).clicked() {
                                    self.select_damage = index;
                                }
                            }
                        });
                        ui.end_row();

                        ui.label("比赛车辆: ");
                        ui.horizontal(|ui| {
                            let filter_car = ui.add_sized([150.0, 25.0], egui::TextEdit::singleline(&mut self.filter_car));
                            let popup_car = ui.make_persistent_id("filter car");
                            if filter_car.changed() || filter_car.clicked() {
                                ui.memory_mut(|mem| mem.open_popup(popup_car));
                            }
                            popup_below_widget(ui, popup_car, &filter_car, |ui| {
                                let patten = self.filter_car.clone().to_lowercase();
                                egui::ScrollArea::new([false, true]).max_height(240.0).show(ui, |ui| {
                                    for (index, car) in self.cars.iter().enumerate() {
                                        if car.name.to_lowercase().contains(patten.as_str()) {
                                            if ui.selectable_label(self.select_car == index, &car.name).clicked() {
                                                self.filter_car = car.name.clone();
                                                self.select_car = index;
                                            }
                                        }
                                    }
                                });
                            });
                            ui.add_sized([25.0, 25.0], egui::Checkbox::new(&mut self.fixed_car, "限定"));
                        });
                        ui.end_row();

                        ui.add_space(-20.0);
                        ui.label(RichText::new("天气设定").size(14.0));
                        ui.end_row();

                        ui.label("湿滑情况：");
                        ComboBox::from_id_source("select wetness").selected_text(self.wetness[self.select_wetness])
                        .width(150.0)
                        .show_ui(ui, |ui| {
                            for (index, item) in self.wetness.iter().enumerate() {
                                if ui.selectable_label(self.select_wetness == index, item.to_string()).clicked() {
                                    self.select_wetness = index;
                                }
                            }
                        });
                        ui.end_row();

                        ui.label("天气状况：");
                        ComboBox::from_id_source("select weather").selected_text(self.weathers[self.select_weather])
                        .width(150.0)
                        .show_ui(ui, |ui| {
                            for (index, weather) in self.weathers.iter().enumerate() {
                                if ui.selectable_label(self.select_weather == index, weather.to_string()).clicked() {
                                    self.select_weather = index;
                                }
                            }
                        });
                        ui.end_row();

                        ui.label("天气类型：");
                        if !self.skytypes.is_empty() {
                            ComboBox::from_id_source("select skytype").selected_text(self.skytypes[self.select_skytype].get_weather_string())
                            .width(150.0)
                            .show_ui(ui, |ui| {
                                for (index, item) in self.skytypes.iter().enumerate() {
                                    if ui.selectable_label(self.select_skytype == index, item.get_weather_string()).clicked() {
                                        self.select_skytype = index;
                                    }
                                }
                            });
                        }
                        ui.end_row();
                    });

                    ui.add_space(20.0);
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(140.0);
                        if ui.button("取消").clicked() {
                            page.route.back_from_page(UiPageState::PageCreate);
                        }
                        if ui.button("确认").clicked() {
                            self.create_room(page);
                        }
                    });
                });
            });
        });
    }
}

impl UiCreateRace {
    fn update_stage_weathers(&mut self, page: &mut UiPageCtx) {
        self.skytypes.clear();
        let mut rbr = RBRGame::new(&page.store.game_path);
        let stage_id: u32 = self.stages[self.select_stage].stage_id.parse().unwrap();
        if let Some(skytypes) = rbr.load_game_stage_weathers(&stage_id) {
            self.skytypes = skytypes;
            self.select_skytype = 0 as usize;
        }
    }

    fn create_room(&mut self, page: &mut UiPageCtx) {
        let mut raceinfo = RaceInfo{
            name: self.room_name.clone(),
            owner: page.store.user_name.clone(),
            stage: self.stages[self.select_stage].name.clone(),
            stage_id: self.stages[self.select_stage].stage_id.parse().unwrap(),
            stage_type: self.stages[self.select_stage].get_surface(),
            stage_len: self.stages[self.select_stage].length.parse().unwrap(),
            car_fixed: self.fixed_car,
            car: self.cars[self.select_car].name.clone(),
            car_id: self.cars[self.select_car].id.parse().unwrap(),
            damage: self.select_damage as u32,
            weather: self.select_weather as u32,
            wetness: self.select_wetness as u32,
            skytype: "Default".to_string(),
            skytype_id: 0u32,
        };
        if let Some(skytype) = self.skytypes.get(self.select_skytype) {
            raceinfo.skytype = skytype.get_weather_string().clone();
            raceinfo.skytype_id = self.select_skytype as u32;
        }

        let mut create = RaceCreate {token: page.store.user_token.clone(), info: raceinfo, locked: false, passwd: None};
        if !self.room_passwd.is_empty() {
            create.locked = true;
            create.passwd = Some(self.room_passwd.clone());
        }

        let url = page.store.get_http_url("api/race/create");
        let tx = page.tx.clone();
        let room_name = self.room_name.clone();
        tokio::spawn(async move {
            let res = reqwest::Client::new().post(url).json(&create).send().await.unwrap();
            match res.status() {
                StatusCode::OK => {
                    tx.send(UiMsg::MsgSetRoomInfo(room_name)).await.unwrap();
                    tx.send(UiMsg::MsgGotoPage(UiPageState::PageInRoom)).await.unwrap();
                }
                _ => {
                    tx.send(UiMsg::MsgSetErrState("创建比赛房间失败".to_string())).await.unwrap();
                }
            }
        });
    }
}
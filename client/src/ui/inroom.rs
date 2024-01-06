use eframe::egui;
use egui::Grid;
use egui::ComboBox;
use egui::containers::popup::popup_below_widget;
use protocol::httpapi::RaceAccess;
use protocol::httpapi::RaceInfoUpdate;
use protocol::httpapi::{RaceQuery, RaceInfo, RaceLeave, RaceUserState, RaceState};
use reqwest::StatusCode;
use crate::{ui::UiPageState, game::rbr::{RBRGame, RBRStageData, RBRCarData}};
use super::UiMsg;
use super::{UiView, UiPageCtx};
use tokio::{sync::mpsc::{channel, Receiver, Sender}, task::JoinHandle};

enum UiInRoomMsg {
    MsgInRoomRaceInfo(RaceInfo),
    MsgInRoomUserState(Vec<RaceUserState>),
    MsgInRoomSetRoomReady,
    MsgInRoomStartRacing,
}

pub struct UiInRoom {
    pub room_name: String,
    pub raceinfo: RaceInfo,
    pub userstates: Vec<RaceUserState>,
    pub show_updatewin: bool,
    pub room_started: bool,
    pub stages: Vec<RBRStageData>,
    pub select_stage: usize,
    pub filter_stage: String,
    pub fixed_car: bool,
    pub cars: Vec<RBRCarData>,
    pub select_car: usize,
    pub filter_car: String,
    pub setups: Vec<String>,
    pub select_setup: usize,
    pub damages: Vec<&'static str>,
    pub select_damage: usize,
    rx: Receiver<UiInRoomMsg>,
    tx: Sender<UiInRoomMsg>,
    pub timed_task: Option<JoinHandle<()>>,
}

impl Default for UiInRoom {
    fn default() -> Self {
        let (tx, rx) = channel::<UiInRoomMsg>(8);
        Self { 
            room_name: "No Room Info".to_string(),
            raceinfo: RaceInfo::default(),
            userstates: vec![],
            show_updatewin: false,
            room_started: false,
            stages: vec![],
            select_stage: 246,
            filter_stage: String::from("Lyon - Gerland"),
            fixed_car: false,
            cars: vec![],
            select_car: 36,
            filter_car: String::from("Ford Fiesta WRC 2019"),
            setups: vec!["Default".to_string()],
            select_setup: 0,
            damages: vec!["Off", "Safe", "Reduced", "Realistic"],
            select_damage: 3,
            rx,
            tx,
            timed_task: None,
        }
    }
}

impl UiView for UiInRoom {
    fn init(&mut self, page: &mut UiPageCtx) {
        let mut rbr = RBRGame::new(&page.store.game_path);
        if let Some(stages) = rbr.load_game_stages() {
            self.stages = stages;
        }
        if let Some(cars) = rbr.load_game_cars() {
            self.cars = cars;
        }
    }

    fn enter(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        self.room_name = page.store.curr_room.clone();
        self.room_started = false;
        let info_url = page.store.get_http_url("api/race/info");
        let info_tx = self.tx.clone();
        let info_query = RaceQuery {name: self.room_name.clone()};

        let state_url = page.store.get_http_url("api/race/state");
        let state_tx = self.tx.clone();
        let state_query = RaceQuery {name: self.room_name.clone()};
        let task = tokio::spawn(async move {
            loop {
                let res = reqwest::Client::new().get(&info_url).json(&info_query).send().await.unwrap();
                if res.status() == StatusCode::OK {
                    let text = res.text().await.unwrap();
                    let raceinfo: RaceInfo = serde_json::from_str(text.as_str()).unwrap();
                    info_tx.send(UiInRoomMsg::MsgInRoomRaceInfo(raceinfo)).await.unwrap();
                }

                let res = reqwest::Client::new().get(&state_url).json(&state_query).send().await.unwrap();
                if res.status() == StatusCode::OK {
                    let text = res.text().await.unwrap();
                    let userstate: Vec<RaceUserState> = serde_json::from_str(text.as_str()).unwrap();
                    state_tx.send(UiInRoomMsg::MsgInRoomUserState(userstate)).await.unwrap();
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        });
        self.timed_task = Some(task);
    }

    fn exit(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, _page: &mut UiPageCtx) {
        if let Some(task) = &self.timed_task {
            task.abort();
            self.timed_task = None;
        }
        self.room_started = false;
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        if let Ok(msg) = self.rx.try_recv() {
            match msg {
                UiInRoomMsg::MsgInRoomRaceInfo(info) => {
                    self.raceinfo = info;
                    self.update_car_setups(page);
                }
                UiInRoomMsg::MsgInRoomUserState(states) => {
                    self.userstates = states;
                }
                UiInRoomMsg::MsgInRoomSetRoomReady => {
                    self.room_started = true;
                }
                UiInRoomMsg::MsgInRoomStartRacing => {
                    self.start_game_racing(page);
                    page.route.switch_to_page(UiPageState::PageRacing);
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(120.0);
                ui.vertical(|ui| {
                    Grid::new("race room table")
                    .min_col_width(80.0)
                    .min_row_height(24.0)
                    .show(ui, |ui| {
                        ui.label("比赛房间：");
                        ui.label(&self.room_name);
                        ui.end_row();

                        ui.label("比赛赛道：");
                        ui.label(self.raceinfo.stage.clone());
                        ui.end_row();

                        ui.label("车辆损坏：");
                        ui.label(self.damages[self.raceinfo.damage as usize]);
                        ui.end_row();

                        if self.raceinfo.car_fixed {
                            ui.label("限定车辆: ");
                            ui.label(&self.raceinfo.car);
                        } else {
                            ui.label("自选车辆: ");
                            let filter_car = ui.add_sized([150.0, 25.0], egui::TextEdit::singleline(&mut self.filter_car));
                            let popup_car = ui.make_persistent_id("filter car");
                            if filter_car.changed() || filter_car.clicked() {
                                ui.memory_mut(|mem| mem.open_popup(popup_car));
                            }
                            popup_below_widget(ui, popup_car, &filter_car, |ui| {
                                let patten = self.filter_car.clone().to_lowercase();
                                egui::ScrollArea::new([false, true]).max_height(240.0).show(ui, |ui| {
                                    for (index, car) in self.cars.clone().iter().enumerate() {
                                        if car.name.to_lowercase().contains(patten.as_str()) {
                                            if ui.selectable_label(self.select_car == index, &car.name).clicked() {
                                                self.filter_car = car.name.clone();
                                                self.select_car = index;
                                                self.update_car_setups(page);
                                            }
                                        }
                                    }
                                });
                            });
                        };
                        ui.end_row();

                        if self.raceinfo.car_fixed {
                            ui.label("限定调校：");
                            ui.label("Default");
                        } else {
                            ui.label("车辆调校：");
                            ComboBox::from_id_source("car setup select").selected_text(&self.setups[self.select_setup])
                            .width(150.0)
                            .show_ui(ui, |ui| {
                                for (index, setup) in self.setups.iter().enumerate() {
                                    if ui.selectable_label(self.select_setup == index, setup).clicked() {
                                        self.select_setup = index;
                                    }
                                }
                            });
                        }
                    });
                    ui.add_space(10.0);

                    if (&self.raceinfo.owner == &page.store.user_name) && !self.room_started {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.add_space(60.0);
                            if ui.button("更新比赛").clicked() {
                                self.show_updatewin = true;
                            }
                            if ui.button("开始比赛").clicked() {
                                self.start_room_racing(page);
                            }
                        });
                    }

                    ui.add_space(20.0);
                    Grid::new("race players table")
                    .min_col_width(80.0)
                    .min_row_height(24.0)
                    .show(ui, |ui| {
                        ui.label("序号");
                        ui.label("车手");
                        ui.label("权限");
                        ui.label("状态");
                        ui.end_row();
                        for (index, player) in self.userstates.iter().enumerate() {
                            ui.label((index+1).to_string());
                            ui.label(&player.name);
                            if &self.raceinfo.owner == &player.name {
                                ui.label("房主");
                            } else {
                                ui.label("玩家");
                            }

                            match &player.state {
                                RaceState::RaceReady => ui.label("已就绪"),
                                RaceState::RaceLoaded => ui.label("加载完成"),
                                RaceState::RaceFinished | RaceState::RaceRetired => ui.label("已完成"),
                                _ => ui.label("未就绪"),
                            };
                            ui.end_row();
                        }
                    });

                    ui.add_space(20.0);
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(80.0);
                        if ui.button("退出").clicked() {
                            self.leave_raceroom(page);
                            page.route.switch_to_page(UiPageState::PageLobby);
                        }                        
                        if ui.button("准备").clicked() {
                            self.set_game_ready(page);
                        }
                    });
                });
            });
        });

        if self.show_updatewin {
            self.show_updatewindow(ctx, frame, page);
        }
    }
}

impl UiInRoom {
    fn start_room_racing(&mut self, page: &mut UiPageCtx) {
        let url = page.store.get_http_url("api/race/start");
        let access = RaceAccess {token: page.store.user_token.clone(), room: self.room_name.clone()};
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let res = reqwest::Client::new().put(url).json(&access).send().await.unwrap();
            match res.status() {
                StatusCode::OK => {
                    tx.send(UiInRoomMsg::MsgInRoomSetRoomReady).await.unwrap();
                },
                _ => {},
            }
        });
    }

    fn set_game_ready(&mut self, page: &mut UiPageCtx) {
        let url = page.store.get_http_url("api/race/start");
        let query = RaceQuery {name: self.room_name.clone()};
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let res = reqwest::Client::new().get(url).json(&query).send().await.unwrap();
            match res.status() {
                StatusCode::OK => {
                    let text = res.text().await.unwrap();
                    if let Ok(started) = serde_json::from_str::<bool>(text.as_str()) {
                        if started {
                            tx.send(UiInRoomMsg::MsgInRoomStartRacing).await.unwrap();
                        }
                    }
                },
                _ => {},
            }

        });
    }

    fn start_game_racing(&mut self, page: &mut UiPageCtx) {
        let mut rbr = RBRGame::new(&page.store.game_path);
        if self.raceinfo.car_fixed {
            rbr.set_race_car(&self.raceinfo.car_id);
            rbr.set_race_car_setup(&self.raceinfo.car_id, &"".to_string());
        }
        else {
            rbr.set_race_car(&self.cars[self.select_car].id.parse().unwrap());
            if self.select_setup == 0 {
                rbr.set_race_car_setup(&self.raceinfo.car_id, &"".to_string());
            } else {
                rbr.set_race_car_setup(&self.cars[self.select_car].id.parse().unwrap(), &self.setups[self.select_setup]);
            }
        }
    }

    fn update_car_setups(&mut self, page: &mut UiPageCtx) {
        self.setups.clear();
        self.setups.push("Default".to_string());
        if self.raceinfo.car_fixed {
            return;
        }

        let mut rbr = RBRGame::new(&page.store.game_path);
        if let Some(setups) = rbr.load_game_car_setups(&self.cars[self.select_car].path) {
            for setup in setups {
                self.setups.push(setup);
            }
        }
    }

    fn leave_raceroom(&mut self, page: &mut UiPageCtx) {
        if !page.store.user_token.is_empty() && !page.store.curr_room.is_empty() {
            let user: RaceLeave = RaceLeave{token: page.store.user_token.clone(), room: page.store.curr_room.clone()};
            let url = page.store.get_http_url("api/race/leave");
            tokio::spawn(async move {
                let _res = reqwest::Client::new().post(url).json(&user).send().await.unwrap();
            });
        }

        page.store.curr_room.clear();
        self.room_name.clear();
    }

    fn update_room(&mut self, page: &mut UiPageCtx) {
        self.raceinfo.stage = self.stages[self.select_stage].name.clone();
        self.raceinfo.stage_id = self.stages[self.select_stage].stage_id.parse().unwrap();
        self.raceinfo.stage_len = self.stages[self.select_stage].length.parse().unwrap();
        self.raceinfo.car_fixed = self.fixed_car;
        self.raceinfo.car = self.cars[self.select_car].name.clone();
        self.raceinfo.car_id = self.cars[self.select_car].id.parse().unwrap();
        self.raceinfo.damage = self.select_damage as u32;

        let update = RaceInfoUpdate {
            token: page.store.user_token.clone(),
            info: self.raceinfo.clone(),
        };

        let url = page.store.get_http_url("api/race/info");
        let tx = page.tx.clone();
        tokio::spawn(async move {
            let res = reqwest::Client::new().put(url).json(&update).send().await.unwrap();
            match res.status() {
                StatusCode::OK => {}
                _ => {
                    tx.send(UiMsg::MsgSetErrState("Failed to change race info".to_string())).await.unwrap();
                }
            }
        });
    }

    fn show_updatewindow(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        egui::Window::new("pop update window").fixed_pos(egui::pos2(320.0, 200.0)).fixed_size([350.0, 450.0])
        .title_bar(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    Grid::new("race room table")
                    .min_col_width(80.0)
                    .min_row_height(24.0)
                    .show(ui, |ui| {
                        ui.label("比赛赛道：");
                        let filter_stage = ui.add_sized([200.0, 25.0], egui::TextEdit::singleline(&mut self.filter_stage));
                        let popup_stage = ui.make_persistent_id("filter stage");
                        if filter_stage.changed() || filter_stage.clicked() {
                            ui.memory_mut(|mem| mem.open_popup(popup_stage));
                        }
                        popup_below_widget(ui, popup_stage, &filter_stage, |ui| {
                            let patten = self.filter_stage.clone().to_lowercase();
                            egui::ScrollArea::new([false, true]).max_height(240.0).show(ui, |ui| {
                                for (index, stage) in self.stages.iter().enumerate() {
                                    if stage.name.to_lowercase().contains(patten.as_str()) {
                                        if ui.selectable_label(self.select_stage == index, &stage.name).clicked() {
                                            self.filter_stage = stage.name.clone();
                                            self.select_stage = index;
                                        }
                                    }
                                }
                            });
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
                    });

                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(80.0);
                        if ui.button("取消").clicked() {
                            self.show_updatewin = false;
                        }
                        if ui.button("确认").clicked() {
                            self.show_updatewin = false;
                            self.update_room(page);
                        }
                    });
                });
            });
        });
    }
}
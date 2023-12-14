use eframe::egui;
use egui::Grid;
use protocol::httpapi::{RaceList, RaceItem, RoomState};
use crate::ui::UiPageState;
use super::{UiView, UiPageCtx};
use reqwest::StatusCode;
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub struct UiLobby {
    pub table_head: Vec<&'static str>,
    pub table_data: RaceList,
    rx: Receiver<RaceList>,
    tx: Sender<RaceList>,
}

impl Default for UiLobby {
    fn default() -> Self {
        let (tx, rx) = channel::<RaceList>(8);
        Self {
            table_head: vec!["序号", "房名", "赛道", "房主", "状态"],
            table_data: RaceList {
                room: vec![RaceItem {
                    name: String::from("Test Room 1"),
                    stage: String::from("semetin 2009"),
                    owner: String::from("Ziye"),
                    state: RoomState::RoomFree,
                },
                RaceItem {
                    name: String::from("Test Room 2"),
                    stage: String::from("semetin 2010"),
                    owner: String::from("Shanyin"),
                    state: RoomState::RoomLocked,
                }]
            },
            rx,
            tx,
        }
    }
}

impl UiView for UiLobby {
    fn enter(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        let url = page.store.get_http_url("api/race/list");
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let res = reqwest::Client::new().get(url).send().await.unwrap();
            match res.status() {
                StatusCode::OK => {
                    let text = res.text().await.unwrap();
                    let racelist: RaceList = serde_json::from_str(text.as_str()).unwrap();
                    tx.send(racelist).await.unwrap();
                },
                StatusCode::NO_CONTENT => {
                    let racelist = RaceList::default();
                    tx.send(racelist).await.unwrap();
                },
                _ => {},
            }
        });
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        if let Ok(msg) = self.rx.try_recv() {
            self.table_data = msg;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(120.0);
                ui.vertical_centered(|ui| {
                    Grid::new("race rooms").min_col_width(120.0).show(ui, |ui| {
                        for content in &self.table_head {
                            ui.label(*content);
                        }
                        ui.end_row();

                        for (index, race) in self.table_data.room.iter().enumerate() {
                            let table = vec![index.to_string(),
                                race.name.clone(),
                                race.stage.clone(),
                                race.owner.clone(),
                                match race.state {
                                    RoomState::RoomLocked => String::from("禁止加入"),
                                    RoomState::RoomRaceOn => String::from("比赛中"),
                                    _ => String::from("空闲")
                                }
                            ];
                            for content in table {
                                ui.label(content);
                            }
                            if ui.button("加入").clicked() {
                                page.route.switch_to_page(UiPageState::PageInRoom);
                            }
                            ui.end_row();
                        }
                    });
                });
                ui.add_space(120.0);
            });
        });
    }
}
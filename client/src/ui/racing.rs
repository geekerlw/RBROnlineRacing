use eframe::egui;
use egui::Grid;
use egui::RichText;
use protocol::httpapi::RaceState;
use tokio::sync::mpsc::{Sender, Receiver};
use crate::ui::UiPageState;
use super::{UiView, UiPageCtx};
use protocol::httpapi::{MetaRaceResult, MetaRaceData};

enum UiRacingMsg {
    MsgGotoPage(UiPageState),
}

pub struct UiRacing {
    pub state: RaceState,
    tx: Sender<UiRacingMsg>,
    rx: Receiver<UiRacingMsg>,
    pub table_head: Vec<&'static str>,
    pub table_data: MetaRaceResult,
}

impl Default for UiRacing {
    fn default() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel::<UiRacingMsg>(16);
        Self {
            state: RaceState::RaceInit,
            tx,
            rx,
            table_head: vec!["排名", "车手", "分段1", "分段2", "完成时间"],
            table_data: MetaRaceResult {
                state: protocol::httpapi::RaceState::RaceFinished,
                board: vec![MetaRaceData {
                    token: String::from("token"),
                    profile_name: String::from("Ziye"),
                    starttime: 0.0,
                    racetime: 120.0,
                    process: 100.0,
                    splittime1: 30.0,
                    splittime2: 80.0,
                    finishtime: 120.0,
                },
                MetaRaceData {
                    token: String::from("token"),
                    profile_name: String::from("somechen"),
                    starttime: 0.0,
                    racetime: 120.0,
                    process: 100.0,
                    splittime1: 33.0,
                    splittime2: 90.0,
                    finishtime: 140.0,
                }]
            }
        }
    }
}

impl UiView for UiRacing {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        if let Ok(msg) = self.rx.try_recv() {
            match msg {
                UiRacingMsg::MsgGotoPage(state) => page.route.switch_to_page(state),
            };
        }

        match self.state {
            RaceState::RaceRunning => self.show_racing(ctx, frame, page),
            RaceState::RaceFinished | RaceState::RaceRetired => self.show_result(ctx, frame, page),
            _ => self.show_loading(ctx, frame, page),
        }
    }
}

impl UiRacing {
    fn show_loading(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("游戏加载中...").size(40.0));
            });
        });
    }

    fn show_racing(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("比赛进行中...").size(40.0));
            });
        });
    }

    fn show_result(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, page: &mut UiPageCtx) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(120.0);
                ui.vertical(|ui| {
                    Grid::new("race result").min_col_width(120.0).show(ui, |ui| {
                        for content in &self.table_head {
                            ui.label(*content);
                        }
                        ui.end_row();

                        for (index, result) in self.table_data.board.iter().enumerate() {
                            let table = vec![index.to_string(),
                                result.profile_name.clone(),
                                result.splittime1.to_string(),
                                result.splittime2.to_string(),
                                result.finishtime.to_string(),
                            ];
                            for content in table {
                                ui.label(content);
                            }
                            ui.end_row();
                        }
                    });

                    ui.add_space(40.0);
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add_space(250.0);
                        if ui.button("确认").clicked() {
                            page.route.switch_to_page(UiPageState::PageInRoom);
                        }
                    });
                })
            });
        });
    }
}
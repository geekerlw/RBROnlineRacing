pub mod client;
pub mod ui;
pub mod store;

#[derive(Default, Clone)]
pub enum UiPageState {
    #[default]
    PageLogin,
    PageLobby,
    PageCreate,
    PageInRoom,
    PageLoading,
    PageRacing,
    PageFinish,
    PageSetting,
}

#[derive(Default)]
pub struct UiPages {
    pub login: ui::login::UiLogin,
    pub finish: ui::finish::UiFinish,
    pub loading: ui::loading::UiLoading,
    pub lobby: ui::lobby::UiLobby,
    pub racing: ui::racing::UiRacing,
    pub setting: ui::setting::UiSetting,
    pub create: ui::create::UiCreateRace,
    pub inroom: ui::inroom::UiInRoom,
}
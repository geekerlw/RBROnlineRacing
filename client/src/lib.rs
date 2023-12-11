pub mod client;
pub mod components;
pub mod store;
pub mod route;

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
    pub login: components::login::UiLogin,
    pub finish: components::finish::UiFinish,
    pub loading: components::loading::UiLoading,
    pub lobby: components::lobby::UiLobby,
    pub racing: components::racing::UiRacing,
    pub setting: components::setting::UiSetting,
    pub create: components::create::UiCreateRace,
    pub inroom: components::inroom::UiInRoom,
}
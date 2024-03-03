use crate::components::store::RacingStore;
use std::sync::Mutex;

#[derive(Default)]
pub struct RacingClient {
    pub store: Mutex<RacingStore>
}

impl RacingClient {
    pub fn init(self) -> Self {
        self.store.lock().unwrap().init();
        self
    }
}
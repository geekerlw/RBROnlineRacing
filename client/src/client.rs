use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
pub struct RacingClient {
    pub count: Arc<Mutex<u32>>,
}

impl RacingClient {
    pub fn start_inc_task(&mut self) {
        let cnt_clone = self.count.clone();
        std::thread::spawn(move || {
            loop {
                let mut cnt = cnt_clone.lock().unwrap();
                *cnt = 0;
                println!("steven: reset cnt to zero\n");
                std::thread::sleep(std::time::Duration::from_secs(3));
            };
        });
    }
}
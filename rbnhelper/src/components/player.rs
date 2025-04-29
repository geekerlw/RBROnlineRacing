use std::{fs::File, path::PathBuf};
use std::io::BufReader;
use ini::Ini;
use rand::seq::SliceRandom;
use rodio::OutputStream;
use rand::thread_rng;

pub struct AudioPlayer {
    file: PathBuf,
    announcer: String,
    timeout: u64,
    volume: f32,
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self {
            file: PathBuf::default(),
            announcer: "xiaoxiao".to_string(),
            timeout: 3,  // default play 3 seconds.
            volume: 0.4, // default volume to 0.4x.
        }
    }
}

impl AudioPlayer {
    pub fn notification(filename: &str) -> Self {
        let mut player = AudioPlayer::default();
        if let Some(game_root) = std::env::current_exe().unwrap().parent() {
            let conf_path = game_root.join("Plugins").join("RBNHelper").join("RBNHelper.ini");
            if let Ok(conf) = Ini::load_from_file(&conf_path) {
                player.announcer = conf.get_from_or(Some("Audio"), "Announcer", "xiaoxiao").to_string();
                player.volume = conf.get_from_or(Some("Audio"), "Volume", "0.4").parse().unwrap();
            }
            player.file = game_root.join("Plugins").join("RBNHelper")
                .join("audio").join("notification")
                .join(player.announcer.as_str()).join(filename);
        }
        player
    }

    pub fn ridicule(player_name: &str, prefix: &str) -> Self {
        let mut player = AudioPlayer::default();
        if let Some(game_root) = std::env::current_exe().unwrap().parent() {
            let conf_path = game_root.join("Plugins").join("RBNHelper").join("RBNHelper.ini");
            if let Ok(conf) = Ini::load_from_file(&conf_path) {
                player.volume = conf.get_from_or(Some("Audio"), "Volume", "0.4").parse().unwrap();
            }

            let mut target_path = game_root.join("Plugins").join("RBNHelper")
                .join("audio").join("ridicule")
                .join(player_name);
            if !target_path.exists() {
                target_path = game_root.join("Plugins").join("RBNHelper")
                    .join("audio").join("ridicule")
                    .join("default");
            }

            let entrys: Vec<_> = std::fs::read_dir(target_path).unwrap()
                .filter_map(Result::ok)
                .filter(|entry| entry.file_name().to_string_lossy().starts_with(prefix))
                .collect();
            if let Some(entry) = entrys.choose(&mut thread_rng()) {
                player.file = entry.path();
            }
        }
        player
    }

    pub fn set_timeout(mut self, secs: u64) -> Self {
        self.timeout = secs;
        self
    }

    #[allow(dead_code)]
    pub fn set_volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }

    pub fn play(&mut self) {
        if self.file.exists() && self.file.is_file() && self.file.extension().map_or(false, |ext| ext == "wav") {
            let audio_file = self.file.clone();
            let timeout = self.timeout.clone();
            let volume = self.volume.clone();
            std::thread::spawn(move || {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let file = BufReader::new(File::open(audio_file).unwrap());
                let sink = stream_handle.play_once(file).unwrap();
                sink.set_volume(volume);
                std::thread::sleep(std::time::Duration::from_secs(timeout));
            });
        }
    }
}
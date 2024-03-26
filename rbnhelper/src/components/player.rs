use std::{fs::File, path::PathBuf};
use std::io::BufReader;
use log::info;
use rodio::{Decoder, OutputStream, source::Source};

#[derive(Default)]
pub struct OggPlayer {
    file: PathBuf,
    timeout: u64,
}

impl OggPlayer {
    pub fn open(file: &str) -> Self {
        let mut player = OggPlayer::default();
        player.timeout = 3; // default play 3 seconds.
        if let Some(game_root) = std::env::current_exe().unwrap().parent() {
            player.file = game_root.join("Plugins").join("RBNHelper").join("audio").join(file);
        }
        player
    }

    pub fn set_timeout(mut self, secs: u64) -> Self {
        self.timeout = secs;
        self
    }

    pub fn play(&mut self) {
        if self.file.exists() && self.file.is_file() {
            let audio_file = self.file.clone();
            let timeout = self.timeout.clone();
            std::thread::spawn(move || {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let file = BufReader::new(File::open(audio_file).unwrap());
                let source = Decoder::new(file).unwrap();
                stream_handle.play_raw(source.convert_samples()).unwrap();
                std::thread::sleep(std::time::Duration::from_secs(timeout));
            });
        }
    }
}
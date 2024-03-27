use std::{fs::File, path::PathBuf};
use std::io::BufReader;
use rodio::OutputStream;

#[derive(Default)]
pub struct OggPlayer {
    file: PathBuf,
    timeout: u64,
    volume: f32,
}

impl OggPlayer {
    pub fn open(file: &str) -> Self {
        let mut player = OggPlayer::default();
        player.timeout = 3; // default play 3 seconds.
        player.volume = 0.4; // default volume to 0.4x.
        if let Some(game_root) = std::env::current_exe().unwrap().parent() {
            player.file = game_root.join("Plugins").join("RBNHelper").join("audio").join(file);
        }
        player
    }

    pub fn set_timeout(mut self, secs: u64) -> Self {
        self.timeout = secs;
        self
    }

    pub fn set_volume(mut self, volume: f32) -> Self {
        self.volume = volume;
        self
    }

    pub fn play(&mut self) {
        if self.file.exists() && self.file.is_file() {
            let audio_file = self.file.clone();
            let timeout = self.timeout.clone();
            let volume = self.volume.clone();
            std::thread::spawn(move || {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let file = BufReader::new(File::open(audio_file).unwrap());
                // let source = Decoder::new(file).unwrap();
                let sink = stream_handle.play_once(file).unwrap();
                sink.set_volume(volume);
                std::thread::sleep(std::time::Duration::from_secs(timeout));
            });
        }
    }
}
use std::sync::mpsc::Receiver;
use std::thread;

use rakelog::rakeError;
use rodio::source::SineWave;
use rodio::Source;
use rodio::{OutputStream, Sink};
use std::time::Duration;

pub enum RakeAudioMessage {
    EatFood,
}

fn main() {
    println!("Hello, world!");
}

pub struct RakeAudio;

impl RakeAudio {
    pub fn main(audio_r: Receiver<RakeAudioMessage>) {
        thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();

            loop {
                match audio_r.recv() {
                    Ok(msg) => match msg {
                        RakeAudioMessage::EatFood => {
                            let source =
                                SineWave::new(700.0).take_duration(Duration::from_secs_f32(0.1));
                            let _ = stream_handle.play_raw(source.convert_samples());
                        }
                    },
                    Err(e) => rakeError!("{e}"),
                }
            }
        });
    }
}

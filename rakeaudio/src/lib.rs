use std::sync::mpsc::Receiver;
use std::thread;

use rakelog::{rakeError, rakeInfo};
use rodio::source::SineWave;
use rodio::Source;
use rodio::{OutputStream, Sink};
use std::time::Duration;
use std::io::BufReader;
use std::fs::File;
use rodio::Decoder;
use std::sync::Arc;

pub enum RakeAudioMessage {
    EatFood,
    Die,
    Buy,
}

fn main() {
    println!("Hello, world!");
}

pub struct RakeAudio;

impl RakeAudio {
    pub fn main(audio_r: Receiver<RakeAudioMessage>) {
        thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();

            let game_music_file = BufReader::new(File::open("snake-baron.mp3").unwrap());
            let game_music_source = Decoder::new(game_music_file).unwrap().repeat_infinite().fade_in(Duration::from_secs(5));

            let _ = stream_handle.play_raw(game_music_source.convert_samples());

            let eat_music_file = BufReader::new(File::open("eat.wav").unwrap());
            let eat_music_source = Decoder::new(eat_music_file).unwrap().buffered();
            
            let crash_music_file = BufReader::new(File::open("crash.wav").unwrap());
            let crash_music_source = Decoder::new(crash_music_file).unwrap().buffered();
            
            let buy_music_file = BufReader::new(File::open("buy.wav").unwrap());
            let buy_music_source = Decoder::new(buy_music_file).unwrap().buffered();
            

            loop {
                match audio_r.recv() {
                    Ok(msg) => match msg {
                        RakeAudioMessage::EatFood => {
                            let _ = stream_handle.play_raw(eat_music_source.clone().convert_samples());
                        },
                        RakeAudioMessage::Die => {
                            let _ = stream_handle.play_raw(crash_music_source.clone().convert_samples());
                        }
                        RakeAudioMessage::Buy => {
                            let _ = stream_handle.play_raw(buy_music_source.clone().convert_samples());
                        }

                    },
                    Err(e) => rakeError!("{e}"),
                }
            }
        });
    }
}

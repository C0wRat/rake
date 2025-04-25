mod game;
mod util;
use std::{sync::mpsc::channel, thread};

use crate::game::Game;
use cursive::Cursive;
use cursive::{CbSink, CursiveExt};
use rakeaudio::{RakeAudio, RakeAudioMessage};
use rakedisplay::{DisplayMsg, RakeGUI};
use rakelog::{rakeError, rakeInfo, rake_log};
use rakemodel::grid::Grid;

fn main() {
    rake_log::init("rake.log");
    rakeInfo!("Started rake!");

    let (audio_s, audio_r) = channel::<RakeAudioMessage>();

    RakeAudio::main(audio_r);
    let mut gui = RakeGUI::new();
    let (display_s, display_r) = channel::<DisplayMsg>();
    RakeGUI::main_menu(&mut gui.siv, display_s.clone());
    let sink = gui.siv.cb_sink().clone();
    Game::init(sink, display_r, display_s, audio_s.clone());
    gui.siv.run();
}

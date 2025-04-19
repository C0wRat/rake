mod game;
mod util;
use std::{sync::mpsc::channel, thread};

use cursive::{CbSink, CursiveExt};
use rakedisplay::{DisplayMsg, RakeGUI};
use rakelog::{rakeError, rakeInfo, rake_log};
use rakemodel::grid::Grid;
use cursive::Cursive;
use crate::game::Game;

fn main() {
    rake_log::init("rake.log");
    rakeInfo!("Started rake!");
    let mut gui = RakeGUI::new();
    let (display_s, display_r) = channel::<DisplayMsg>();
    RakeGUI::main_menu(&mut gui.siv, display_s.clone());
    let sink = gui.siv.cb_sink().clone();
    Game::init(sink, display_r, display_s);
    gui.siv.run();
}
use cursive::views::Dialog;
use cursive::{Cursive, CursiveExt};
use rakelog::rakeDebug;

pub struct RakeGUI {}

impl RakeGUI {
    pub fn main_menu() {
        rakeDebug!("loading rake start screen");

        let mut siv = Cursive::new();

        siv.add_layer(
            Dialog::text("Welcome to Rake!")
                .title("R A K E")
                .button("Start", |s| start(s))
                .button("Quit", |s| s.quit()),
        );

        siv.run();
    }
}

pub fn start(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("Starting..."));
}

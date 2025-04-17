use rakedisplay::RakeGUI;
use rakelog::{rakeInfo, rake_log};

fn main() {
    rake_log::init("rake.log");
    rakeInfo!("Started rake!");
    let mut gui = RakeGUI::new();
    RakeGUI::main_menu(&mut gui.siv);
}

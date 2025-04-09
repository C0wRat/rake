use rakelog::{rake_log, rakeInfo};

use rakedisplay::RakeGUI;

fn main() {
    rake_log::init("rake.log");
    rakeInfo!("Started rake!");
    RakeGUI::main_menu();
}

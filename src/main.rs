use rakelog::{rake_log, rakeInfo};

fn main() {
    rake_log::init("rake.log");
    rakeInfo!("Started rake!");
}

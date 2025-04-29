use std::fs;

use rakelog::{rakeError, rakeInfo};

pub fn save_score(score: i32) {
    rakeInfo!("Saving {score} to rake.dat");
    let _ = fs::write("rake.dat", format!("{score}"));
}

pub fn read_score() -> i32 {
    match fs::read_to_string("rake.dat") {
        Ok(score) => return score.parse::<i32>().unwrap(),
        Err(e) => {
            rakeError!("read_score error: {e}");
            return 0;
        }
    }
}

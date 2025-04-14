use cursive::views::Dialog;
use cursive::{Cursive, CursiveExt};
use rakelog::{rakeDebug, rakeInfo};
use std::thread;
use std::time::Duration;

pub struct RakeGUI {
    pub siv: Cursive,
}

impl RakeGUI {
    pub fn new() -> Self {
        let siv = Cursive::new();
        Self { siv }
    }

    pub fn main_menu(s: &mut Cursive) {
        rakeDebug!("loading rake start screen");
        s.pop_layer();
        s.add_layer(
            Dialog::text("Welcome to Rake!")
                .title("R A K E")
                .button("Start", |s| start(s))
                .button("Demo", |s| demo(s, Grid::new(20, 10)))
                .button("Quit", |s| s.quit()),
        );

        s.run();
    }

    pub fn screen_test(s: &mut Cursive, snake: (i32, i32)) {
        s.pop_layer();
        let mut grid = Grid::new(20, 10);
        s.add_layer(Dialog::text(grid.gen_grid(snake)).title("R A K E"));
    }
}

#[derive(Clone, Copy)]
pub struct Grid {
    x: usize,
    y: usize,
}

impl Grid {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn gen_grid(&mut self, snake: (i32, i32)) -> String {
        // Generating a grid needs the x & y for the gird size
        // We also need a snake corodinate (x,y)
        // So we first want to get the ammont of rows we are going to create (y)
        let mut grid = String::new();
        for y_cord in 0..self.y {
            // For each row we want to know if the snake is in it.
            if snake.1 == y_cord as i32 {
                // If the sanke is in the row then we will need to draw it.
                let row = self.gen_row(Some(snake)) + "\n";
                grid.push_str(&row);
            } else {
                // If no snake is in the row then we won't bother drawing it.
                // This probably could just take the logic out of gen_row, but for now this works.
                let row = self.gen_row(None) + "\n";
                grid.push_str(&row);
            }
        }
        // We then return the grid string to be displayed.
        return grid;
    }

    fn gen_row(&mut self, snake: Option<(i32, i32)>) -> String {
        let mut row = String::new();

        // Check if we are drawing a snake or an empty row.
        let snake_x = match snake {
            Some(snake) => snake.0,
            None => {
                // We can just draw loads of " "'s if its empty.
                let empty_row = " ".repeat(self.x);
                row.push_str(&empty_row);
                return row;
            }
        };

        // if the snake is in this row we need to find it's x cord.
        for x_cord in 0..self.x {
            if snake_x == x_cord as i32 {
                // if the snake is in this x cord we add o to the row.
                row.push('o');
            } else {
                row.push(' ');
            }
        }

        return row;
    }
}

pub fn start(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("Starting..."));
}

pub fn demo(s: &mut Cursive, grid: Grid) {
    s.pop_layer();
    let sink = s.cb_sink().clone();
    for x in 0..grid.x as i32 {
        for y in 0..grid.y as i32 {
            // We need the sink to allow for the thread sleep.
            // Maybe there is a better way to apporach this.
            let sink = sink.clone();
            let delay = (x * grid.y as i32 + y) as u64 * 120;
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(delay));
                rakeDebug!("Snake moved to {}:{}", x, y);
                // Send the command for the sink.
                // Im not fully sure but I belive the main cursive is not thread safe.
                let _ = sink.send(Box::new(move |s: &mut Cursive| {
                    RakeGUI::screen_test(s, (x, y));
                }));

                // We are basically just waiting for this to finish.
                // It's only a demo so I really cant be bothered to try and get
                // this workign properly as it won't really be reachable by a player.
                if (x == (grid.x as i32 - 1)) && (y == (grid.y as i32 - 1)) {
                    let _ = sink.send(Box::new(move |s| {
                        RakeGUI::main_menu(s);
                    }));
                }
            });
        }
    }
}

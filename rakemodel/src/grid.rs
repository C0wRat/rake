use crate::snake::SnakeDirection;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Grid {
    pub x: usize,
    pub y: usize,
}

impl Grid {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn gen_grid(&mut self, objects: Vec<GridObject>) -> String {
        // Generating a grid needs the x & y for the gird size
        // We also need a snake corodinate (x,y)
        // So we first want to get the ammont of rows we are going to create (y)
        let mut grid = String::new();
        for y in 0..self.y {
            let mut row = String::new();
            for x in 0..self.x {
                row.push(self.gen_cell(&objects, x as i32, y as i32));
            }
            row = row + "\n";
            grid.push_str(&row);
        }
        return grid;
        // We then return the grid string to be displayed.
    }

    fn gen_cell(&mut self, objects: &Vec<GridObject>, x: i32, y: i32) -> char {
        for object in objects {
            // We will return the first object that has the x&y cords for this cell.
            if (object.x == x) && (object.y == y) {
                return object.i;
            }
        }
        return '·';
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GridObject {
    pub x: i32,
    pub y: i32,
    pub i: char,
    pub obj_type: ObjectType,
    pub direction: Option<SnakeDirection>,
}
impl GridObject {
    pub fn new(
        x: i32,
        y: i32,
        i: char,
        obj_type: ObjectType,
        direction: Option<SnakeDirection>,
    ) -> Self {
        Self {
            x,
            y,
            i,
            obj_type,
            direction,
        }
    }

    pub fn reset(&mut self, grid: Grid) {
        let mut rng = rand::rng();
        self.x = rng.random_range(0..grid.x) as i32;
        self.y = rng.random_range(0..grid.y) as i32;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ObjectType {
    None,
    Snake,
    Food(i32),
}

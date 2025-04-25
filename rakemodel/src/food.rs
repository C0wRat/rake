use crate::grid::{Grid, GridObject};
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Food {
    pub body: GridObject,
}

impl Food {
    pub fn new(grid: Grid, value: i32, icon: char) -> Self {
        let mut body = GridObject::new(0, 0, icon, crate::grid::ObjectType::Food(value), None);
        body.reset(grid);
        Self { body }
    }
    pub fn reset(&mut self, grid: Grid) {
        self.body.reset(grid);
    }
}

use rakelog::rakeInfo;
use crate::grid::ObjectType;
use crate::grid::GridObject;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone)]
pub struct Snake {
    pub head: GridObject,
    pub body: Vec<GridObject>,
    pub size: i32,
    // direction: SnakeDirection,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            head: GridObject::new(x, y, '●', ObjectType::Snake, Some(SnakeDirection::Right)),
            body: Vec::new(),
            size: 0,
            // direction: SnakeDirection::Right,
        }
    }

    pub fn reset(&mut self){
        rakeInfo!("reseting snake");
        self.head.x = 0;
        self.head.y = 0;
        self.size = 0;
        self.head.direction = Some(SnakeDirection::Right);
        self.body.clear();

    }

    pub fn update_body(
        // previous_node: GridObject,
        current_node: GridObject,
        next_node: GridObject,
    ) -> char {
        let current_dir = current_node.direction.unwrap();
    
        let next_dir = next_node.direction.unwrap();
    
        // rakeInfo!("Going from {:#?} -> {:#?}", current_dir, next_dir);
    
        let char = match (current_dir, next_dir) {
            (SnakeDirection::Right, SnakeDirection::Right)
            | (SnakeDirection::Left, SnakeDirection::Left) => '═',
            (SnakeDirection::Up, SnakeDirection::Up) | (SnakeDirection::Down, SnakeDirection::Down) => {
                '║'
            }
    
            (SnakeDirection::Right, SnakeDirection::Up)
            | (SnakeDirection::Down, SnakeDirection::Left) => '╝',
            (SnakeDirection::Right, SnakeDirection::Down)
            | (SnakeDirection::Up, SnakeDirection::Left) => '╗',
            (SnakeDirection::Left, SnakeDirection::Up)
            | (SnakeDirection::Down, SnakeDirection::Right) => '╚',
            (SnakeDirection::Left, SnakeDirection::Down)
            | (SnakeDirection::Up, SnakeDirection::Right) => '╔',
    
            (_, _) => ' ',
        };
        return char;
    }
}
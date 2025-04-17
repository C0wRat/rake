use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use cursive::builder::Object;
use cursive::event::Key;
use cursive::reexports::time::format_description::modifier::Padding;
use cursive::view::Margins;
use cursive::views::{Button, ListView, Panel, ShadowView, TextContent, TextView};
use cursive::views::{Dialog, LinearLayout, TextArea};
use cursive::Vec2;
use cursive::View;
use cursive::{direction, Printer};
use cursive::{CbSink, Cursive, CursiveExt};
use rakelog::{rakeDebug, rakeInfo};
use rakemodel::food::food;
use rakemodel::{grid::{Grid,GridObject,ObjectType}, snake::{Snake, SnakeDirection}};
use rand::Rng;

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

        s.add_layer(LinearLayout::vertical().child(TextView::new(" R A K E"))
            .child(Panel::new(LinearLayout::vertical()
                .child(Button::new("Start", |s| sandbox(s, Grid::new(20, 10))))
                .child(Button::new("Info", |s| rakeInfo!("Info button pushed")))
                .child(Button::new("Help", |s| rakeInfo!("HELP button pushed")))
                .child(Button::new("Quit", |s| s.quit()))
                )
            )
        );
            

        s.run();
    }

    pub fn death_screen(s: &mut Cursive, grid: Grid) {
        s.add_layer(
            Dialog::text("YOU DIED!")
                .button("Main Menu", |s| {
                    s.pop_layer();
                    RakeGUI::main_menu(s);
                })
                .button("Replay", move |s| {
                    s.pop_layer();
                    sandbox(s, grid);
                }),
        );
    }

    pub fn render_screen(
        s: &mut Cursive,
        objects: Vec<GridObject>,
        snake_direction: &SnakeDirection,
        grid: &mut Grid,
        score: i32
    ) {
        s.pop_layer();

        // let title = format!("{:#?}", snake_direction);
        s.add_layer(
            LinearLayout::horizontal()
            .child( LinearLayout::vertical()
            .child(Dialog::text(grid.gen_grid(objects))
            .padding(Margins::lrtb(0, 0, 0, 0))
            .title("R A K E"))
            .child(Dialog::text("").title(format!("score:{score}")))
    ).child(Dialog::text("")));
       
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
                let mut grid_objects = Vec::new();
                grid_objects.push(GridObject::new(x, y, 'o', ObjectType::Snake, None));
                let _ = sink.send(Box::new(move |s: &mut Cursive| {
                    let mut grid_c = grid.clone();
                    RakeGUI::render_screen(s, grid_objects, &SnakeDirection::Down, &mut grid_c, 0);
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


pub fn sandbox(s: &mut Cursive, grid: Grid) {
    let snake = Arc::new(Mutex::new(Snake::new(0, 0))).clone();
    rakeInfo!("Grid Size: {}.{}", grid.x, grid.y);
    // let mut snake = Snake::new();
    s.pop_layer();
    let sink = s.cb_sink().clone();
    let delay = 150 as u64;

    let snake_clone_l = snake.clone();
    s.add_global_callback(Key::Left, move |_s| {
        snake_clone_l.lock().unwrap().head.direction = Some(SnakeDirection::Left)
    });

    let snake_clone_r = snake.clone();
    s.add_global_callback(Key::Right, move |_s| {
        snake_clone_r.lock().unwrap().head.direction = Some(SnakeDirection::Right)
    });

    let snake_clone_u = snake.clone();
    s.add_global_callback(Key::Up, move |_s| {
        snake_clone_u.lock().unwrap().head.direction = Some(SnakeDirection::Up)
    });

    let snake_clone_d = snake.clone();
    s.add_global_callback(Key::Down, move |_s| {
        snake_clone_d.lock().unwrap().head.direction = Some(SnakeDirection::Down)
    });

    let mut food = food::new(5, 5, 1, 'o');

    // Having input handlers would require snake to be an Arc<Mutex<Snake>> :/
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(delay));
        let mut snake = snake.lock().unwrap();

        if snake.body.len() < snake.size as usize {
            let body_node = if snake.body.is_empty() {
                // rakeInfo!("Adding body_node to head");
                GridObject::new(
                    snake.head.x,
                    snake.head.y,
                    'X',
                    ObjectType::Snake,
                    // snake.head.direction,
                    snake.head.direction,
                )
            } else {
                let tail = snake.body.last().unwrap();
                // rakeInfo!("Adding body_node to tail");
                GridObject::new(tail.x, tail.y, 'X', ObjectType::Snake, tail.direction)
            };
            // rakeInfo!("Adding {:#?} to snake body.", body_node);
            snake.body.push(body_node);
        }

        if (snake.head.x < 0 || snake.head.y < 0)
            || (snake.head.x >= grid.x as i32 || snake.head.y >= grid.y as i32)
        {
            let _ = sink.send(Box::new(move |s| {
                RakeGUI::death_screen(s, grid);
            }));
            break;
        }

        if !snake.body.is_empty() {
            let old_snake = snake.clone();
            for (index, body_node) in snake.body.iter_mut().enumerate() {
                if index != 0 {
                    body_node.i = old_snake.body[index - 1].i;
                    body_node.x = old_snake.body[index - 1].x;
                    body_node.y = old_snake.body[index - 1].y;
                } else {
                    body_node.i = Snake::update_body(body_node.clone(), old_snake.head.clone());
                    body_node.direction = old_snake.head.direction;
                    body_node.x = old_snake.head.x;
                    body_node.y = old_snake.head.y;
                }
            }
        }

        let old_head = snake.head.clone();
        match snake.head.direction.unwrap() {
            SnakeDirection::Up => snake.head.y = snake.head.y - 1,
            SnakeDirection::Down => snake.head.y = snake.head.y + 1,
            SnakeDirection::Right => snake.head.x = snake.head.x + 1,
            SnakeDirection::Left => snake.head.x = snake.head.x - 1,
        }
        
        snake.head.i = Snake::update_body(snake.head.clone(), old_head);


        // rakeDebug!("Snake moved to {}:{}", snake.head.x, snake.head.y);
        // Send the command for the sink.
        // Im not fully sure but I belive the main cursive is not thread safe.
        let snake_head = snake.head.clone();
        // This is gonna be cloning quite a bit of data if the snake gets too long.
        let snake_body = snake.body.clone();

        let mut grid_objects = Vec::new();

        grid_objects.push(snake_head);

        grid_objects.push(food.clone().body);

        for body_node in snake_body.iter() {
            grid_objects.push(body_node.clone());
        }

        // rakeInfo!("Objects: {:#?}", grid_objects);
        let mut collisions: Vec<GridObject> = Vec::new();
        for (index_a, object_a) in grid_objects.iter().enumerate() {
            if object_a.obj_type == ObjectType::Snake {
                for (index_b, object_b) in grid_objects.iter().enumerate() {
                    if (object_a.x == object_b.x)
                        && (object_a.y == object_b.y)
                        && (index_a != index_b)
                    {
                        collisions.push(object_b.clone());
                    }
                }
            }
        }

        let mut die = false;
        if !collisions.is_empty() {
            for collision in collisions.iter_mut() {
                match collision.obj_type {
                    ObjectType::None => rakeDebug!("Collided with nothing?"),
                    ObjectType::Food(value) => {
                        snake.size = snake.size + value ;
                        let mut rng = rand::rng();

                        let x = rng.random_range(0..grid.x) as i32;
                        let y = rng.random_range(0..grid.y) as i32;
                        rakeInfo!("Old Location: {}.{}", collision.x, collision.y);
                        rakeInfo!("New random Location: {x}.{y}");
                        food.body.x = rng.random_range(0..grid.x) as i32;
                        food.body.y = rng.random_range(0..grid.y) as i32;
                    },
                    ObjectType::Snake => {
                        // rakeInfo!("Collided with self.");
                        die = true;
                    }
                }
            }
        }
        if die {
            let _ = sink.send(Box::new(move |s| {
                RakeGUI::death_screen(s, grid);
            }));
            break;
        }

        let dir = snake.head.direction.unwrap().clone();
        let score = snake.clone().size;
        let _ = sink.send(Box::new(move |s: &mut Cursive| {
            let mut grid_c = grid.clone();
            
            RakeGUI::render_screen(s, grid_objects, &dir, &mut grid_c, score);
        }));
    });
}

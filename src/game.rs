use crate::{util, RakeGUI};
use rakeaudio::RakeAudioMessage;
use rakedisplay::DisplayMsg;
use rakelog::{rakeDebug, rakeError, rakeInfo};
use rakemodel::item::{self, Item, ItemType};
use rakemodel::shop::Shop;
use rakemodel::snake;
use rakemodel::{
    food::Food, grid::Grid, grid::GridObject, grid::ObjectType, snake::Snake, snake::SnakeDirection,
};

use cursive::{event::Key, CbSink, Cursive};
use rand::Rng;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use rodio::source::SineWave;
use rodio::OutputStream;
use rodio::Source;

pub struct Game {
    round: u32,
    round_score: u32,
    round_goal: u32,
}

impl Game {
    pub fn new(round: u32, round_goal: u32) -> Self {
        Self {
            round,
            round_score: 0,
            round_goal,
        }
    }
    pub fn init(
        sink: cursive::reexports::crossbeam_channel::Sender<Box<dyn FnOnce(&mut Cursive) + Send>>,
        display_r: Receiver<DisplayMsg>,
        display_s: Sender<DisplayMsg>,
        audio_s: Sender<RakeAudioMessage>,
    ) {
        thread::spawn(move || {
            let init_display_s: Sender<DisplayMsg> = display_s.clone();
            loop {
                match display_r.recv() {
                    Ok(msg) => {
                        rakeInfo!("got: {:#?}", msg);
                        match msg {
                            DisplayMsg::Start => {
                                let start_s = init_display_s.clone();
                                let audio_s_clone = audio_s.clone();
                                let _ = sink.send(Box::new(move |s: &mut Cursive| {
                                    Game::sandbox(s, Grid::new(30, 20), start_s, audio_s_clone);
                                }));
                            }
                            DisplayMsg::MainMenu => {
                                let main_menu_s = init_display_s.clone();
                                let _ = sink.send(Box::new(move |s: &mut Cursive| {
                                    RakeGUI::main_menu(s, main_menu_s);
                                }));
                            }
                        }
                    }
                    Err(e) => rakeError!("Error: {:#?}", e),
                }
            }
        });
    }

    fn reset_items(snake: Arc<Mutex<Snake>>) {
        let mut snake = snake.lock().unwrap();

        for item in snake.items.iter_mut() {
            match item.item_type {
                ItemType::Foody => item.trigger_count = 0,
                _ => rakeInfo!("uhhh"),
            }
        }
    }

    pub fn build_border_wall(grid: Grid) -> Vec<GridObject> {
        let mut wall_objs = Vec::new();

        for x in 0..grid.x as i32{
            for y in 0..grid.y as i32{
                if x == 0 || x == grid.x as i32 -1  || y == 0 || y == grid.y as i32 -1{
                    wall_objs.push(GridObject::new(x, y, '▮', ObjectType::Snake, None));
                }
            }
        }


        return wall_objs;
    }

    fn trigger_items(
        snake_m: Arc<Mutex<Snake>>,
        food_items: &mut Vec<Food>,
        grid: Grid,
        round_score: &mut i32,
    ) {
        let mut snake = snake_m.lock().unwrap();

        let mut triggered_items: Vec<Item> = Vec::new();
        let food_eaten = snake.new_food_eaten;
        for item in snake.items.iter_mut() {
            // rakeInfo!("triggering {:#?}", item);
            item.food_count += food_eaten;

            if !item.triggered {
                if item.item_type == ItemType::Foody {
                    if food_eaten > 0 {
                        item.trigger_count += 1
                    }
                }

                triggered_items.push(item.clone());

                match item.item_type {
                    ItemType::LongBoi => item.triggered = false,
                    ItemType::PhantomSnake => item.triggered = false,
                    ItemType::Shedding => item.triggered = false,
                    ItemType::GoldenSnack => item.triggered = false,
                    ItemType::Foody => item.triggered = false,
                    ItemType::Snackception => {
                        item.triggered = false;
                        if item.food_count == 5 {
                            item.food_count = 0;
                        }
                    }
                    _ => item.triggered = true,
                }
            }
        }

        for item in triggered_items {
            match item.item_type {
                ItemType::Double => {
                    for _ in 0..food_items.len() {
                        food_items.push(Food::new(grid, 1, 'x'))
                    }
                }
                ItemType::Snacks => {
                    for food in food_items.iter_mut() {
                        match food.body.obj_type {
                            ObjectType::Food(val) => {
                                rakeInfo!("Old value: {val}");
                                food.body.obj_type = ObjectType::Food(val * 2);
                                rakeInfo!("new value: {:#?}", food.body.obj_type);
                            }
                            _ => rakeError!("uhhh."),
                        }
                    }
                }

                ItemType::LongBoi => {
                    snake.size = 500;
                }

                ItemType::PhantomSnake => {
                    for obj in snake.body.iter_mut() {
                        obj.obj_type = ObjectType::None
                    }
                }

                ItemType::Shedding => {
                    for _ in 0..snake.new_food_eaten {
                        let mut rng = rand::rng();
                        let random_number = rng.random_range(0..10);
                        if random_number == 1 {
                            for _ in 0..(snake.size / 10) {
                                snake.body.pop();
                            }
                            snake.size -= (snake.size / 10);
                        }
                    }
                }
                ItemType::Foody => {
                    rakeInfo!("new Food {}", snake.new_food_eaten);
                    rakeInfo!("trigger count {}", item.trigger_count);
                    for _ in 0..snake.new_food_eaten {
                        for _ in 0..item.trigger_count {
                            rakeInfo!("Adding 1 to snake size");
                            snake.size += 1;
                            *round_score += 1;
                        }
                    }
                }
                ItemType::GoldenSnack => {
                    rakeInfo!(
                        "Snack check: {} > 0 & {} == 1",
                        snake.new_food_eaten,
                        item.food_count
                    );
                    if snake.new_food_eaten > 0 {
                        if item.food_count == 1 {
                            rakeError!("GOLDEN SNACK TRIGGERED!");
                            snake.size += 300;
                            *round_score += 300;
                        }
                    }
                }

                ItemType::Snackception => {
                    rakeInfo!("Food count {}", item.food_count);
                    if item.food_count == 5 {
                        for food in food_items.iter_mut() {
                            match food.body.obj_type {
                                ObjectType::Food(val) => {
                                    food.body.obj_type = ObjectType::Food(val + 1);
                                }
                                _ => rakeError!("uhhh."),
                            }
                        }
                    }
                }

                _ => rakeError!("NA"),
            }
        }
    }

    pub fn sandbox(
        s: &mut Cursive,
        grid: Grid,
        display_s: Sender<DisplayMsg>,
        audio_s: Sender<RakeAudioMessage>,
    ) {
        let high_score = util::read_score();
        let mut shop = Shop::new();
        let snake_m = Arc::new(Mutex::new(Snake::new(1, 1))).clone();
        snake_m.lock().unwrap().reset();;
        let snake = snake_m.clone();
        rakeInfo!("Grid Size: {}.{}", grid.x, grid.y);
        // let mut snake = Snake::new();
        s.pop_layer();
        let sink = s.cb_sink().clone();
        let delay = 150 as u64;

        let snake_clone_l = snake.clone();
        s.add_global_callback(Key::Left, move |_s| {
            let mut snake = snake_clone_l.lock().unwrap();
            if snake.head.direction != Some(SnakeDirection::Right) {
                snake.head.direction = Some(SnakeDirection::Left)
            }
        });

        let snake_clone_r = snake.clone();
        s.add_global_callback(Key::Right, move |_s| {
            let mut snake = snake_clone_r.lock().unwrap();
            if snake.head.direction != Some(SnakeDirection::Left) {
                snake.head.direction = Some(SnakeDirection::Right)
            }
        });

        let snake_clone_u = snake.clone();
        s.add_global_callback(Key::Up, move |_s| {
            let mut snake = snake_clone_u.lock().unwrap();
            if snake.head.direction != Some(SnakeDirection::Down) {
                snake.head.direction = Some(SnakeDirection::Up)
            }
        });

        let snake_clone_d = snake.clone();
        s.add_global_callback(Key::Down, move |_s| {
            let mut snake = snake_clone_d.lock().unwrap();
            if snake.head.direction != Some(SnakeDirection::Up) {
                snake.head.direction = Some(SnakeDirection::Down)
            }
        });

        let start_round = Arc::new(AtomicBool::new(true));

        let in_shop = Arc::new(AtomicBool::new(false));

        let start_round_clone = start_round.clone();
        s.add_global_callback(Key::Enter, move |_s| {
            start_round_clone.store(true, Ordering::Relaxed);
        });

        let mut food_items = Vec::new();
        let food = Food::new(grid, 10, 'o');
        food_items.push(food.clone());

        let mut round_goal = 2;
        let mut round_score = 0;
        // let mut money = Arc::0;
        let mut total_score = 0;
        let mut round = 1;

        // Having input handlers would require snake to be an Arc<Mutex<Snake>> :/
        thread::spawn(move || loop {
            let snake_clone = snake.clone();

            thread::sleep(Duration::from_millis(delay));

            if in_shop.load(Ordering::Relaxed) && start_round.load(Ordering::Relaxed) {
                start_round.store(false, Ordering::Relaxed);
                in_shop.store(false, Ordering::Relaxed);
                let start_round_clone_2 = start_round.clone();
                let mut items = Vec::new();

                let audio_s_clone = audio_s.clone();
                items.push(shop.get_shop_item().unwrap().clone());
                items.push(shop.get_shop_item().unwrap().clone());
                items.push(shop.get_shop_item().unwrap().clone());
                let _ = sink.send(Box::new(move |s: &mut Cursive| {
                    RakeGUI::shop(s, start_round_clone_2, items, snake_clone.clone(),audio_s_clone);
                }));
            }

            if start_round.load(Ordering::Relaxed) && !in_shop.load(Ordering::Relaxed) {
                let _ = sink.send(Box::new(move |s: &mut Cursive| {
                    s.pop_layer();
                }));

                Game::trigger_items(snake.clone(), &mut food_items, grid, &mut round_score);

                let mut snake = snake.lock().unwrap();
                snake.new_food_eaten = 0;
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

                let mut die = false;
                if (snake.head.x < 0 || snake.head.y < 0)
                    || (snake.head.x >= grid.x as i32 || snake.head.y >= grid.y as i32)
                {
                    die = true;
                }

                if !snake.body.is_empty() {
                    let old_snake = snake.clone();
                    for (index, body_node) in snake.body.iter_mut().enumerate() {
                        if index != 0 {
                            body_node.i = old_snake.body[index - 1].i;
                            body_node.x = old_snake.body[index - 1].x;
                            body_node.y = old_snake.body[index - 1].y;
                        } else {
                            body_node.i =
                                Snake::update_body(body_node.clone(), old_snake.head.clone());
                            body_node.direction = old_snake.head.direction;
                            body_node.x = old_snake.head.x;
                            body_node.y = old_snake.head.y;
                        }
                    }
                }

                // Not really sure if this is making it better :/
                // I cant really be bothered to fix this
                // But its clearly becasue I lock out the snake object for a stupid ammount of time
                // Probably could move ui updated to a different thread that can then access the snake data.
                std::mem::drop(snake);
                let mut snake = snake_m.lock().unwrap();

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


                // let mut wall_objs = Game::build_border_wall(grid);
                // grid_objects.append(&mut wall_objs);

                // grid_objects.push();

                for item in food_items.clone() {
                    grid_objects.push(item.body);
                }

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
                                collisions.push(*object_b);
                            }
                        }
                    }
                }

                if !collisions.is_empty() {
                    for collision in collisions.iter_mut() {
                        match collision.obj_type {
                            ObjectType::None => rakeInfo!("Collided with nothing?"),
                            ObjectType::Food(value) => {
                                // let _ = stream_handle.play_raw(source.clone().convert_samples());
                                let _ = audio_s.send(RakeAudioMessage::EatFood);
                                snake.size = snake.size + value;
                                round_score = round_score + value;
                                snake.new_food_eaten += 1;
                                for food in food_items.iter_mut() {
                                    if food.body.x == collision.x && food.body.y == collision.y {
                                        food.reset(grid);
                                    }
                                }
                            }
                            ObjectType::Snake => {
                                rakeInfo!("Collided with self");
                                die = true;
                            }
                        }
                    }
                }
                if die {
                    let _ = audio_s.send(RakeAudioMessage::Die);
                    for item in food_items.iter_mut() {
                        item.reset(grid);
                    }

                    total_score = total_score + round_score;

                    if total_score > high_score {
                        util::save_score(total_score);
                    }

                    if round_score >= round_goal {
                        start_round.store(false, Ordering::Relaxed);
                        let new_money = round_score / round_goal;
                        snake.add_lives(new_money - 1);
                        snake.money = snake.money + new_money;

                        snake.reset();
                        let _ = sink.send(Box::new(move |s| {
                            RakeGUI::round_win(s, new_money);
                        }));

                        round = round + 1;
                        round_goal =
                            round_goal + (round_goal / 2) + ((round_score / 2) * (new_money / 2));

                        round_score = 0;

                        if (round % 1) == 0 {
                            in_shop.store(true, Ordering::Relaxed);
                        }

                        // die = false;
                    } else {
                        snake.lives = snake.lives - 1;

                        if snake.lives <= 0 {
                            let _ = sink.send(Box::new(move |s| {
                                RakeGUI::death_screen(s, grid, display_s);
                                return;
                            }));
                            break;
                        } else {
                            start_round.store(false, Ordering::Relaxed);
                            snake.reset();
                            let lives = snake.lives;
                            let _ = sink.send(Box::new(move |s| {
                                RakeGUI::round_failed(s, lives);
                            }));
                            round_score = 0;
                        }
                    }
                } else {
                    let dir = snake.head.direction.unwrap().clone();
                    let lives = snake.lives;
                    let money = snake.money;
                    let length = snake.size;
                    let items = snake.items.clone();
                    let _ = sink.send(Box::new(move |s: &mut Cursive| {
                        let mut grid_c = grid.clone();
                        RakeGUI::render_screen(
                            s,
                            grid_objects,
                            &dir,
                            &mut grid_c,
                            round_score,
                            high_score,
                            round_goal,
                            money,
                            total_score,
                            round,
                            lives,
                            length,
                            items,
                        );
                    }));
                }
            };
        });
    }
}

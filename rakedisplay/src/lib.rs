use std::fmt::format;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use cursive::event::Key;
use cursive::view::Margins;
use cursive::views::{Button, Panel, TextView};
use cursive::views::{Dialog, LinearLayout};
use cursive::Cursive;
use rakelog::{rakeDebug, rakeInfo};
use rakemodel::item::Item;
use rakemodel::shop::Shop;
use rakemodel::snake;
use rakemodel::{
    grid::{Grid, GridObject, ObjectType},
    snake::{Snake, SnakeDirection},
};
use rand::Rng;

pub struct RakeGUI {
    pub siv: Cursive,
}

#[derive(Debug, Clone, Copy)]
pub enum DisplayMsg {
    Start,
    MainMenu,
}

impl RakeGUI {
    pub fn new() -> Self {
        let siv = Cursive::new();
        Self { siv }
    }

    pub fn main_menu(s: &mut Cursive, display_s: Sender<DisplayMsg>) {
        rakeDebug!("loading rake start screen");
        s.pop_layer();

        s.add_layer(
            LinearLayout::vertical()
                .child(TextView::new(" R A K E"))
                .child(Panel::new(
                    LinearLayout::vertical()
                        .child(Button::new("Start", move |s| {
                            display_s.send(DisplayMsg::Start).unwrap()
                        }))
                        // .child(Button::new("Info", |s| rakeInfo!("Info button pushed")))
                        .child(Button::new("shoptest", |s| {
                            let mut shop = Shop::new();
                            let mut items = Vec::new();
                            let snake = Arc::new(Mutex::new(Snake::new(0, 0)));
                            items.push(shop.get_shop_item().unwrap().clone());
                            items.push(shop.get_shop_item().unwrap().clone());
                            items.push(shop.get_shop_item().unwrap().clone());
                            RakeGUI::shop(
                                s,
                                Arc::new(AtomicBool::new(false)),
                                items,
                                snake.clone(),
                            );
                            let snake_lock = snake.lock().unwrap();
                            rakeInfo!("Snake Items: {:#?}", snake_lock.items);
                        }))
                        .child(Button::new("Quit", |s| s.quit())),
                )),
        );
    }

    pub fn death_screen(s: &mut Cursive, grid: Grid, display_s: Sender<DisplayMsg>) {
        let mut display_mm_s = display_s.clone();
        let mut display_rp_s = display_s.clone();

        s.add_layer(
            Dialog::text("YOU DIED!")
                .button("Main Menu", move |s| {
                    s.pop_layer();
                    let _ = display_mm_s.send(DisplayMsg::MainMenu);
                })
                .button("Replay", move |s| {
                    s.pop_layer();
                    let _ = display_rp_s.clone().send(DisplayMsg::Start);
                }),
        );
    }

    pub fn render_screen(
        s: &mut Cursive,
        objects: Vec<GridObject>,
        snake_direction: &SnakeDirection,
        grid: &mut Grid,
        score: i32,
        high_score: i32,
        round_goal: i32,
        money: i32,
        total_score: i32,
        round: i32,
        lives: i32,
    ) {
        s.pop_layer();

        // let title = format!("{:#?}", snake_direction);

        let game_window = Dialog::text(grid.gen_grid(objects))
            .padding(Margins::lrtb(0, 0, 0, 0))
            .title("R A K E");

        let mut map: Vec<char> = "----S".repeat(40).chars().collect();
        map[79] = 'B';
        let start = (round - 1) as usize;
        // This is based off of the grid size
        let end = start + 30;
        let current_map: String = map[start as usize..end].iter().collect();

        let mini_map_window =
            Dialog::text(format!("{current_map}\n^")).title(format!("Round Tracker"));

        let live_string = "(=)".repeat(lives as usize);
        let lives_window: Dialog = Dialog::text(format!("{live_string}")).title("Lives");
        let info_window = Dialog::text(format!(
            "Round: {round}\nCoins: {money}\nRun Score: {total_score}"
        ));

        let item_one = Dialog::text("Every 10 food chop the snake in half").title("Shears");
        let item_two = Dialog::text("Start each round with a snake length of 500").title("LongBoi");
        let item_three = Dialog::text("Snake Cannot collide with itself.").title("PhantomSnake");
        let item_four = Dialog::text("Every 5 snacks increases the value of a snack by 1")
            .title("Snackception");

        let round_goal_window = Dialog::text(format!("{score}/{round_goal}")).title("Round Goal");

        let item_pane = LinearLayout::vertical()
            .child(item_one)
            .child(item_two)
            .child(item_three)
            .child(item_four);

        let info_pane = LinearLayout::vertical()
            .child(round_goal_window)
            .child(lives_window)
            .child(info_window);

        s.add_layer(
            LinearLayout::vertical().child(
                LinearLayout::horizontal()
                    .child(
                        LinearLayout::vertical()
                            .child(game_window)
                            .child(mini_map_window),
                    )
                    .child(info_pane),
            ),
        );
    }

    pub fn shop(
        s: &mut Cursive,
        start_round: Arc<AtomicBool>,
        items: Vec<Item>,
        snake: Arc<Mutex<Snake>>,
    ) {
        let item_one = items.get(0).unwrap().clone();
        let item_two = items.get(1).unwrap().clone();
        let item_three = items.get(2).unwrap().clone();

        let snake_a = snake.clone();
        let snake_b = snake.clone();
        let snake_c = snake.clone();

        let item1 = Dialog::text(format!("{}", item_one.description))
            .title(format!("{}", item_one.item_name))
            .button(format!("{} Coins", item_one.value), move |_s| {
                let mut snake = snake_a.lock().unwrap();
                if snake.money >= item_one.value {
                    snake.money = snake.money - item_one.value;
                    snake.items.push(item_one.clone());
                }
            });

        let item2 = Dialog::text(format!("{}", item_two.description))
            .title(format!("{}", item_two.item_name))
            .button(format!("{} Coins", item_two.value), move |_s| {
                let mut snake = snake_b.lock().unwrap();
                if snake.money >= item_two.value {
                    snake.money = snake.money - item_two.value;
                    snake.items.push(item_two.clone());
                }
            });

        let item3 = Dialog::text(format!("{}", item_three.description))
            .title(format!("{}", item_three.item_name))
            .button(format!("{} Coins", item_three.value), move |_s| {
                let mut snake = snake_c.lock().unwrap();
                if snake.money >= item_three.value {
                    snake.money = snake.money - item_three.value;
                    snake.items.push(item_three.clone());
                }
            });

        let exit = Button::new("Exit Shop", move |_s| {
            start_round.store(true, std::sync::atomic::Ordering::Relaxed);
        });

        s.add_layer(
            LinearLayout::vertical()
                .child(Dialog::text("").title("R A K E SHOP"))
                .child(
                    LinearLayout::horizontal()
                        .child(item1)
                        .child(item2)
                        .child(item3),
                )
                .child(exit),
        );
    }

    pub fn iteminfo(s: &mut Cursive, item_name: String, item_decription: String) {}

    pub fn round_win(s: &mut Cursive, new_money: i32) {
        s.add_layer(Dialog::text(format!("+ {new_money} coins")).title("Round Win"));
    }

    pub fn round_failed(s: &mut Cursive, lives: i32) {
        s.add_layer(Dialog::text(format!("{lives} lives left")).title("Round Failed"));
    }
}

pub fn start(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("Starting..."));
}

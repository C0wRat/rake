use std::fmt::format;
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
use rakemodel::food::food;
use rakemodel::{grid::{Grid,GridObject,ObjectType}, snake::{Snake, SnakeDirection}};
use rand::Rng;

pub struct RakeGUI {
    pub siv: Cursive,
}

#[derive(Debug, Clone, Copy)]
pub enum DisplayMsg{
    Start,
    MainMenu,
}

impl RakeGUI {
    pub fn new() -> Self {
        let siv = Cursive::new();
        Self { siv }
    }

    pub fn main_menu(s: &mut Cursive, display_s: Sender::<DisplayMsg>) {
        rakeDebug!("loading rake start screen");
        s.pop_layer();

        s.add_layer(LinearLayout::vertical().child(TextView::new(" R A K E"))
            .child(Panel::new(LinearLayout::vertical()
                .child(Button::new("Start", move|s| display_s.send(DisplayMsg::Start).unwrap()))
                // .child(Button::new("Info", |s| rakeInfo!("Info button pushed")))
                // .child(Button::new("Help", |s| rakeInfo!("HELP button pushed")))
                .child(Button::new("Quit", |s| s.quit()))
                )
            )
        );
    }

    pub fn death_screen(s: &mut Cursive, grid: Grid, display_s: Sender::<DisplayMsg>) {
        let mut display_mm_s = display_s.clone();
        let mut display_rp_s = display_s.clone();
        
        s.add_layer(
            Dialog::text("YOU DIED!")
                .button("Main Menu", move|s| {
                    s.pop_layer();
                    let _ = display_mm_s.send(DisplayMsg::MainMenu);
                })
                .button("Replay", move|s| {
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
        round: i32
    ) {
        s.pop_layer();

        // let title = format!("{:#?}", snake_direction);
        s.add_layer(
            LinearLayout::horizontal()
            .child( LinearLayout::vertical()
            .child(Dialog::text(grid.gen_grid(objects))
            .padding(Margins::lrtb(0, 0, 0, 0))
            .title("R A K E"))
            .child(Dialog::text(format!("High Score: {high_score}")).title(format!("score:{score}")))
    ).child(Dialog::text(format!("Round: {round}\nRound Goal: {score}/{round_goal}\nMoney: {money}\nTotal Score: {total_score}"))));
       
    }

    pub fn round_win(s: &mut Cursive, new_money: i32){
        s.add_layer(Dialog::text(format!("Round Complete +${new_money}!")).title("Round Win"));
    }
}

pub fn start(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("Starting..."));
}

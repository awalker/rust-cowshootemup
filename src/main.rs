use std::rc::Rc;

use anyhow::Result;
use cowshmup::{
    drawable::{Drawable, Graphic},
    state::{ExitState, ModalState, State},
    world::{RcWorld, World},
};
use macroquad::{input, prelude::*};

#[derive(Debug, Default, Clone)]
pub struct MainState {
    world: RcWorld,
    fps: i32,
}

impl State for MainState {
    fn update(self: Box<Self>) -> Box<dyn State> {
        let mut new_state = self.clone();
        new_state.fps = get_fps();
        if input::is_key_pressed(KeyCode::Escape) {
            return Box::new(ExitState);
        }
        if input::is_key_pressed(KeyCode::Space) {
            return ModalState::new(Box::new(PausedState::default()), new_state);
        }
        new_state
    }

    fn draw(&self) {
        clear_background(RED);

        for d in self.world.iter() {
            d.draw()
        }

        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_text(&format!("HELLO {}", self.fps), 20.0, 20.0, 20.0, DARKGRAY);
    }
}

#[derive(Debug, Default, Clone)]
pub struct PausedState(bool);

impl State for PausedState {
    fn update(self: Box<Self>) -> Box<dyn State> {
        if input::is_key_pressed(KeyCode::Escape) {
            Box::new(PausedState(true))
        } else {
            self
        }
    }

    fn draw(&self) {
        draw_text("Paused", 120.0, 120.0, 20.0, WHITE);
    }

    fn should_continue(&self) -> bool {
        !self.0
    }
}

#[macroquad::main("OMG Cows")]
async fn main() -> Result<()> {
    flexi_logger::Logger::try_with_env_or_str("warn")?.start()?;
    log::info!("Hello, World!");
    let mut world = World::default();
    world.add_item(Graphic::line(40.0, 40.0, 100.0, 200.0, BLUE));
    world.add_item(Graphic::circle(
        (screen_width() - 30.0, screen_height() - 30.0).into(),
        15.0,
        YELLOW,
    ));
    let mut state: Box<dyn State> = Box::from(MainState {
        world: Rc::from(world),
        ..MainState::default()
    });
    while state.should_continue() {
        state.draw();
        next_frame().await;
        state = state.update();
    }
    Ok(())
}

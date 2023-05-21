use std::rc::Rc;

use anyhow::Result;
use cowshmup::{
    drawable::{Drawable, Graphic},
    state::{ExitState, ModalState, NextState, State},
    world::{RcWorld, World},
};
use macroquad::{input, prelude::*};

#[derive(Default, Clone, Debug)]
pub struct MainState {
    world: RcWorld,
    fps: i32,
    next_state: NextState,
}

impl State for MainState {
    fn transition(&self) -> Option<Box<dyn State>> {
        self.next_state.take()
    }

    fn update(&mut self) {
        if input::is_key_pressed(KeyCode::Escape) {
            self.next_state = NextState::boxed(ExitState);
        } else if input::is_key_pressed(KeyCode::Space) {
            self.next_state = NextState::some(ModalState::new(
                Box::new(PausedState::default()),
                Box::new(self.clone()),
            ));
        }
        self.fps = get_fps();
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
    fn update(&mut self) {
        self.0 = input::is_key_pressed(KeyCode::Escape);
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
        state.update();
        if let Some(new_state) = state.transition() {
            state = new_state;
        }
    }
    Ok(())
}

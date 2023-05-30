use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use cowshmup::{
    drawable::{Drawable, Graphic},
    particle::CircleParticle,
    state::{ExitState, ModalState, NextState, State},
    updateable::Updateable,
    world::{RcWorld, World},
};
use macroquad::{input, prelude::*};

#[derive(Default, Clone, Debug)]
pub struct GameData {
    world: RcWorld,
    fps: i32,
    next_state: NextState,
    time: f32,
}

impl State for GameData {
    fn transition(&self) -> Option<Box<dyn State>> {
        self.next_state.take()
    }
}

impl Updateable for GameData {
    fn update(&mut self, delta_time: f32) {
        self.time += delta_time;
        if input::is_key_pressed(KeyCode::Escape) {
            self.next_state = NextState::boxed(ExitState);
        } else if input::is_key_pressed(KeyCode::Space) {
            self.next_state = NextState::some(ModalState::new(
                Box::new(PausedState::default()),
                Box::new(self.clone()),
            ));
        }
        self.world.update(delta_time);
        self.fps = get_fps();
    }
}

impl Drawable for GameData {
    fn draw(&self) {
        clear_background(RED);

        self.world.draw();

        let mut x = 20.0 * self.time;
        if x > 60.0 {
            x = 60.0
        }
        draw_rectangle(screen_width() / 2.0 - x, 100.0, 120.0, 60.0, GREEN);
        draw_text(&format!("HELLO {}", self.fps), 20.0, 20.0, 20.0, DARKGRAY);
        draw_text(&format!("TIME {}", self.time), 20.0, 40.0, 20.0, DARKGRAY);
    }
}

#[derive(Debug, Default, Clone)]
pub struct PausedState(bool);

impl State for PausedState {
    fn should_continue(&self) -> bool {
        !self.0
    }
}

impl Updateable for PausedState {
    fn update(&mut self, _delta_time: f32) {
        self.0 = input::is_key_pressed(KeyCode::Escape);
    }
}

impl Drawable for PausedState {
    fn draw(&self) {
        draw_text("Paused", 120.0, 120.0, 20.0, WHITE);
    }
}

#[macroquad::main("OMG Cows")]
async fn main() -> Result<()> {
    flexi_logger::Logger::try_with_env_or_str("warn")?.start()?;
    log::info!("Hello, World!");
    let mut world = World::default();
    world.add_graphic(Graphic::line(40.0, 40.0, 100.0, 200.0, BLUE));
    let part = CircleParticle::new(
        (screen_width() - 30.0, screen_height() - 30.0).into(),
        15.0,
        YELLOW,
    )
    .with_velocity((-10., -10.).into());

    world.add_particle(Box::new(part));
    let mut state: Box<dyn State> = Box::from(GameData {
        world: Rc::from(RefCell::new(world)),
        ..GameData::default()
    });
    while state.should_continue() {
        state.update(get_frame_time());
        state.draw();
        next_frame().await;
        if let Some(new_state) = state.transition() {
            state = new_state;
        }
    }
    Ok(())
}

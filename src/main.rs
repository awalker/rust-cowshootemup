use std::rc::Rc;

use anyhow::Result;
use cowshmup::{
    drawable::{Drawable, Graphic},
    state::{ExitState, State},
    world::World,
};
use macroquad::{input, prelude::*};

#[derive(Debug, Default, Clone)]
pub struct MainState {
    world: Rc<World>,
}

impl State for MainState {
    fn update(self: Box<Self>) -> Box<dyn State> {
        if input::is_key_pressed(KeyCode::Escape) {
            return ExitState::new(self);
        }
        self
    }

    fn draw(&self) {
        clear_background(RED);

        for d in self.world.iter() {
            d.draw()
        }

        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_text(&format!("HELLO {}", get_fps()), 20.0, 20.0, 20.0, DARKGRAY);
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
        state = state.update();
        state.draw();
        next_frame().await
    }
    Ok(())
}

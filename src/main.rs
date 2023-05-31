/// Cow Shoot 'em up in Rust
use std::{cell::RefCell, f32::consts::PI, matches, rc::Rc};

use anyhow::Result;
use cowshmup::{
    drawable::{Drawable, Graphic},
    particle::Explosion,
    updateable::Updateable,
    world::{RcWorld, World},
};
use macroquad::{input, prelude::*};

#[derive(Debug, Default, Clone, Copy)]
enum State {
    #[default]
    Init,
    Playing,
    Paused,
    Exit,
}

#[derive(Default, Clone, Debug)]
pub struct GameData {
    world: RcWorld,
    fps: i32,
    time: f32,
    state: State,
}

impl GameData {
    fn update_game(&mut self, delta_time: f32) {
        self.time += delta_time;
        self.handle_common_input(delta_time);
        self.world.update(delta_time);
        self.fps = get_fps();
    }

    fn update_paused(&mut self, delta_time: f32) {
        self.handle_common_input(delta_time);
    }

    fn handle_common_input(&mut self, _delta_time: f32) {
        if input::is_key_pressed(KeyCode::Escape) {
            self.press_escape();
        } else if input::is_key_pressed(KeyCode::Space) {
            self.press_space();
        }
    }

    fn draw_game(&self) {
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

    fn draw_paused(&self) {
        draw_text("Paused", 120.0, 120.0, 20.0, WHITE);
    }

    fn press_escape(&mut self) {
        match self.state {
            State::Playing => self.state = State::Exit,
            State::Paused => self.state = State::Playing,
            _ => {}
        }
    }

    fn press_space(&mut self) {
        if self.state.is_playing() {
            self.state = State::Paused
        }
    }
}

impl Updateable for GameData {
    fn update(&mut self, delta_time: f32) {
        match self.state {
            State::Init => self.state = State::Playing,
            State::Playing => self.update_game(delta_time),
            State::Paused => self.update_paused(delta_time),
            State::Exit => {}
        }
    }
}

impl Drawable for GameData {
    fn draw(&self) {
        match self.state {
            State::Init => {}
            State::Playing => self.draw_game(),
            State::Paused => {
                self.draw_game();
                self.draw_paused();
            }
            State::Exit => self.draw_game(),
        }
    }
}

impl State {
    fn is_exit(&self) -> bool {
        matches!(self, State::Exit)
    }

    fn is_playing(&self) -> bool {
        matches!(self, State::Playing)
    }
}

#[macroquad::main("OMG Cows")]
async fn main() -> Result<()> {
    flexi_logger::Logger::try_with_env_or_str("warn")?.start()?;
    log::info!("Hello, World!");
    let mut world = World::default();
    world.add_graphic(Graphic::line(40.0, 40.0, 100.0, 200.0, BLUE));
    let part = Explosion::begin((screen_width() / 2.0, screen_height() / 2.0).into())
        .with_age(5., 5.)
        .with_radius(8., 128.)
        .with_circle_stage()
        .with_radius(32., 64.)
        .with_count(3, 4)
        .with_angle(PI, PI * 2.)
        .with_dist(10., 25.)
        .with_color(BROWN)
        .with_circle_stage()
        .build();
    /* 64.0,
    YELLOW, */

    world.add_particle(Box::new(part));
    let mut game = GameData {
        world: Rc::from(RefCell::new(world)),
        ..GameData::default()
    };

    while !game.state.is_exit() {
        game.update(get_frame_time());
        game.draw();
        next_frame().await;
    }
    Ok(())
}

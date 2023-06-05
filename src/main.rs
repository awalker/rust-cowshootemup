#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
/// Cow Shoot 'em up in Rust
mod editor;
mod state;
use std::{
    cell::RefCell,
    fs::File,
    io::{BufReader, BufWriter},
    matches,
    rc::Rc,
};

use anyhow::{Context, Result};
use cowshmup::{
    drawable::{Drawable, Graphic},
    particle::Explosion,
    updateable::Updateable,
    world::{RcWorld, World},
};
use editor::Editor;
use macroquad::{input, prelude::*};
use state::State;

#[derive(Default, Clone, Debug)]
pub struct GameData {
    world: RcWorld,
    fps: i32,
    time: f32,
    state: State,
    gizmos: bool,
}

impl GameData {
    fn update_game(&mut self, delta_time: f32) {
        self.time += delta_time;
        self.handle_common_input(delta_time);
        self.world.update(delta_time);
        self.fps = get_fps();
    }

    fn is_editor(&self) -> bool {
        self.state.is_editor()
    }

    fn update_paused(&mut self, delta_time: f32) {
        self.handle_common_input(delta_time);
    }

    fn handle_common_input(&mut self, _delta_time: f32) {
        self.step();
        if input::is_key_pressed(KeyCode::E) {
            self.state = State::Editor;
        }
        if input::is_key_pressed(KeyCode::C) {
            self.gizmos = !self.gizmos;
        }
        if input::is_key_pressed(KeyCode::Escape) {
            self.press_escape();
        } else if input::is_key_pressed(KeyCode::Space) {
            self.press_space();
        }
    }

    fn step(&mut self) {
        if matches!(self.state, State::Step | State::StepAdvance) {
            if input::is_key_down(KeyCode::S) {
                self.state = State::StepAdvance;
            }
            if input::is_key_released(KeyCode::S) {
                self.state = State::Step
            }
            if input::is_key_pressed(KeyCode::G) {
                self.state = State::Playing
            }
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
        draw_text(
            &format!("Gizmos {}", self.gizmos),
            20.0,
            60.0,
            20.0,
            DARKGRAY,
        );
        draw_text(
            &format!("State {:?}", self.state),
            20.0,
            80.0,
            20.0,
            DARKGRAY,
        );
    }

    fn draw_paused(&self) {
        draw_text("Paused", 120.0, 120.0, 20.0, WHITE);
    }

    fn draw_step(&self) {
        draw_text("Press s to Step", 120.0, 120.0, 20.0, WHITE);
    }

    fn press_escape(&mut self) {
        match self.state {
            State::Playing => self.state = State::Exit,
            State::Paused => self.state = State::Playing,
            State::Step => self.state = State::Exit,
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
            State::Init => self.state = State::Editor,
            State::Playing => self.update_game(delta_time),
            State::Paused => self.update_paused(delta_time),
            State::Step => self.handle_common_input(delta_time),
            State::StepAdvance => {
                self.update_game(delta_time);
            }
            State::Exit => {}
            State::Editor => {}
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
            State::Step | State::StepAdvance => {
                self.draw_game();
                self.draw_step();
            }
            State::Exit => self.draw_game(),
            State::Editor => {
                self.draw_game();
            }
        }
    }

    fn draw_gizmos(&self) {
        if self.gizmos {
            self.world.draw_gizmos();
        }
    }
}

fn load_editor() -> anyhow::Result<Editor> {
    let rdr =
        BufReader::new(File::open("editor.yaml").with_context(|| "Could not open editor.yaml")?);
    serde_yaml::from_reader::<_, Editor>(rdr).with_context(|| "could not parse editor.yaml")
}

#[macroquad::main("OMG Cows")]
async fn main() -> Result<()> {
    flexi_logger::Logger::try_with_env_or_str("warn")?.start()?;
    log::info!("Hello, World!");
    let mut editor = match load_editor() {
        Err(err) => {
            // TODO: Result error
            log::warn!("Unable to load editor: {:#?}", err);
            Editor::default()
        }
        Ok(v) => v,
    };
    editor.init();
    let mut world = World::default();
    world.add_graphic(Graphic::line(40.0, 40.0, 100.0, 200.0, BLUE));

    // let mut part = Explosion::begin((screen_width() / 2.0, screen_height() / 2.0).into());
    // let stage = part
    // .build_stage()
    // .with_age(5., 5.)
    // .with_radius(80., 80.)
    // .with_circle_stage(&mut part);
    // editor.explosion_stages.push(stage.clone());
    /* world.add_gizmos(Rc::new(stage.clone()));
    let stage = part
        .build_stage()
        .with_angle(PI * 0.65, PI * 1.35)
        .with_delay(2., 4.)
        .with_radius(32., 64.)
        .with_count(3, 4)
        .with_dist(10., 25.)
        .with_color(BROWN)
        .with_circle_stage(&mut part);
    editor.explosion_stages.push(stage.clone());
    world.add_gizmos(Rc::new(stage.clone()));
    let part = part.build();
    world.add_particle(Box::new(part)); */

    let mut game = GameData {
        world: Rc::from(RefCell::new(world)),
        gizmos: true,
        ..GameData::default()
    };

    let mut explosion: Option<Explosion> = None;

    while !game.state.is_exit() {
        let mut delta_time = get_frame_time();
        let old_time = game.time;
        egui_macroquad::ui(|egui_ctx| {
            if game.is_editor() {
                editor.time = old_time;
                editor.update_egui(egui_ctx, delta_time);
                game.gizmos = editor.show_gizmos;
                if let Some(new_state) = editor.state.take() {
                    game.state = new_state;
                }
                let new_time = editor.time;
                delta_time = new_time - old_time;
                if let Some(editor) = editor.build_explosion(cowshmup::CenterPt::new(100.0, 200.0))
                {
                    explosion = Some(editor)
                }
            }
        });
        game.update(delta_time);
        game.draw();
        if let Some(mut exp) = explosion {
            exp.update(delta_time);
            exp.draw();
            explosion = Some(exp)
        }
        if game.is_editor() {
            editor.draw_gizmos();
        }
        game.draw_gizmos();
        egui_macroquad::draw();
        next_frame().await;
    }
    // TODO: Should probably support manually loading and saving, instead of always auto-saving...
    // Or maybe both...
    serde_yaml::to_writer(BufWriter::new(File::create("editor.yaml")?), &editor)?;
    Ok(())
}

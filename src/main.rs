#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
/// Cow Shoot 'em up in Rust
mod editor;
mod state;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    matches,
};

use anyhow::{Context, Result};
use cowshmup::{
    drawable::{Drawable, Graphic},
    particle::Explosion,
    updateable::Updateable,
    world::{World, GAME_HEIGHT, GAME_WIDTH},
};
use editor::Editor;
use macroquad::{input, prelude::*};
use state::State;

#[derive(Default, Debug)]
pub struct GameData {
    world: World,
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

        // FIXME: Debug info in egui instead?
        let font_size = 10.;
        let y = font_size;
        draw_text(&format!("HELLO {}", self.fps), 0.0, y, font_size, DARKGRAY);
        let y = y + font_size;
        draw_text(&format!("TIME {}", self.time), 0.0, y, font_size, DARKGRAY);
        let y = y + font_size;
        draw_text(
            &format!("Gizmos {}", self.gizmos),
            0.0,
            y,
            font_size,
            DARKGRAY,
        );
        let y = y + font_size;
        draw_text(
            &format!("State {:?}", self.state),
            0.0,
            y,
            font_size,
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

    let mut game = GameData {
        world,
        gizmos: true,
        ..GameData::default()
    };

    let mut explosion: Option<Explosion> = None;

    let editor_object_center = cowshmup::CenterPt::new(GAME_WIDTH / 2., GAME_HEIGHT / 2.);
    let render_target = render_target(GAME_WIDTH as u32, GAME_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Nearest);
    let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., GAME_WIDTH, GAME_HEIGHT));
    camera.render_target = Some(render_target);
    camera.zoom.y *= -1.;
    let mut zoom;

    while !game.state.is_exit() {
        let mut game_canvas = Rect::new(0., 0., screen_width(), screen_height());
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
                if let Some(editor) = editor.build_explosion(editor_object_center) {
                    explosion = Some(editor)
                }
                let avail = egui_ctx.available_rect();
                game_canvas.y = avail.top();
                game_canvas.h -= game_canvas.y;
                game_canvas.w = avail.width();
                game_canvas.x = avail.left();
            }
        });
        zoom = (game_canvas.w / GAME_WIDTH).floor();
        zoom = zoom.min((game_canvas.h / GAME_HEIGHT).floor());
        game_canvas.x += (game_canvas.w - (GAME_WIDTH * zoom)) / 2.;
        game_canvas.y += (game_canvas.h - (GAME_HEIGHT * zoom)) / 2.;
        game.update(delta_time);
        push_camera_state();
        set_camera(&camera);
        game.draw();
        if let Some(mut exp) = explosion {
            exp.update(delta_time);
            exp.draw();
            explosion = Some(exp)
        }
        if game.is_editor() {
            editor.draw_gizmos_at(editor_object_center);
        }
        game.draw_gizmos();
        pop_camera_state();
        draw_texture_ex(
            render_target.texture,
            game_canvas.x,
            game_canvas.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(GAME_WIDTH * zoom, GAME_HEIGHT * zoom)),
                ..Default::default()
            },
        );
        egui_macroquad::draw();
        next_frame().await;
    }
    // TODO: Should probably support manually loading and saving, instead of always auto-saving...
    // Or maybe both...
    serde_yaml::to_writer(BufWriter::new(File::create("editor.yaml")?), &editor)?;
    Ok(())
}

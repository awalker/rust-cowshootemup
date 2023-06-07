#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
/// Cow Shoot 'em up in Rust
mod editor;
mod game_data;
mod prelude;
mod state;
use cowshmup::particle::Explosion;
use editor::Editor;
use prelude::*;
use state::State;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use crate::game_data::GameData;

fn load_editor() -> anyhow::Result<Editor> {
    let rdr =
        BufReader::new(File::open("editor.yaml").with_context(|| "Could not open editor.yaml")?);
    serde_yaml::from_reader::<_, Editor>(rdr).with_context(|| "could not parse editor.yaml")
}

#[macroquad::main("OMG Cows")]
async fn main() -> Result<()> {
    info!("Hello, World!");
    let mut editor = match load_editor() {
        Err(err) => {
            // TODO: Result error
            warn!("Unable to load editor: {:#?}", err);
            Editor::default()
        }
        Ok(v) => v,
    };
    editor.init();
    let mut world = World::default();
    world.add_graphic(Graphic::line(40.0, 40.0, 100.0, 200.0, BLUE));

    let mut game = GameData {
        world,
        show_gizmos: true,
        show_editor: true,
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
        let delta_time = get_frame_time();
        egui_macroquad::ui(|egui_ctx| {
            if game.is_editor() {
                editor.update_egui(egui_ctx, &mut game);
                if editor.re_add_objects_to_game {
                    if let Some(editor) = editor.build_explosion(editor_object_center) {
                        explosion = Some(editor)
                    }
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
        clear_background(BLACK);
        set_camera(&camera);
        game.draw();
        if let Some(mut exp) = explosion {
            exp.update(delta_time);
            exp.draw();
            explosion = Some(exp)
        }
        if game.show_gizmos && game.is_editor() {
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

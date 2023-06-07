#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
/// Cow Shoot 'em up in Rust
mod editor;
mod game_data;
mod prelude;
mod preview;
mod state;
use cowshmup::{particle::Explosion, retro_camera::RetroCamera};
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

    // EDITOR SETUP
    let mut editor = match load_editor() {
        Err(err) => {
            // TODO: Result error
            warn!("Unable to load editor: {:#?}", err);
            Editor::default()
        }
        Ok(v) => v,
    };
    editor.init();

    // GAME SETUP
    let mut world = World::default();
    world.add_graphic(Graphic::line(40.0, 40.0, 100.0, 200.0, BLUE));
    let mut game = GameData {
        world,
        show_gizmos: true,
        show_editor: true,
        ..GameData::default()
    };

    // TEMP EDITOR STUFF
    let mut explosion: Option<Explosion> = None;
    let editor_object_center = cowshmup::CenterPt::new(GAME_WIDTH / 2., GAME_HEIGHT / 2.);

    // Retro Camera Setup
    let mut retrocam = RetroCamera::default();

    // GAME LOOP
    while !game.state.is_exit() {
        // TIMING
        game.frame_time = get_frame_time();

        // EGUI + EDITOR STUFF (egui may be used for more than editor)
        egui_macroquad::ui(|egui_ctx| {
            if game.is_editor() {
                editor.update_egui(egui_ctx, &mut game);
                // TODO: TEMP: WANT TO REWRITE...
                if editor.re_add_objects_to_game {
                    if let Some(editor) = editor.build_explosion(editor_object_center) {
                        explosion = Some(editor)
                    }
                }
            }
            // NOTE: Game Canvas is being reset by egui, if we want to disable egui for use
            // release, with will need to reset the canvas to 0,0,screen width, height
            retrocam.reset_canvas(egui_ctx);
        });

        // UPDATE GAME
        game.update(game.frame_time);

        // Adjust Cameras and Canvas...
        clear_background(BLACK);
        retrocam.setup_camera();

        // DRAW (to texture/Retro Camera)
        game.draw();
        // TEMP EDITOR PREVIEW...
        if let Some(mut exp) = explosion {
            exp.update(game.frame_time);
            exp.draw();
            explosion = Some(exp)
        }
        if game.show_gizmos {
            editor.draw_gizmos_at(editor_object_center);
        }
        game.draw_gizmos();

        // DRAW at native rez (stretch Retro Camera, then render egui)
        retrocam.render();
        egui_macroquad::draw();

        // Finally wait for next frame...
        next_frame().await;
    }

    // GAME LOOP EXITED
    // TODO: Should probably support manually loading and saving, instead of always auto-saving...
    // Or maybe both...
    serde_yaml::to_writer(BufWriter::new(File::create("editor.yaml")?), &editor)?;
    Ok(())
}

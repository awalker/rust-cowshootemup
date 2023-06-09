use std::collections::HashMap;

use crate::preview::{Preview, PreviewBuildableData};
use crate::State;
use crate::{game_data::GameData, prelude::*};
use cowshmup::particle::ExplosionBuilder;
use macroquad::rand;

/// Editor represents an editor for various ascpects of the game. An editor can be serialized so
/// that it opens in the same state again. An editor can operate on, but not include, `GameData`
/// because game data should not be serialized as part of the editor.
#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Editor {
    /// Things we edit may use randomness, we need to reset that every editor frame
    seed: Option<u64>,
    pub re_add_objects_to_game: bool,
    pub show_debug: bool,
    pub show_properties: bool,

    pub previews: HashMap<EditorPreview, PreviewMeta>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct PreviewMeta {
    opened: bool,
    #[serde(skip)] // for now
    preview: Option<Box<dyn Preview>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord, Clone, Copy)]
pub enum EditorPreview {
    Explosion,
}

impl EditorPreview {
    fn create_preview(&self) -> Box<dyn Preview> {
        match self {
            EditorPreview::Explosion => Box::<PreviewBuildableData<ExplosionBuilder>>::default(),
        }
    }
    fn get_name(&self) -> &str {
        match self {
            EditorPreview::Explosion => "Explosion Preview",
        }
    }
}

impl std::fmt::Debug for Editor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Editor")
            .field("seed", &self.seed)
            .field("re_add_objects_to_game", &self.re_add_objects_to_game)
            .field("show_debug", &self.show_debug)
            .field("show_properties", &self.show_properties)
            .finish()
    }
}

impl Editor {
    pub fn init(&mut self) {
        self.re_add_objects_to_game = true;
        self.previews
            .entry(EditorPreview::Explosion)
            .or_insert_with(PreviewMeta::default);
        if self.seed.is_none() {
            self.seed = Some(69420);
        }
    }

    pub fn update_egui(&mut self, egui_ctx: &egui::Context, game: &mut GameData) {
        // Maybe update time
        self.seed_rand();
        egui::TopBottomPanel::top("State Menu").show(egui_ctx, |ui| {
            self.state_window_ui(ui, game);
        });
        egui::TopBottomPanel::bottom("Messages").show(egui_ctx, |ui| {
            self.message_ui(ui, game);
        });
        if self.show_debug {
            egui::SidePanel::left("Debug").show(egui_ctx, |ui| {
                self.debug_ui(ui, game);
            });
        }
        if self.show_properties {
            // egui::SidePanel::right("Properties").show(egui_ctx, |ui| {
            // self.explosion_builder.editor_ui(ui);
            // });
        }
        self.previews(egui_ctx, game);
    }

    pub fn seed_rand(&self) {
        if let Some(seed) = self.seed {
            rand::srand(seed);
        }
    }

    fn debug_ui(&mut self, ui: &mut egui::Ui, game: &mut GameData) {
        ui.label(format!("FPS {}", game.fps));
        ui.label(format!("TIME {}", game.time));
        ui.label(format!("FT {}", game.frame_time));
    }

    fn message_ui(&mut self, ui: &mut egui::Ui, game: &mut GameData) {
        egui::SidePanel::right("giz_ind").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.toggle_value(&mut self.show_properties, "Properties Panel");
                ui.toggle_value(&mut game.show_gizmos, "Show Gizmos");
                ui.toggle_value(&mut self.show_debug, "Debug Panel");
            });
        });
        let frame = egui::Frame::default();
        egui::CentralPanel::default()
            .frame(frame)
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("{:?}", game.state));
                    if let State::Paused = game.state {
                        ui.label("Press S to Step");
                    }
                });
            });
    }

    fn state_window_ui(&mut self, ui: &mut egui::Ui, game: &mut GameData) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("Editor", |ui| {
                // egui::widgets::C
                let mut keys = self.previews.keys().copied().collect::<Vec<_>>();
                keys.sort();
                keys.iter().for_each(|k| {
                    let meta = self.previews.get_mut(k).unwrap();
                    ui.checkbox(&mut meta.opened, k.get_name());
                });

                ui.separator();
                ui.checkbox(&mut self.show_properties, "Properties Panel");
                ui.checkbox(&mut game.show_gizmos, "Show Gizmos");
                ui.checkbox(&mut self.show_debug, "Debug Panel");
                ui.checkbox(&mut self.re_add_objects_to_game, "Editor Manages Objects");
                ui.separator();
                if ui.button("Exit").clicked() {
                    game.state = State::Exit
                }
            });
            ui.menu_button("Game", |ui| {
                ui.radio_value(&mut game.state, State::Playing, "Play");
                ui.radio_value(&mut game.state, State::Step, "Step");
                ui.radio_value(&mut game.state, State::Paused, "Pause");
                // ui.allocate_space(ui.available_size());
            });
        });
    }

    fn previews(&mut self, ctx: &egui::Context, game: &GameData) {
        self.previews.iter_mut().for_each(|(key, meta)| {
            if meta.opened {
                let preview = &mut meta.preview;
                if preview.is_none() {
                    *preview = Some(key.create_preview())
                }
                if let Some(preview) = preview {
                    egui::Window::new(key.get_name())
                        .min_width(500.)
                        .resizable(true)
                        .open(&mut meta.opened)
                        .show(ctx, |ui| {
                            preview.update_ui(game.frame_time, ui);
                            preview.draw();
                            if game.show_gizmos {
                                preview.draw_gizmos();
                            }
                            preview.draw_ui(ui);
                        });
                }
            }
        });
    }
}

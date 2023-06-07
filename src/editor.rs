use crate::{game_data::GameData, prelude::*};
use cowshmup::{
    particle::{Explosion, ExplosionBuilder},
    CenterPt,
};
use macroquad::rand;

use crate::State;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Editor {
    /// Things we edit may use randomness, we need to reset that every editor frame
    seed: Option<u64>,
    pub re_add_objects_to_game: bool,
    pub show_debug: bool,
    pub show_properties: bool,

    // --- things we edit below here --
    // pub explosion_stages: Vec<ExplosionStage>,
    pub explosion_builder: ExplosionBuilder,
}

impl Editor {
    pub fn init(&mut self) {
        self.re_add_objects_to_game = true;
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
            egui::SidePanel::right("Properties").show(egui_ctx, |ui| {
                self.explosion_builder.editor_ui(ui);
            });
        }
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

    pub fn build_explosion(&mut self, center: CenterPt) -> Option<Explosion> {
        self.explosion_builder.clone().build(center)
    }

    pub fn draw_gizmos_at(&self, center: CenterPt) {
        self.explosion_builder.draw_gizmos_at(center);
    }
}

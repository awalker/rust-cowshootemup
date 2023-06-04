use cowshmup::{
    drawable::Drawable,
    particle::{Explosion, ExplosionBuilder, ExplosionStage},
    CenterPt,
};
use egui_macroquad::egui::{self, Ui};
use macroquad::{prelude::*, rand};
use serde::{Deserialize, Serialize};

use crate::State;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Editor {
    #[serde(skip)]
    pub state: Option<State>,
    pub show_gizmos: bool,
    pub time: f32,
    /// Things we edit may use randomness, we need to reset that every editor frame
    seed: Option<u64>,

    // --- things we edit below here --
    pub explosion_stages: Vec<ExplosionStage>,
}

impl Editor {
    pub fn init(&mut self) {
        self.show_gizmos = true;
        if self.seed.is_none() {
            self.seed = Some(69420);
        }
    }

    pub fn update_egui(&mut self, egui_ctx: &egui::Context, delta_time: f32) {
        // Maybe update time
        self.time += delta_time;
        if let Some(seed) = self.seed {
            rand::srand(seed);
        }
        egui::TopBottomPanel::top("State Menu").show(egui_ctx, |ui| {
            self.state_window_ui(ui);
        });
        egui::SidePanel::right("Explosion").show(egui_ctx, |ui| {
            ui.heading("Explosion");
            for (i, exp) in self.explosion_stages.iter_mut().enumerate() {
                ui.group(|ui| {
                    exp.editor_ui(ui, i);
                });
            }
        });
    }

    fn state_window_ui(&mut self, ui: &mut Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("Game", |ui| {
                if ui.button("Play").clicked() {
                    self.state = Some(State::Playing)
                }
                if ui.button("Step").clicked() {
                    self.state = Some(State::Step)
                }
                if ui.button("Pause").clicked() {
                    self.state = Some(State::Paused)
                }
                if ui.button("Toggle Gizmos").clicked() {
                    self.show_gizmos = !self.show_gizmos;
                }
                ui.separator();
                if ui.button("Exit").clicked() {
                    self.state = Some(State::Exit)
                }
                // ui.allocate_space(ui.available_size());
            });
        });
    }

    pub fn build_explosion(&self, center: CenterPt) -> Option<Explosion> {
        if self.explosion_stages.is_empty() {
            None
        } else {
            let builder = Explosion::begin(center).with_stages(self.explosion_stages.clone());
            Some(builder.build())
        }
    }

    pub fn draw_gizmos(&self) {
        self.explosion_stages.iter().for_each(|es| es.draw_gizmos())
    }
}

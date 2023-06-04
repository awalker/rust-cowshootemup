use cowshmup::{
    drawable::Drawable,
    particle::{Explosion, ExplosionStage},
    CenterPt,
};
use egui_macroquad::egui::{self, Key, Ui};
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
        self.seed_rand();
        self.time += delta_time;
        if egui_ctx.input(|i| i.key_pressed(Key::Escape)) {
            self.state = Some(State::Exit);
        }
        egui::TopBottomPanel::top("State Menu").show(egui_ctx, |ui| {
            self.state_window_ui(ui);
        });
        egui::SidePanel::right("Explosion").show(egui_ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Explosion");
                if ui.small_button("Add Stage").clicked() {
                    if let Some(last) = self.explosion_stages.last() {
                        self.explosion_stages.push(last.clone());
                    } else {
                        self.explosion_stages.push(ExplosionStage::default());
                    }
                }
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, exp) in self.explosion_stages.iter_mut().enumerate() {
                    ui.group(|ui| {
                        exp.editor_ui(ui, i);
                    });
                }
            });
        });
    }

    pub fn seed_rand(&self) {
        if let Some(seed) = self.seed {
            rand::srand(seed);
        }
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

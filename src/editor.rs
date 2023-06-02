use cowshmup::particle::ExplosionStage;
use egui_macroquad::egui::{self, Ui};
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

use crate::State;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Editor {
    #[serde(skip)]
    pub state: Option<State>,
    pub show_gizmos: bool,
    y1: f32,
    y2: f32,
    pub explosion_stages: Vec<ExplosionStage>,
}

impl Editor {
    pub fn update_egui(&mut self, egui_ctx: &egui::Context) {
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
}

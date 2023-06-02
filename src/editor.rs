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

/* for i in 0..30 {
    Group::new(hash!("exp", i), Vec2::new(300., 80.)).ui(ui, |ui| {
        ui.label(Vec2::new(10., 10.), &format!("Item N {}", i));
        ui.label(Vec2::new(260., 40.), "10/10");
        ui.label(Vec2::new(200., 58.), &format!("{} kr", 800));
        if ui.button(Vec2::new(260., 55.), "buy") {
            // data.inventory.push(format!("Item {}", i));
        }
    });
} */

impl Editor {
    pub fn update_egui(&mut self, egui_ctx: &egui::Context) {
        egui::TopBottomPanel::top("State Menu").show(egui_ctx, |ui| {
            self.state_window_ui(ui);
        });
        egui::SidePanel::right("Explosion").show(egui_ctx, |ui| {
            ui.label("Explosion");
            for (i, exp) in self.explosion_stages.iter_mut().enumerate() {
                // Group::new(hash!("exp", i), Vec2::new(w, 40.)).ui(ui, |ui| exp.editor_ui(ui));
                ui.group(|ui| {
                    ui.label(format!("Stage {}", i));
                    exp.editor_ui(ui);
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

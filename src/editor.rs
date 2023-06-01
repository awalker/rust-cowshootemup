use macroquad::{
    hash,
    prelude::*,
    ui::{
        root_ui,
        widgets::{self, Group},
        Ui,
    },
};

use crate::State;

#[derive(Debug, Default, Clone)]
pub struct Editor {
    pub state: Option<State>,
    pub show_gizmos: bool,
    y1: f32,
    y2: f32,
}

impl Editor {
    pub fn draw_editor_update(&mut self) {
        let sw = screen_width();
        let sh = screen_height();
        let w = sw / 3.;
        widgets::Window::new(hash!(), vec2(50., sh - 60.), vec2(sw - 100., 40.))
            .label("State")
            .titlebar(true)
            .ui(&mut root_ui(), |ui| self.state_window_ui(ui));
        widgets::Window::new(hash!(), vec2(sw - w - 5., 20.), vec2(w, sh - 80.))
            .label("Explosion")
            .titlebar(true)
            .ui(&mut root_ui(), |ui| {
                ui.slider(hash!("y1"), "Slider", 0.0..10.0, &mut self.y1);
                ui.slider(hash!("y2"), "Slider", 0.0..10.0, &mut self.y2);
                Group::new(hash!("bb"), vec2(w, 20.)).ui(ui, |ui| {
                    if ui.button(vec2(0., 0.), "Play") {
                        self.state = Some(State::Playing)
                    }
                });
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
            });
    }

    fn state_window_ui(&mut self, ui: &mut Ui) {
        let y = 1.;
        let mut x = 10.;
        let w = 40.;
        if ui.button(vec2(x, y), "Play") {
            self.state = Some(State::Playing)
        }
        x += w;
        if ui.button(vec2(x, y), "Step") {
            self.state = Some(State::Step)
        }
        x += w;
        if ui.button(vec2(x, y), "Pause") {
            self.state = Some(State::Paused)
        }
        x += w;
        if ui.button(vec2(x, y), "Exit") {
            self.state = Some(State::Exit)
        }

        x += w;
        if ui.button(vec2(x, y), "Toggle Gizmos") {
            self.show_gizmos = !self.show_gizmos;
        }
    }
}

use crate::prelude::*;
use cowshmup::{
    particle::{Explosion, ExplosionBuilder},
    retro_camera::RetroCamera,
};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ExplosionPreview {
    builder: ExplosionBuilder,
    filename: String,
    zoom: f32,
    // --- skip serde for all the rest
    #[serde(skip)]
    camera: RetroCamera,
    #[serde(skip)]
    explosion: Option<Explosion>,
    #[serde(skip)]
    time: f32,
    #[serde(skip)]
    max_time: f32,
}

impl Default for ExplosionPreview {
    fn default() -> Self {
        let mut obj = Self {
            zoom: 1.5,
            camera: Default::default(),
            builder: Default::default(),
            explosion: Default::default(),
            time: Default::default(),
            max_time: Default::default(),
            filename: String::from("explosion.yaml"),
        };
        obj.camera.free_scale();
        let _r = obj.load();
        obj
    }
}

/// Create an explosion from a builder in a loop once the explosion is finished.
/// This creates a window(, maybe?)
/// Previews do not operate on `GameData`, they maintain their own data.
impl ExplosionPreview {
    pub fn update(&mut self, delta_time: f32) {
        if let Some(explosion) = &mut self.explosion {
            self.time += delta_time;
            explosion.update(delta_time);
            if self.time >= self.max_time {
                self.explosion = None;
            }
        } else {
            self.explosion = self.builder.clone().build(self.camera.center());
            self.time = 0.;
            self.max_time = self.builder.max_loop_time();
        }
    }

    pub fn load(&mut self) -> anyhow::Result<()> {
        let rdr = BufReader::new(
            File::open(&self.filename)
                .with_context(|| format!("Could not open {}", self.filename))?,
        );
        let obj = serde_yaml::from_reader::<_, ExplosionBuilder>(rdr)
            .with_context(|| format!("could not parse {}", self.filename))?;
        self.builder = obj;
        Ok(())
    }

    pub fn save(&mut self) -> anyhow::Result<()> {
        serde_yaml::to_writer(BufWriter::new(File::create(&self.filename)?), &self.builder)?;
        Ok(())
    }

    pub fn update_ui(&mut self, delta_time: f32, ui: &mut egui::Ui) {
        self.update(delta_time);
        self.camera.reset_canvas_ui(ui);
        self.camera.setup_camera();
    }

    pub fn draw_ui(&mut self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("exp file").show_inside(ui, |ui| {
            ui.horizontal_centered(|ui| {
                ui.text_edit_singleline(&mut self.filename);
                if ui.button("load").clicked() {
                    if let Err(err) = self.load() {
                        error!("Can't load: {:?}", err);
                    }
                }
                if ui.button("save").clicked() {
                    if let Err(err) = self.save() {
                        error!("Can't save: {:?}", err);
                    }
                }
            });
        });
        egui::TopBottomPanel::bottom("exp time").show_inside(ui, |ui| {
            ui.horizontal_centered(|ui| {
                if self.max_time > 0. {
                    let bar = egui::widgets::ProgressBar::new(self.time / self.max_time)
                        .animate(true)
                        .desired_width(200.);
                    ui.add(bar);
                }
                if ui.small_button("Restart").clicked() {
                    self.explosion = None;
                }
            });
        });

        egui::SidePanel::right("exp panel").show_inside(ui, |ui| {
            self.builder.editor_ui(ui);
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let txt = self.camera.render_texture();
            let txt_id =
                egui::TextureId::User(txt.raw_miniquad_texture_handle().gl_internal_id().into());
            let img = egui::widgets::Image::new(txt_id, self.camera.size() * self.zoom);
            ui.add(img);
        });
    }

    pub fn draw(&mut self) {
        clear_background(GREEN);
        if let Some(explosion) = &self.explosion {
            explosion.draw()
        }
    }

    pub fn draw_gizmos(&mut self) {
        self.builder.draw_gizmos_at(self.camera.center());
    }
}

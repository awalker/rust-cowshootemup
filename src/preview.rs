use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use cowshmup::{buildable::Buildable, retro_camera::RetroCamera};

use crate::prelude::*;

/// Create a preview
/// This creates a window(, maybe?)
/// Previews do not operate on `GameData`, they maintain their own data.
pub trait Preview {
    fn update(&mut self, delta_time: f32);
    fn load(&mut self) -> anyhow::Result<()>;
    fn save(&mut self) -> anyhow::Result<()>;
    fn update_ui(&mut self, delta_time: f32, ui: &mut egui::Ui);
    fn draw_ui(&mut self, ui: &mut egui::Ui);
    fn draw_gizmos(&mut self);
    fn draw(&mut self);
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PreviewBuildableData<Editing>
where
    Editing: Buildable,
{
    builder: Editing,
    filename: String,
    zoom: f32,
    // --- skip serde for all the rest
    #[serde(skip)]
    camera: RetroCamera,
    #[serde(skip)]
    game_object: Option<Editing::Byproduct>,
    #[serde(skip)]
    time: f32,
    #[serde(skip)]
    max_time: f32,
}

impl<Editing> Default for PreviewBuildableData<Editing>
where
    Editing: Buildable + Serialize + for<'de> Deserialize<'de>,
{
    fn default() -> Self {
        let mut obj = Self {
            zoom: 1.5,
            camera: Default::default(),
            builder: Default::default(),
            game_object: Default::default(),
            time: Default::default(),
            max_time: Default::default(),
            filename: String::from(Editing::get_default_file_name()),
        };
        obj.camera.free_scale();
        let _r = obj.load();
        obj
    }
}

impl<Editing> Preview for PreviewBuildableData<Editing>
where
    Editing: Buildable + Serialize + for<'de> Deserialize<'de>,
{
    fn update(&mut self, delta_time: f32) {
        if let Some(game_obj) = &mut self.game_object {
            self.time += delta_time;
            game_obj.update(delta_time);
            if self.time >= self.max_time {
                self.game_object = None;
            }
        } else {
            self.game_object = self.builder.clone().build(self.camera.center());
            self.time = 0.;
            self.max_time = self.builder.max_loop_time();
        }
    }

    fn load(&mut self) -> anyhow::Result<()> {
        let rdr = BufReader::new(
            File::open(&self.filename)
                .with_context(|| format!("Could not open {}", self.filename))?,
        );
        let obj = serde_yaml::from_reader::<_, Editing>(rdr)
            .with_context(|| format!("could not parse {}", self.filename))?;
        self.builder = obj;
        Ok(())
    }

    fn save(&mut self) -> anyhow::Result<()> {
        serde_yaml::to_writer(BufWriter::new(File::create(&self.filename)?), &self.builder)?;
        Ok(())
    }

    fn update_ui(&mut self, delta_time: f32, ui: &mut egui::Ui) {
        self.update(delta_time);
        self.camera.reset_canvas_ui(ui);
        self.camera.setup_camera();
    }

    fn draw_ui(&mut self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top(format!("{} file", Editing::get_base_id())).show_inside(
            ui,
            |ui| {
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
            },
        );
        egui::TopBottomPanel::bottom(format!("{} time", Editing::get_base_id())).show_inside(
            ui,
            |ui| {
                ui.horizontal_centered(|ui| {
                    if self.max_time > 0. {
                        let bar = egui::widgets::ProgressBar::new(self.time / self.max_time)
                            .animate(true)
                            .desired_width(200.);
                        ui.add(bar);
                    }
                    if ui.small_button("Restart").clicked() {
                        self.game_object = None;
                    }
                });
            },
        );

        egui::SidePanel::right(format!("{} panel", Editing::get_base_id())).show_inside(ui, |ui| {
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

    fn draw(&mut self) {
        clear_background(GREEN);
        if let Some(game_object) = &self.game_object {
            game_object.draw()
        }
    }

    fn draw_gizmos(&mut self) {
        self.builder.draw_gizmos_at(self.camera.center());
    }
}

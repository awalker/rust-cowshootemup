use egui_macroquad::egui::Ui;

use crate::{drawable::Drawable, updateable::Updateable, CenterPt};

pub trait Buildable: Default + Clone {
    type Byproduct: Updateable + Drawable;

    fn build(self, center: CenterPt) -> Option<Self::Byproduct>;
    fn max_loop_time(&self) -> f32;
    fn draw_gizmos_at(&self, center: CenterPt);
    fn editor_ui(&mut self, ui: &mut Ui);
    fn get_base_id() -> &'static str;
    fn get_default_file_name() -> &'static str;
}

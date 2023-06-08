use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::CenterPt;

pub struct RetroCamera {
    render_target: RenderTarget,
    game_canvas: Rect,
    camera: Camera2D,
    zoom: f32,
    size: Vec2,
    allow_non_int_scaling: bool,
}

impl std::fmt::Debug for RetroCamera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RetroCamera")
            .field("game_canvas", &self.game_canvas)
            .field("zoom", &self.zoom)
            .field("size", &self.size)
            .finish()
    }
}

impl RetroCamera {
    pub fn new(width: f32, height: f32) -> Self {
        let render_target = render_target(width as u32, height as u32);
        render_target.texture.set_filter(FilterMode::Nearest);
        let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., width, height));
        camera.render_target = Some(render_target);
        camera.zoom.y *= -1.;
        let game_canvas = Rect::new(0., 0., screen_width(), screen_height());
        Self {
            render_target,
            game_canvas,
            camera,
            zoom: 1.,
            size: Vec2::new(width, height),
            allow_non_int_scaling: false,
        }
    }

    pub fn free_scale(&mut self) {
        self.allow_non_int_scaling = true;
    }

    /// Figure out the position and zoom factor for the game when given a rect with the available
    /// space
    pub fn calculate_canvas_position_for_int_scale(&mut self) {
        let game_canvas = &mut self.game_canvas;
        let mut zoom = game_canvas.w / self.size.x;
        if !self.allow_non_int_scaling {
            zoom = zoom.floor();
        }
        zoom = zoom.min(game_canvas.h / self.size.y);
        if !self.allow_non_int_scaling {
            zoom = zoom.floor();
        }
        game_canvas.x += (game_canvas.w - (self.size.x * zoom)) / 2.;
        game_canvas.y += (game_canvas.h - (self.size.y * zoom)) / 2.;
        self.zoom = zoom
    }

    pub fn size(&self) -> egui::Vec2 {
        egui::Vec2::new(self.size.x, self.size.y)
    }

    pub fn reset_canvas(&mut self, egui_ctx: &egui::Context) {
        let game_canvas = &mut self.game_canvas;
        let avail = egui_ctx.available_rect();
        game_canvas.y = avail.top();
        game_canvas.h = avail.height();
        game_canvas.w = avail.width();
        game_canvas.x = avail.left();
    }

    pub fn reset_canvas_ui(&mut self, _ui: &mut egui_macroquad::egui::Ui) {}

    pub fn setup_camera(&mut self) {
        self.calculate_canvas_position_for_int_scale();
        push_camera_state();
        set_camera(&self.camera);
    }

    pub fn render_texture(&self) -> macroquad::texture::Texture2D {
        pop_camera_state();
        self.render_target.texture
    }

    pub fn center(&self) -> CenterPt {
        CenterPt::new(self.size.x / 2., self.size.y / 2.)
    }

    pub fn render(&self) {
        pop_camera_state();
        draw_texture_ex(
            self.render_target.texture,
            self.game_canvas.x,
            self.game_canvas.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size.x * self.zoom, self.size.y * self.zoom)),
                ..Default::default()
            },
        );
    }
}

impl Default for RetroCamera {
    fn default() -> Self {
        Self::new(crate::world::GAME_WIDTH, crate::world::GAME_HEIGHT)
    }
}

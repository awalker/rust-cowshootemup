use egui_macroquad::egui::{
    self,
    color_picker::{self, Alpha},
    epaint::{Hsva, HsvaGamma},
    Color32, Rgba, Ui,
};
use macroquad::prelude::Color;
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Default, Debug, Clone, Copy)]
pub struct GameColor(Color);

pub const WHITE: GameColor = GameColor(macroquad::color::WHITE);
pub const BLACK: GameColor = GameColor(macroquad::color::BLACK);
pub const LIGHTGRAY: GameColor = GameColor(macroquad::color::LIGHTGRAY);
pub const GRAY: GameColor = GameColor(macroquad::color::GRAY);
pub const DARKGRAY: GameColor = GameColor(macroquad::color::DARKGRAY);
pub const BLUE: GameColor = GameColor(macroquad::color::BLUE);
pub const RED: GameColor = GameColor(macroquad::color::RED);
pub const GREEN: GameColor = GameColor(macroquad::color::GREEN);
pub const PURPLE: GameColor = GameColor(macroquad::color::PURPLE);
pub const GOLD: GameColor = GameColor(macroquad::color::GOLD);
pub const LIME: GameColor = GameColor(macroquad::color::LIME);
pub const PINK: GameColor = GameColor(macroquad::color::PINK);
pub const BEIGE: GameColor = GameColor(macroquad::color::BEIGE);
pub const BROWN: GameColor = GameColor(macroquad::color::BROWN);
pub const MAROON: GameColor = GameColor(macroquad::color::MAROON);
pub const ORANGE: GameColor = GameColor(macroquad::color::ORANGE);
pub const YELLOW: GameColor = GameColor(macroquad::color::YELLOW);
pub const VIOLET: GameColor = GameColor(macroquad::color::VIOLET);
pub const MAGENTA: GameColor = GameColor(macroquad::color::MAGENTA);
pub const SKYBLUE: GameColor = GameColor(macroquad::color::SKYBLUE);
pub const DARKBLUE: GameColor = GameColor(macroquad::color::DARKBLUE);
pub const DARKBROWN: GameColor = GameColor(macroquad::color::DARKBROWN);
pub const DARKGREEN: GameColor = GameColor(macroquad::color::DARKGREEN);
pub const DARKPURPLE: GameColor = GameColor(macroquad::color::DARKPURPLE);

pub const PALETTE: [GameColor; 24] = [
    BLACK, WHITE, LIGHTGRAY, GRAY, DARKGRAY, SKYBLUE, BLUE, DARKBLUE, PINK, RED, MAROON, MAGENTA,
    LIME, GREEN, DARKGREEN, PURPLE, VIOLET, DARKPURPLE, BEIGE, BROWN, DARKBROWN, GOLD, YELLOW,
    ORANGE,
];

impl GameColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self(Color::new(r, g, b, a))
    }

    pub fn brighten(self, amt: f32) -> Self {
        let hsva: HsvaGamma = self.into();
        let hsva = HsvaGamma {
            v: hsva.v + amt,
            ..hsva
        };
        hsva.into()
    }

    pub fn darken(self, amt: f32) -> Self {
        self.brighten(-amt)
    }

    pub fn editor_ui(mut self, ui: &mut Ui) -> Self {
        let mut srgba = Rgba::from(self);
        ui.horizontal(|ui| {
            if color_picker::color_edit_button_rgba(ui, &mut srgba, Alpha::Opaque).changed() {
                self = srgba.into();
            }
            if ui.small_button("+").clicked() {
                self = self.brighten(0.1)
            }
            if ui.small_button("-").clicked() {
                self = self.darken(0.1)
            }
        });
        self
    }
}

impl From<Color> for GameColor {
    fn from(value: Color) -> Self {
        Self(value)
    }
}

impl From<GameColor> for Color {
    fn from(value: GameColor) -> Self {
        value.0
    }
}

impl From<egui::Rgba> for GameColor {
    fn from(value: egui::Rgba) -> Self {
        let color = Color::new(value.r(), value.g(), value.b(), value.a());
        Self(color)
    }
}

impl From<GameColor> for egui::Rgba {
    fn from(value: GameColor) -> Self {
        let c = value.0;
        egui::Rgba::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)
    }
}

impl From<HsvaGamma> for GameColor {
    fn from(value: HsvaGamma) -> Self {
        let value: egui::Rgba = value.into();
        let color = Color::new(value.r(), value.g(), value.b(), value.a());
        Self(color)
    }
}

impl From<GameColor> for HsvaGamma {
    fn from(value: GameColor) -> Self {
        let c = value.0;
        let rgba = egui::Rgba::from_rgba_unmultiplied(c.r, c.g, c.b, c.a);
        rgba.into()
    }
}

impl From<Color32> for GameColor {
    fn from(value: Color32) -> Self {
        let value: egui::Rgba = value.into();
        let color = Color::new(value.r(), value.g(), value.b(), value.a());
        Self(color)
    }
}

impl From<GameColor> for Color32 {
    fn from(value: GameColor) -> Self {
        let c = value.0;
        let rgba = egui::Rgba::from_rgba_unmultiplied(c.r, c.g, c.b, c.a);
        rgba.into()
    }
}

impl From<Hsva> for GameColor {
    fn from(value: Hsva) -> Self {
        let value: egui::Rgba = value.into();
        let color = Color::new(value.r(), value.g(), value.b(), value.a());
        Self(color)
    }
}

impl From<GameColor> for Hsva {
    fn from(value: GameColor) -> Self {
        let c = value.0;
        let rgba = egui::Rgba::from_rgba_premultiplied(c.r, c.g, c.b, c.a);
        rgba.into()
    }
}

impl Serialize for GameColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("rgba({},{},{},{})", self.0.r, self.0.g, self.0.b, self.0.a);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for GameColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if let Some(s) = s.strip_prefix("rgba(") {
            if let Some(s) = s.strip_suffix(')') {
                let v: Vec<_> = s
                    .splitn(4, ',')
                    .map_while(|v| v.parse::<f32>().ok())
                    .collect();
                if v.len() == 4 {
                    return Ok(Self(Color::new(v[0], v[1], v[2], v[3])));
                }
            }
        }
        Err(serde::de::Error::custom("should be in rgba(...) format"))
    }
}

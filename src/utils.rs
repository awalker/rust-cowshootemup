use egui_macroquad::egui;
use macroquad::prelude::Color;
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Default, Debug, Clone, Copy)]
pub struct GameColor(Color);

impl GameColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self(Color::new(r, g, b, a))
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
        egui::Rgba::from_rgba_premultiplied(c.r, c.g, c.b, c.a)
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

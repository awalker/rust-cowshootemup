pub mod color_format {
    use macroquad::prelude::Color;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("rgba({},{},{},{})", color.r, color.g, color.b, color.a);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
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
                    return Ok(Color::new(v[0], v[1], v[2], v[4]));
                }
            }
        }
        Err(serde::de::Error::custom("should be in rgba(...) format"))
    }
}

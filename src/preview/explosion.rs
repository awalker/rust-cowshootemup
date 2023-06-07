use cowshmup::{particle::ExplosionBuilder, retro_camera::RetroCamera};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExplosionPreview {
    #[serde(skip)]
    camera: RetroCamera,
    builder: ExplosionBuilder,
}

impl ExplosionPreview {
    pub fn new() -> Self {
        Self {
            camera: RetroCamera::default(),
            builder: ExplosionBuilder::default(),
        }
    }
}

use crate::{drawable::Drawable, updateable::Updateable};

pub trait Particle: Drawable + Updateable {}

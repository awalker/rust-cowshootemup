use std::slice::Iter;

// use serde::{Deserialize, Serialize};

use crate::{drawable::Graphic, Rc};

#[derive(Debug, Default /*, Serialize, Deserialize*/)]
pub struct World {
    items: Vec<Graphic>,
}

impl World {
    pub fn add_item(&mut self, d: Graphic) {
        self.items.push(d)
    }

    pub fn iter(&self) -> Iter<'_, Graphic> {
        self.items.iter()
    }
}

pub type RcWorld = Rc<World>;

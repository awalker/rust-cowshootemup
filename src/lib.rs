use serde::{Deserialize, Serialize};

pub mod drawable;
pub mod movable;
pub mod particle;
pub mod state;
pub mod updateable;
pub mod world;

pub type Rc<T> = std::rc::Rc<T>;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct CenterPt(f32, f32);
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct TopLeftPt(f32, f32);
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct BottomRightPt(f32, f32);
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct Size(f32, f32);
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct Velocity(f32, f32);

macro_rules! impl_vec2 {
    ($id: ident) => {
        impl $id {
            pub fn new(x: f32, y: f32) -> Self {
                Self(x, y)
            }

            pub fn get(&self) -> (f32, f32) {
                (self.0, self.1)
            }
        }
        impl From<(f32, f32)> for $id {
            fn from(value: (f32, f32)) -> Self {
                Self(value.0, value.1)
            }
        }
        impl From<$id> for (f32, f32) {
            fn from(value: $id) -> Self {
                (value.0, value.1)
            }
        }
    };
}

impl_vec2!(CenterPt);
impl_vec2!(TopLeftPt);
impl_vec2!(BottomRightPt);
impl_vec2!(Size);

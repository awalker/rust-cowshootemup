use std::ops::{Add, Mul};

use serde::{Deserialize, Serialize};

pub mod alive;
pub mod drawable;
pub mod minmax;
pub mod particle;
pub mod retro_camera;
pub mod timers;
pub mod updateable;
pub mod utils;
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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Accel(f32, f32);

impl Default for Accel {
    fn default() -> Self {
        Self(1., 1.)
    }
}

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

impl Mul<f32> for Velocity {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<Accel> for Velocity {
    type Output = Velocity;

    fn mul(self, rhs: Accel) -> Self::Output {
        Velocity(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Add<Velocity> for CenterPt {
    type Output = CenterPt;

    fn add(self, rhs: Velocity) -> Self::Output {
        CenterPt(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<Size> for CenterPt {
    type Output = CenterPt;

    fn add(self, rhs: Size) -> Self::Output {
        CenterPt(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl_vec2!(CenterPt);
impl_vec2!(TopLeftPt);
impl_vec2!(BottomRightPt);
impl_vec2!(Size);
impl_vec2!(Velocity);
impl_vec2!(Accel);

#[macro_export]
macro_rules! impl_pts {
    (center $id:ident) => {
        impl $crate::drawable::HasCenter for $id {
            fn center(&self) -> $crate::CenterPt {
                self.center
            }
        }

        impl $crate::drawable::UpdateCenter for $id {
            fn update_center(&mut self, v: $crate::CenterPt) {
                self.center = v
            }
        }
    };
    (velocity $id:ident) => {
        impl $crate::drawable::HasVelocity for $id {
            fn velocity(&self) -> $crate::Velocity {
                self.velocity
            }
        }

        impl $crate::drawable::UpdateVelocity for $id {
            fn update_velocity(&mut self, v: $crate::Velocity) {
                self.velocity = v
            }
        }
    };
    (accel $id:ident) => {
        impl $crate::drawable::HasAccel for $id {
            fn accel(&self) -> $crate::Accel {
                self.accel
            }
        }

        impl $crate::drawable::UpdateAccel for $id {
            fn update_accel(&mut self, v: $crate::Accel) {
                self.accel = v
            }
        }
    };
}

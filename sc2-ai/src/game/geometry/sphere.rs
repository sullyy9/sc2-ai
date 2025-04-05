use bevy::ecs::component::Component;

use super::point::Vec3;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub const fn from_center(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub const fn center(&self) -> &Vec3 {
        &self.center
    }

    pub const fn radius(&self) -> f32 {
        self.radius
    }
}

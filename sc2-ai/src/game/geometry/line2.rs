use bevy::ecs::component::Component;

use super::vec2::Vec2;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub struct Line2(pub Vec2, pub Vec2);

impl Line2 {
    pub const fn new(p0: Vec2, p1: Vec2) -> Self {
        Self(p0, p1)
    }
}

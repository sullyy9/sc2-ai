use bevy::ecs::component::Component;

use super::vec2::Vec2;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub struct Rect(Vec2, Vec2);

impl Rect {
    pub const fn from_corners(p0: Vec2, p1: Vec2) -> Self {
        let min = Vec2::new(p0.0.x.min(p1.0.x), p0.0.y.min(p1.0.y));
        let max = Vec2::new(p0.0.x.max(p1.0.x), p0.0.y.max(p1.0.y));
        Self(min, max)
    }

    pub const fn from_center(centre: Vec2, size: Vec2) -> Self {
        debug_assert!(size.0.x >= 0.0);
        debug_assert!(size.0.y >= 0.0);

        let size_x = size.0.x / 2.0;
        let size_y = size.0.y / 2.0;

        let min = Vec2::new(centre.0.x - size_x, centre.0.y - size_y);
        let max = Vec2::new(centre.0.x + size_x, centre.0.y + size_y);

        Self(min, max)
    }

    pub const fn min(&self) -> Vec2 {
        self.0
    }

    pub const fn max(&self) -> Vec2 {
        self.1
    }

    pub const fn center(&self) -> Vec2 {
        let min = self.0;
        let max = self.1;

        let x = (min.0.x + max.0.x) / 2.0;
        let y = (min.0.y + max.0.y) / 2.0;

        Vec2::new(x, y)
    }

    pub const fn overlaps(&self, other: &Self) -> bool {
        (self.max().0.x > other.min().0.x)
            && (self.min().0.x < other.max().0.x)
            && (self.max().0.y > other.min().0.y)
            && (self.min().0.y < other.max().0.y)
    }

    pub fn min_distance(&self, other: &Self) -> f32 {
        f32::sqrt(self.min_distance_squared(other))
    }

    pub fn min_distance_squared(&self, other: &Self) -> f32 {
        let u = self.min() - other.max();
        let u = Vec2::new(f32::max(0.0, u.x), f32::max(0.0, u.y));

        let v = other.min() - self.max();
        let v = Vec2::new(f32::max(0.0, v.x), f32::max(0.0, v.y));

        u.euclidean_norm_squared() + v.euclidean_norm_squared()
    }
}

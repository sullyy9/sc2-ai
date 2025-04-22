use bevy::ecs::component::Component;

use super::vec2::Vec2;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub struct Rect(Vec2, Vec2);

impl Rect {
    pub const fn from_corners(p0: Vec2, p1: Vec2) -> Self {
        let min = Vec2::new(p0.x.min(p1.x), p0.y.min(p1.y));
        let max = Vec2::new(p0.x.max(p1.x), p0.y.max(p1.y));
        Self(min, max)
    }

    pub const fn from_center(centre: Vec2, size: Vec2) -> Self {
        debug_assert!(size.x >= 0.0);
        debug_assert!(size.y >= 0.0);

        let size_x = size.x / 2.0;
        let size_y = size.y / 2.0;

        let min = Vec2::new(centre.x - size_x, centre.y - size_y);
        let max = Vec2::new(centre.x + size_x, centre.y + size_y);

        Self(min, max)
    }

    /// Return the smallest [`Rect`] which contains a given set of points.
    ///
    /// Returns [`None`] if points is empty.
    pub fn bounding_points(mut points: impl Iterator<Item = Vec2>) -> Option<Self> {
        let first = points.next()?;

        let (min, max) = points.fold((first, first), |(prev_min, prev_max), point| {
            (point.min(prev_min), point.max(prev_max))
        });

        Some(Self(min, max))
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

        let x = (min.x + max.x) / 2.0;
        let y = (min.y + max.y) / 2.0;

        Vec2::new(x, y)
    }

    pub fn size(self) -> Vec2 {
        self.1 - self.0
    }

    pub const fn overlaps(&self, other: &Self) -> bool {
        (self.max().x > other.min().x)
            && (self.min().x < other.max().x)
            && (self.max().y > other.min().y)
            && (self.min().y < other.max().y)
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

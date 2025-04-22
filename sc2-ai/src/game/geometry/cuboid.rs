use bevy::ecs::component::Component;

use super::vec3::Vec3;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub struct Cuboid(Vec3, Vec3);

impl Cuboid {
    pub const fn from_corners(p0: Vec3, p1: Vec3) -> Self {
        let min = Vec3::new(p0.x.min(p1.x), p0.y.min(p1.y), p0.z.min(p1.z));
        let max = Vec3::new(p0.x.max(p1.x), p0.y.max(p1.y), p0.z.max(p1.z));
        Self(min, max)
    }

    pub const fn from_center(centre: Vec3, size: Vec3) -> Self {
        debug_assert!(size.x >= 0.0);
        debug_assert!(size.y >= 0.0);
        debug_assert!(size.z >= 0.0);

        let size_x = size.x / 2.0;
        let size_y = size.y / 2.0;
        let size_z = size.z / 2.0;

        let min = Vec3::new(centre.x - size_x, centre.y - size_y, centre.z - size_z);
        let max = Vec3::new(centre.x + size_x, centre.y + size_y, centre.z + size_z);

        Self(min, max)
    }

    /// Create a new cuboid from the centre of its bottom face and a size.
    pub const fn from_base_center(centre: Vec3, size: Vec3) -> Self {
        debug_assert!(size.x >= 0.0);
        debug_assert!(size.y >= 0.0);
        debug_assert!(size.z >= 0.0);

        let size_x = size.x / 2.0;
        let size_y = size.y / 2.0;

        let min = Vec3::new(centre.x - size_x, centre.y - size_y, centre.z);
        let max = Vec3::new(centre.x + size_x, centre.y + size_y, centre.z + size.z);

        Self(min, max)
    }

    /// Return the smallest [`Cuboid`] which contains a given set of points.
    ///
    /// Returns [`None`] if points is empty.
    pub fn bounding_points(mut points: impl Iterator<Item = Vec3>) -> Option<Self> {
        let first = points.next()?;

        let (min, max) = points.fold((first, first), |(prev_min, prev_max), point| {
            (point.min(prev_min), point.max(prev_max))
        });

        Some(Self(min, max))
    }

    pub const fn min(&self) -> &Vec3 {
        &self.0
    }

    pub const fn max(&self) -> &Vec3 {
        &self.1
    }

    pub const fn center(&self) -> Vec3 {
        let min = self.0;
        let max = self.1;

        let x = (min.x + max.x) / 2.0;
        let y = (min.y + max.y) / 2.0;
        let z = (min.z + max.z) / 2.0;

        Vec3::new(x, y, z)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        (self.max().x > other.min().x)
            && (self.min().x < other.max().x)
            && (self.max().y > other.min().y)
            && (self.min().y < other.max().y)
            && (self.max().z > other.min().z)
            && (self.min().z < other.max().z)
    }
}

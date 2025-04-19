use bevy::ecs::component::Component;

use super::vec3::Vec3;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub struct Cuboid(Vec3, Vec3);

impl Cuboid {
    pub const fn from_corners(p0: Vec3, p1: Vec3) -> Self {
        let min = Vec3::new(p0.0.x.min(p1.0.x), p0.0.y.min(p1.0.y), p0.0.z.min(p1.0.z));
        let max = Vec3::new(p0.0.x.max(p1.0.x), p0.0.y.max(p1.0.y), p0.0.z.max(p1.0.z));
        Self(min, max)
    }

    pub const fn from_center(centre: Vec3, size: Vec3) -> Self {
        debug_assert!(size.0.x >= 0.0);
        debug_assert!(size.0.y >= 0.0);
        debug_assert!(size.0.z >= 0.0);

        let size_x = size.0.x / 2.0;
        let size_y = size.0.y / 2.0;
        let size_z = size.0.z / 2.0;

        let min = Vec3::new(
            centre.0.x - size_x,
            centre.0.y - size_y,
            centre.0.z - size_z,
        );
        let max = Vec3::new(
            centre.0.x + size_x,
            centre.0.y + size_y,
            centre.0.z + size_z,
        );

        Self(min, max)
    }

    /// Create a new cuboid from the centre of its bottom face and a size.
    pub const fn from_base_center(centre: Vec3, size: Vec3) -> Self {
        debug_assert!(size.0.x >= 0.0);
        debug_assert!(size.0.y >= 0.0);
        debug_assert!(size.0.z >= 0.0);

        let size_x = size.0.x / 2.0;
        let size_y = size.0.y / 2.0;

        let min = Vec3::new(centre.0.x - size_x, centre.0.y - size_y, centre.0.z);
        let max = Vec3::new(
            centre.0.x + size_x,
            centre.0.y + size_y,
            centre.0.z + size.0.z,
        );

        Self(min, max)
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

        let x = (min.0.x + max.0.x) / 2.0;
        let y = (min.0.y + max.0.y) / 2.0;
        let z = (min.0.z + max.0.z) / 2.0;

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

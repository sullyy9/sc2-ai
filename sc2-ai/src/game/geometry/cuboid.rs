use bevy::ecs::component::Component;

use super::point::Vec3;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub struct Cuboid(Vec3, Vec3);

impl Cuboid {
    pub const fn from_corners(p0: Vec3, p1: Vec3) -> Self {
        let min = Vec3::new_3d(p0.0.x.min(p1.0.x), p0.0.y.min(p1.0.y), p0.0.z.min(p1.0.z));
        let max = Vec3::new_3d(p0.0.x.max(p1.0.x), p0.0.y.max(p1.0.y), p0.0.z.max(p1.0.z));
        Self(min, max)
    }

    pub const fn from_center(centre: Vec3, size: Vec3) -> Self {
        debug_assert!(size.0.x >= 0.0);
        debug_assert!(size.0.y >= 0.0);
        debug_assert!(size.0.z >= 0.0);

        let size_x = size.0.x / 2.0;
        let size_y = size.0.y / 2.0;
        let size_z = size.0.z / 2.0;

        let min = Vec3::new_3d(centre.0.x - size_x, centre.0.y - size_y, centre.0.z - size_z);
        let max = Vec3::new_3d(centre.0.x + size_x, centre.0.y + size_y, centre.0.z + size_z);

        Self(min, max)
    }

    pub const fn min(&self) -> &Vec3 {
        &self.0
    }

    pub const fn max(&self) -> &Vec3 {
        &self.1
    }
}

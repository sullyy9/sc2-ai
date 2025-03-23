use bevy::ecs::component::Component;

use super::point::Vec3;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub struct Rect(Vec3, Vec3);

impl Rect {
    pub fn from_corners(p0: Vec3, p1: Vec3) -> Self {
        let min = Vec3::new_3d(p0.x.min(p1.x), p0.y.min(p1.y), p0.z.min(p1.z));
        let max = Vec3::new_3d(p0.x.max(p1.x), p0.y.max(p1.y), p0.z.max(p1.z));
        Self(min, max)
    }

    pub fn from_center(centre: Vec3, size: Vec3) -> Self {
        debug_assert!(size.x >= 0.0);
        debug_assert!(size.y >= 0.0);
        debug_assert!(size.z >= 0.0);

        let size_x = size.x / 2.0;
        let size_y = size.y / 2.0;
        let size_z = size.z / 2.0;

        let min = Vec3::new_3d(centre.x - size_x, centre.y - size_y, centre.z - size_z);
        let max = Vec3::new_3d(centre.x + size_x, centre.y + size_y, centre.z + size_z);

        Self(min, max)
    }

    pub fn min(&self) -> &Vec3 {
        &self.0
    }

    pub fn max(&self) -> &Vec3 {
        &self.1
    }
}

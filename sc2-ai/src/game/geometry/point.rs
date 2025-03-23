use bevy::ecs::component::Component;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new_3d(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn new_2d(x: f32, y: f32) -> Self {
        Self { x, y, z: 0.0 }
    }
}

impl From<sc2_proto::common::Point> for Vec3 {
    fn from(value: sc2_proto::common::Point) -> Self {
        Self {
            x: value.x(),
            y: value.y(),
            z: value.z(),
        }
    }
}

impl From<Vec3> for sc2_proto::common::Point {
    fn from(value: Vec3) -> Self {
        let mut point = sc2_proto::common::Point::new();
        point.set_x(value.x);
        point.set_y(value.y);
        point.set_z(value.z);
        point
    }
}

impl From<sc2_proto::common::Point2D> for Vec3 {
    fn from(value: sc2_proto::common::Point2D) -> Self {
        Self {
            x: value.x(),
            y: value.y(),
            z: 0.0,
        }
    }
}

impl From<Vec3> for sc2_proto::common::Point2D {
    fn from(value: Vec3) -> Self {
        let mut point = sc2_proto::common::Point2D::new();
        point.set_x(value.x);
        point.set_y(value.y);
        point
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
            z: self.z.add(rhs.z),
        }
    }
}

impl std::ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Vec3 {
        self.add(*rhs)
    }
}

impl std::ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Vec3 {
        (*self).add(*rhs)
    }
}

impl std::ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        (*self).add(rhs)
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
        self.z.add_assign(rhs.z);
    }
}

impl std::ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Self) {
        self.add_assign(*rhs)
    }
}

use bevy::ecs::component::Component;
use duplicate::duplicate_item;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub struct Vec3(bevy::math::Vec3);

impl Vec3 {
    pub fn new_3d(x: f32, y: f32, z: f32) -> Self {
        Self(bevy::math::Vec3::new(x, y, z))
    }

    pub fn new_2d(x: f32, y: f32) -> Self {
        Self(bevy::math::Vec3::new(x, y, 0.0))
    }
}

impl From<sc2_proto::common::Point> for Vec3 {
    fn from(value: sc2_proto::common::Point) -> Self {
        Self(bevy::math::Vec3::new(value.x(), value.y(), value.z()))
    }
}

impl From<Vec3> for sc2_proto::common::Point {
    fn from(value: Vec3) -> Self {
        let mut point = sc2_proto::common::Point::new();
        point.set_x(value.0.x);
        point.set_y(value.0.y);
        point.set_z(value.0.z);
        point
    }
}

impl From<sc2_proto::common::Point2D> for Vec3 {
    fn from(value: sc2_proto::common::Point2D) -> Self {
        Self(bevy::math::Vec3::new(value.x(), value.y(), 0.0))
    }
}

impl From<Vec3> for sc2_proto::common::Point2D {
    fn from(value: Vec3) -> Self {
        let mut point = sc2_proto::common::Point2D::new();
        point.set_x(value.0.x);
        point.set_y(value.0.y);
        point
    }
}

impl std::ops::Deref for Vec3 {
    type Target = bevy::math::Vec3;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Vec3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[duplicate_item(
    Lhs       Rhs;
    [ Vec3  ] [ Vec3  ];
    [ Vec3  ] [ &Vec3 ];
    [ &Vec3 ] [ Vec3  ];
    [ &Vec3 ] [ &Vec3 ];
)]
impl std::ops::Add<Rhs> for Lhs {
    type Output = Vec3;

    fn add(self, rhs: Rhs) -> Vec3 {
        Vec3(self.0.add(rhs.0))
    }
}

#[duplicate_item(
    Lhs       Rhs;
    [ Vec3  ] [ Vec3  ];
    [ Vec3  ] [ &Vec3 ];
)]
impl std::ops::AddAssign<Rhs> for Lhs {
    fn add_assign(&mut self, rhs: Rhs) {
        self.0.add_assign(rhs.0);
    }
}

#[duplicate_item(
    Lhs       Rhs;
    [ Vec3  ] [ Vec3  ];
    [ Vec3  ] [ &Vec3 ];
    [ &Vec3 ] [ Vec3  ];
    [ &Vec3 ] [ &Vec3 ];
)]
impl std::ops::Sub<Rhs> for Lhs {
    type Output = Vec3;

    fn sub(self, rhs: Rhs) -> Vec3 {
        Vec3(self.0.sub(rhs.0))
    }
}

#[duplicate_item(
    Lhs       Rhs;
    [ Vec3  ] [ Vec3  ];
    [ Vec3  ] [ &Vec3 ];
)]
impl std::ops::SubAssign<Rhs> for Lhs {
    fn sub_assign(&mut self, rhs: Rhs) {
        self.0.sub_assign(rhs.0);
    }
}

impl Vec3 {
    pub fn length(&self) -> f32 {
        self.0.length()
    }

    pub fn distance(&self, other: Self) -> f32 {
        self.0.distance(other.0)
    }
}

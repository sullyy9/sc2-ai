use bevy::{ecs::component::Component, math::NormedVectorSpace};
use duplicate::duplicate_item;

use super::vec3::Vec3;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub struct Vec2(pub(super) bevy::math::Vec2);

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self(bevy::math::Vec2::new(x, y))
    }

    pub fn with_x(mut self, x: f32) -> Self {
        self.0.x = x;
        self
    }

    pub fn with_y(mut self, y: f32) -> Self {
        self.0.y = y;
        self
    }

    pub const fn with_z(self, z: f32) -> Vec3 {
        Vec3(bevy::math::Vec3::new(self.0.x, self.0.y, z))
    }

    pub fn midpoint<I>(points: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        let (count, point) = points
            .into_iter()
            .fold((0, Vec2::default()), |(i, acc), point| (i + 1, acc + point));

        point / count as f32
    }

    pub fn euclidean_norm(&self) -> f32 {
        self.0.norm()
    }

    pub fn euclidean_norm_squared(&self) -> f32 {
        self.0.norm_squared()
    }
}

impl From<sc2_proto::common::Point2D> for Vec2 {
    fn from(value: sc2_proto::common::Point2D) -> Self {
        Self(bevy::math::Vec2::new(value.x(), value.y()))
    }
}

impl From<Vec2> for sc2_proto::common::Point2D {
    fn from(value: Vec2) -> Self {
        let mut point = sc2_proto::common::Point2D::new();
        point.set_x(value.0.x);
        point.set_y(value.0.y);
        point
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

impl std::ops::Deref for Vec2 {
    type Target = bevy::math::Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Vec2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[duplicate_item(
    Lhs       Rhs;
    [ Vec2  ] [ Vec2  ];
    [ Vec2  ] [ &Vec2 ];
    [ &Vec2 ] [ Vec2  ];
    [ &Vec2 ] [ &Vec2 ];
)]
impl std::ops::Add<Rhs> for Lhs {
    type Output = Vec2;

    fn add(self, rhs: Rhs) -> Vec2 {
        Vec2(self.0.add(rhs.0))
    }
}

#[duplicate_item(
    Lhs       Rhs;
    [ Vec2  ] [ Vec2  ];
    [ Vec2  ] [ &Vec2 ];
)]
impl std::ops::AddAssign<Rhs> for Lhs {
    fn add_assign(&mut self, rhs: Rhs) {
        self.0.add_assign(rhs.0);
    }
}

#[duplicate_item(
    Lhs       Rhs;
    [ Vec2  ] [ Vec2  ];
    [ Vec2  ] [ &Vec2 ];
    [ &Vec2 ] [ Vec2  ];
    [ &Vec2 ] [ &Vec2 ];
)]
impl std::ops::Sub<Rhs> for Lhs {
    type Output = Vec2;

    fn sub(self, rhs: Rhs) -> Vec2 {
        Vec2(self.0.sub(rhs.0))
    }
}

#[duplicate_item(
    Lhs       Rhs;
    [ Vec2  ] [ Vec2  ];
    [ Vec2  ] [ &Vec2 ];
)]
impl std::ops::SubAssign<Rhs> for Lhs {
    fn sub_assign(&mut self, rhs: Rhs) {
        self.0.sub_assign(rhs.0);
    }
}

impl Vec2 {
    pub fn length(&self) -> f32 {
        self.0.length()
    }

    pub fn distance(&self, other: Self) -> f32 {
        self.0.distance(other.0)
    }
}

#[duplicate_item(
    Lhs       Rhs;
    [ Vec2  ] [ f32  ];
    [ Vec2  ] [ &f32 ];
    [ &Vec2 ] [ f32  ];
    [ &Vec2 ] [ &f32 ];
)]
impl std::ops::Div<Rhs> for Lhs {
    type Output = Vec2;

    fn div(self, rhs: Rhs) -> Vec2 {
        Vec2(self.0.div(rhs))
    }
}

#[duplicate_item(
    Lhs       Rhs;
    [ Vec2  ] [ f32  ];
    [ Vec2  ] [ &f32 ];
)]
impl std::ops::DivAssign<Rhs> for Lhs {
    fn div_assign(&mut self, rhs: Rhs) {
        self.0.div_assign(rhs);
    }
}

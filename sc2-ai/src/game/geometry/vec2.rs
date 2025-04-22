use bevy::ecs::component::Component;
use duplicate::duplicate_item;

use super::vec3::Vec3;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn with_x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }

    pub fn with_y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }

    pub const fn with_z(self, z: f32) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z,
        }
    }

    pub const fn min(self, other: Self) -> Self {
        Self {
            x: f32::min(self.x, other.x),
            y: f32::min(self.y, other.y),
        }
    }

    pub const fn max(self, other: Self) -> Self {
        Self {
            x: f32::max(self.x, other.x),
            y: f32::max(self.y, other.y),
        }
    }

    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
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

    pub fn euclidean_norm(self) -> f32 {
        f32::sqrt(self.euclidean_norm_squared())
    }

    pub const fn euclidean_norm_squared(self) -> f32 {
        (self.x * self.x) + (self.y * self.y)
    }

    pub fn length(self) -> f32 {
        self.euclidean_norm()
    }

    pub fn length_squared(self) -> f32 {
        self.euclidean_norm_squared()
    }

    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }
}

impl From<sc2_proto::common::Point2D> for Vec2 {
    fn from(value: sc2_proto::common::Point2D) -> Self {
        Self {
            x: value.x(),
            y: value.y(),
        }
    }
}

impl From<Vec2> for sc2_proto::common::Point2D {
    fn from(value: Vec2) -> Self {
        let mut point = sc2_proto::common::Point2D::new();
        point.set_x(value.x);
        point.set_y(value.y);
        point
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
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
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[duplicate_item(
    Lhs       Rhs;
    [ Vec2  ] [ Vec2  ];
    [ Vec2  ] [ &Vec2 ];
)]
impl std::ops::AddAssign<Rhs> for Lhs {
    fn add_assign(&mut self, rhs: Rhs) {
        self.x += rhs.x;
        self.y += rhs.y;
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
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[duplicate_item(
    Lhs       Rhs;
    [ Vec2  ] [ Vec2  ];
    [ Vec2  ] [ &Vec2 ];
)]
impl std::ops::SubAssign<Rhs> for Lhs {
    fn sub_assign(&mut self, rhs: Rhs) {
        self.x -= rhs.x;
        self.y -= rhs.y;
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
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

#[duplicate_item(
    Lhs       Rhs;
    [ Vec2  ] [ f32  ];
    [ Vec2  ] [ &f32 ];
)]
impl std::ops::DivAssign<Rhs> for Lhs {
    fn div_assign(&mut self, rhs: Rhs) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

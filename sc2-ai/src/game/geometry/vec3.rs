use bevy::ecs::component::Component;
use duplicate::duplicate_item;

use super::vec2::Vec2;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn from_vec2(vec: Vec2, z: f32) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z,
        }
    }

    pub fn with_x(mut self, x: f32) -> Self {
        self.x = x;
        self
    }

    pub fn with_y(mut self, y: f32) -> Self {
        self.y = y;
        self
    }

    pub fn with_z(mut self, z: f32) -> Self {
        self.z = z;
        self
    }

    pub const fn without_z(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    pub const fn min(self, other: Self) -> Self {
        Self {
            x: f32::min(self.x, other.x),
            y: f32::min(self.y, other.y),
            z: f32::min(self.z, other.z),
        }
    }

    pub const fn max(self, other: Self) -> Self {
        Self {
            x: f32::max(self.x, other.x),
            y: f32::max(self.y, other.y),
            z: f32::max(self.z, other.z),
        }
    }

    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }

    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }

    pub fn midpoint<I>(points: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        let (count, point) = points
            .into_iter()
            .fold((0, Vec3::default()), |(i, acc), point| (i + 1, acc + point));

        point / count as f32
    }

    pub fn euclidean_norm(self) -> f32 {
        f32::sqrt(self.euclidean_norm_squared())
    }

    pub const fn euclidean_norm_squared(self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
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

impl From<(f32, f32, f32)> for Vec3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self::new(x, y, z)
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
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[duplicate_item(
    Lhs       Rhs;
    [ Vec3  ] [ Vec3  ];
    [ Vec3  ] [ &Vec3 ];
)]
impl std::ops::AddAssign<Rhs> for Lhs {
    fn add_assign(&mut self, rhs: Rhs) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
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
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[duplicate_item(
    Lhs       Rhs;
    [ Vec3  ] [ Vec3  ];
    [ Vec3  ] [ &Vec3 ];
)]
impl std::ops::SubAssign<Rhs> for Lhs {
    fn sub_assign(&mut self, rhs: Rhs) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

#[duplicate_item(
    Lhs       Rhs;
    [ Vec3  ] [ f32  ];
    [ Vec3  ] [ &f32 ];
    [ &Vec3 ] [ f32  ];
    [ &Vec3 ] [ &f32 ];
)]
impl std::ops::Div<Rhs> for Lhs {
    type Output = Vec3;

    fn div(self, rhs: Rhs) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[duplicate_item(
    Lhs       Rhs;
    [ Vec3  ] [ f32  ];
    [ Vec3  ] [ &f32 ];
)]
impl std::ops::DivAssign<Rhs> for Lhs {
    fn div_assign(&mut self, rhs: Rhs) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

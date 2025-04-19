use protobuf::MessageField;

use crate::game::geometry::{Cuboid, Rect, Vec2};

use super::{color::Color, draw::SURFACE_Z_OFFSET};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawBox {
    cuboid: Cuboid,
    color: Color,
}

impl DrawBox {
    pub fn new(cuboid: Cuboid) -> Self {
        Self {
            cuboid,
            color: Color::default(),
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl From<DrawBox> for sc2_proto::debug::DebugBox {
    fn from(value: DrawBox) -> Self {
        let mut boxx = sc2_proto::debug::DebugBox::new();
        boxx.min = MessageField(Some(Box::new((*value.cuboid.min()).into())));
        boxx.max = MessageField(Some(Box::new((*value.cuboid.max()).into())));
        boxx.color = MessageField(Some(Box::new(value.color.into())));

        boxx
    }
}

impl From<sc2_proto::debug::DebugBox> for DrawBox {
    fn from(value: sc2_proto::debug::DebugBox) -> Self {
        let sc2_proto::debug::DebugBox {
            min: MessageField(Some(min)),
            max: MessageField(Some(max)),
            color: MessageField(color),
            ..
        } = value
        else {
            panic!("Unexpected None value in sc2_proto::debug::DebugLine");
        };

        Self {
            cuboid: Cuboid::from_corners((*min).into(), (*max).into()),
            color: color.map(|c| (*c).into()).unwrap_or_default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawSurfaceBox {
    base: Rect,
    height: f32,
    color: Color,
}

impl DrawSurfaceBox {
    pub fn new(base: Rect, height: f32) -> Self {
        Self {
            base,
            height,
            color: Color::default(),
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn map_to_surface(self, map: impl std::ops::Index<Vec2, Output = f32>) -> DrawBox {
        let z_height = *map.index(self.base.center()) + SURFACE_Z_OFFSET;

        DrawBox {
            cuboid: Cuboid::from_corners(
                self.base.min().with_z(z_height),
                self.base.max().with_z(z_height + self.height),
            ),
            color: self.color,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawSurfaceRect {
    rect: Rect,
    color: Color,
}

impl DrawSurfaceRect {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            color: Color::default(),
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn map_to_surface(self, map: impl std::ops::Index<Vec2, Output = f32>) -> DrawBox {
        let height = *map.index(self.rect.center()) + SURFACE_Z_OFFSET;

        DrawBox {
            cuboid: Cuboid::from_corners(
                self.rect.min().with_z(height),
                self.rect.max().with_z(height),
            ),
            color: self.color,
        }
    }
}

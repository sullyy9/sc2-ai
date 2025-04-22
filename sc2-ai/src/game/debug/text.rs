use protobuf::MessageField;

use crate::game::geometry::{Vec2, Vec3};

use super::{color::Color, draw::SURFACE_Z_OFFSET};

#[derive(Clone, Debug, PartialEq)]
pub struct DrawText {
    text: String,
    position: Vec3,
    color: Color,
    size: u32,
}

impl DrawText {
    pub const DEFAULT_TEXT_SIZE: u32 = 12;

    pub fn new(text: String, position: Vec3) -> Self {
        Self {
            text,
            position,
            color: Color::default(),
            size: Self::DEFAULT_TEXT_SIZE,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }
}

impl From<DrawText> for sc2_proto::debug::DebugText {
    fn from(value: DrawText) -> Self {
        let mut text = sc2_proto::debug::DebugText::new();
        text.set_text(value.text);
        text.world_pos = MessageField(Some(Box::new(value.position.into())));
        text.color = MessageField(Some(Box::new(value.color.into())));
        text.size = Some(value.size);

        text
    }
}

impl From<sc2_proto::debug::DebugText> for DrawText {
    fn from(value: sc2_proto::debug::DebugText) -> Self {
        let sc2_proto::debug::DebugText {
            text: Some(text),
            world_pos: MessageField(Some(position)),
            color: MessageField(color),
            size,
            ..
        } = value
        else {
            panic!("Unexpected None value in sc2_proto::debug::DebugLine");
        };

        Self {
            text,
            position: (*position).into(),
            color: color.map(|c| (*c).into()).unwrap_or_default(),
            size: size.unwrap_or(Self::DEFAULT_TEXT_SIZE),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DrawSurfaceText {
    text: String,
    position: Vec2,
    color: Color,
    size: u32,
}

impl DrawSurfaceText {
    pub const DEFAULT_TEXT_SIZE: u32 = DrawText::DEFAULT_TEXT_SIZE;

    pub fn new(text: String, position: Vec2) -> Self {
        Self {
            text,
            position,
            color: Color::default(),
            size: Self::DEFAULT_TEXT_SIZE,
        }
    }

    pub const fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub const fn with_size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }

    pub fn map_to_surface(self, map: impl std::ops::Index<Vec2, Output = f32>) -> DrawText {
        let z_height = *map.index(self.position) + SURFACE_Z_OFFSET;

        DrawText {
            text: self.text,
            position: self.position.with_z(z_height),
            color: self.color,
            size: self.size,
        }
    }
}

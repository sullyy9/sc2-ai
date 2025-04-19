use protobuf::MessageField;

use crate::game::geometry::{Line2, Line3, Vec2};

use super::{color::Color, draw::SURFACE_Z_OFFSET};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawLine {
    line: Line3,
    color: Color,
}

impl DrawLine {
    pub fn new(line: Line3) -> Self {
        Self {
            line,
            color: Color::default(),
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl From<DrawLine> for sc2_proto::debug::DebugLine {
    fn from(value: DrawLine) -> Self {
        let mut line = sc2_proto::debug::DebugLine::new();
        line.line = MessageField(Some(Box::new(value.line.into())));
        line.color = MessageField(Some(Box::new(value.color.into())));

        line
    }
}

impl From<sc2_proto::debug::DebugLine> for DrawLine {
    fn from(value: sc2_proto::debug::DebugLine) -> Self {
        let sc2_proto::debug::DebugLine {
            line: MessageField(Some(line)),
            color: MessageField(color),
            ..
        } = value
        else {
            panic!("Unexpected None value in sc2_proto::debug::DebugLine");
        };

        Self {
            line: (*line).into(),
            color: color.map(|c| (*c).into()).unwrap_or_default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawSurfaceLine {
    line: Line2,
    color: Color,
}

impl DrawSurfaceLine {
    pub fn new(line: Line2) -> Self {
        Self {
            line,
            color: Color::default(),
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn map_to_surface(self, map: impl std::ops::Index<Vec2, Output = f32>) -> DrawLine {
        let height_p0 = *map.index(self.line.0) + SURFACE_Z_OFFSET;
        let height_p1 = *map.index(self.line.1) + SURFACE_Z_OFFSET;

        DrawLine {
            line: Line3::new(self.line.0.with_z(height_p0), self.line.1.with_z(height_p1)),
            color: self.color,
        }
    }
}

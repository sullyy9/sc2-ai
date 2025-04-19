use bevy::ecs::component::Component;
use protobuf::MessageField;

use super::vec3::Vec3;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub struct Line3(Vec3, Vec3);

impl Line3 {
    pub const fn new(p0: Vec3, p1: Vec3) -> Self {
        Self(p0, p1)
    }
}

impl From<sc2_proto::debug::Line> for Line3 {
    fn from(value: sc2_proto::debug::Line) -> Self {
        let sc2_proto::debug::Line {
            p0: MessageField(Some(p0)),
            p1: MessageField(Some(p1)),
            ..
        } = value
        else {
            panic!("Unexpected None value in sc2_proto::debug::Line");
        };

        Self((*p0).into(), (*p1).into())
    }
}

impl From<Line3> for sc2_proto::debug::Line {
    fn from(value: Line3) -> Self {
        let mut line = sc2_proto::debug::Line::new();
        line.p0 = MessageField(Some(Box::new(value.0.into())));
        line.p1 = MessageField(Some(Box::new(value.1.into())));
        line
    }
}

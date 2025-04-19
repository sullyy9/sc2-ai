use protobuf::MessageField;

use crate::game::geometry::Sphere;

use super::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawSphere {
    sphere: Sphere,
    color: Color,
}

impl DrawSphere {
    pub fn new(sphere: Sphere) -> Self {
        Self {
            sphere,
            color: Color::default(),
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl From<DrawSphere> for sc2_proto::debug::DebugSphere {
    fn from(value: DrawSphere) -> Self {
        let mut sphere = sc2_proto::debug::DebugSphere::new();
        sphere.p = MessageField(Some(Box::new((*value.sphere.center()).into())));
        sphere.set_r(value.sphere.radius());
        sphere.color = MessageField(Some(Box::new(value.color.into())));

        sphere
    }
}
impl From<sc2_proto::debug::DebugSphere> for DrawSphere {
    fn from(value: sc2_proto::debug::DebugSphere) -> Self {
        let sc2_proto::debug::DebugSphere {
            p: MessageField(Some(point)),
            r: Some(radius),
            color: MessageField(color),
            ..
        } = value
        else {
            panic!("Unexpected None value in sc2_proto::debug::DebugLine");
        };

        Self {
            sphere: Sphere::from_center((*point).into(), radius),
            color: color.map(|c| (*c).into()).unwrap_or_default(),
        }
    }
}

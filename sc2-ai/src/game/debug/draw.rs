use bevy::ecs::world::Command;
use protobuf::MessageField;

use crate::{
    core::DebugCommands,
    game::geometry::{Line, Cuboid, Sphere, Vec3},
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: u8::MAX,
            g: u8::MAX,
            b: u8::MAX,
        }
    }
}

impl From<sc2_proto::debug::Color> for Color {
    fn from(value: sc2_proto::debug::Color) -> Self {
        Self {
            r: value
                .r()
                .try_into()
                .expect("Out of range value for red channel"),
            g: value
                .g()
                .try_into()
                .expect("Out of range value for green channel"),
            b: value
                .b()
                .try_into()
                .expect("Out of range value for blue channel"),
        }
    }
}

impl From<Color> for sc2_proto::debug::Color {
    fn from(value: Color) -> Self {
        let mut color = sc2_proto::debug::Color::new();
        color.set_r(value.r.into());
        color.set_g(value.g.into());
        color.set_b(value.b.into());
        color
    }
}

#[derive(Clone, Debug, PartialEq)]
struct DrawText {
    text: String,
    position: Vec3,
    color: Option<Color>,
    size: Option<u32>,
}

impl From<DrawText> for sc2_proto::debug::DebugText {
    fn from(value: DrawText) -> Self {
        let mut text = sc2_proto::debug::DebugText::new();
        text.set_text(value.text);
        text.world_pos = MessageField(Some(Box::new(value.position.into())));
        text.color = MessageField(value.color.map(|c| Box::new(c.into())));
        text.size = value.size;

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
            color: color.map(|c| (*c).into()),
            size,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct DrawLine {
    line: Line,
    color: Option<Color>,
}

impl From<DrawLine> for sc2_proto::debug::DebugLine {
    fn from(value: DrawLine) -> Self {
        let mut line = sc2_proto::debug::DebugLine::new();
        line.line = MessageField(Some(Box::new(value.line.into())));
        line.color = MessageField(value.color.map(|c| Box::new(c.into())));

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
            color: color.map(|c| (*c).into()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct DrawBox {
    rect: Cuboid,
    color: Option<Color>,
}

impl From<DrawBox> for sc2_proto::debug::DebugBox {
    fn from(value: DrawBox) -> Self {
        let mut boxx = sc2_proto::debug::DebugBox::new();
        boxx.min = MessageField(Some(Box::new((*value.rect.min()).into())));
        boxx.max = MessageField(Some(Box::new((*value.rect.max()).into())));
        boxx.color = MessageField(value.color.map(|c| Box::new(c.into())));

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
            rect: Cuboid::from_corners((*min).into(), (*max).into()),
            color: color.map(|c| (*c).into()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct DrawSphere {
    sphere: Sphere,
    color: Option<Color>,
}

impl From<DrawSphere> for sc2_proto::debug::DebugSphere {
    fn from(value: DrawSphere) -> Self {
        let mut sphere = sc2_proto::debug::DebugSphere::new();
        sphere.p = MessageField(Some(Box::new((*value.sphere.center()).into())));
        sphere.set_r(value.sphere.radius());
        sphere.color = MessageField(value.color.map(|c| Box::new(c.into())));

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
            color: color.map(|c| (*c).into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Draw {
    Text(DrawText),
    Line(DrawLine),
    Box(DrawBox),
    Sphere(DrawSphere),
}

impl From<Draw> for sc2_proto::debug::DebugDraw {
    fn from(value: Draw) -> Self {
        let mut draw = sc2_proto::debug::DebugDraw::new();

        match value {
            Draw::Line(line) => draw.lines.push(line.into()),
            Draw::Box(boxx) => draw.boxes.push(boxx.into()),
            Draw::Text(text) => draw.text.push(text.into()),
            Draw::Sphere(sphere) => draw.spheres.push(sphere.into()),
        }

        draw
    }
}

impl From<Draw> for sc2_proto::debug::DebugCommand {
    fn from(value: Draw) -> Self {
        let mut command = sc2_proto::debug::DebugCommand::new();
        command.set_draw(value.into());
        command
    }
}

impl Command for Draw {
    fn apply(self, world: &mut bevy::ecs::world::World) {
        let mut commands = world.resource_mut::<DebugCommands>();
        if let Some(cmd) = commands.iter_mut().find(|cmd| cmd.has_draw()) {
            let cmd = cmd.mut_draw();

            match self {
                Draw::Text(text) => cmd.text.push(text.into()),
                Draw::Line(line) => cmd.lines.push(line.into()),
                Draw::Box(boxx) => cmd.boxes.push(boxx.into()),
                Draw::Sphere(sphere) => cmd.spheres.push(sphere.into()),
            }
        } else {
            commands.push(self.into());
        }
    }
}

pub trait DrawCommandsExt {
    fn draw_text(&mut self, text: impl Into<String>, position: Vec3, color: Color);
    fn draw_line(&mut self, line: Line, color: Color);
    fn draw_box(&mut self, rect: Cuboid, color: Color);
    fn draw_sphere(&mut self, sphere: Sphere, color: Color);
}

impl DrawCommandsExt for bevy::ecs::system::Commands<'_, '_> {
    fn draw_text(&mut self, text: impl Into<String>, position: Vec3, color: Color) {
        self.queue(Draw::Text(DrawText {
            text: text.into(),
            position,
            color: Some(color),
            size: Some(12),
        }));
    }

    fn draw_line(&mut self, line: Line, color: Color) {
        self.queue(Draw::Line(DrawLine {
            line,
            color: Some(color),
        }));
    }

    fn draw_box(&mut self, rect: Cuboid, color: Color) {
        self.queue(Draw::Box(DrawBox {
            rect,
            color: Some(color),
        }));
    }
    fn draw_sphere(&mut self, sphere: Sphere, color: Color) {
        self.queue(Draw::Sphere(DrawSphere {
            sphere,
            color: Some(color),
        }));
    }
}

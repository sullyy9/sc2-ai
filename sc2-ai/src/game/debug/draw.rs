use bevy::ecs::world::Command;
use protobuf::MessageField;

use crate::{
    core::DebugCommands,
    game::geometry::{Line, Rect, Vec3},
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
enum Draw {
    Text(String, Vec3, Option<Color>, Option<u32>),
    Line(Line, Option<Color>),
    Box(Rect, Option<Color>),
    // Sphere(Point3, f32, Option<Color>),
}

impl From<sc2_proto::debug::DebugText> for Draw {
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

        Self::Text(text, (*position).into(), color.map(|c| (*c).into()), size)
    }
}

impl From<sc2_proto::debug::DebugLine> for Draw {
    fn from(value: sc2_proto::debug::DebugLine) -> Self {
        let sc2_proto::debug::DebugLine {
            line: MessageField(Some(line)),
            color: MessageField(color),
            ..
        } = value
        else {
            panic!("Unexpected None value in sc2_proto::debug::DebugLine");
        };

        Self::Line((*line).into(), color.map(|c| (*c).into()))
    }
}

impl From<sc2_proto::debug::DebugBox> for Draw {
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

        Self::Box(
            Rect::from_corners((*min).into(), (*max).into()),
            color.map(|c| (*c).into()),
        )
    }
}

impl From<Draw> for sc2_proto::debug::DebugDraw {
    fn from(value: Draw) -> Self {
        let mut draw = sc2_proto::debug::DebugDraw::new();
        match value {
            Draw::Line(line, color) => {
                let mut api_line = sc2_proto::debug::DebugLine::new();
                api_line.line = MessageField(Some(Box::new(line.into())));
                api_line.color = MessageField(color.map(|c| Box::new(c.into())));

                draw.lines.push(api_line);
            }
            Draw::Box(rect, color) => {
                let mut api_box = sc2_proto::debug::DebugBox::new();
                api_box.min = MessageField(Some(Box::new((*rect.min()).into())));
                api_box.max = MessageField(Some(Box::new((*rect.max()).into())));
                api_box.color = MessageField(color.map(|c| Box::new(c.into())));

                draw.boxes.push(api_box);
            }
            Draw::Text(text, point, color, size) => {
                let mut api_text = sc2_proto::debug::DebugText::new();
                api_text.set_text(text);
                api_text.world_pos = MessageField(Some(Box::new(point.into())));
                api_text.color = MessageField(color.map(|c| Box::new(c.into())));
                api_text.size = size;

                draw.text.push(api_text);
            }
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
        commands.push(self.into());
    }
}

pub trait DrawCommandsExt {
    fn draw_text(&mut self, text: impl Into<String>, position: Vec3, color: Color);
    fn draw_line(&mut self, line: Line, color: Color);
    fn draw_box(&mut self, rect: Rect, color: Color);
}

impl DrawCommandsExt for bevy::ecs::system::Commands<'_, '_> {
    fn draw_text(&mut self, text: impl Into<String>, position: Vec3, color: Color) {
        self.queue(Draw::Text(text.into(), position, Some(color), Some(12)));
    }

    fn draw_line(&mut self, line: Line, color: Color) {
        self.queue(Draw::Line(line, Some(color)));
    }

    fn draw_box(&mut self, rect: Rect, color: Color) {
        self.queue(Draw::Box(rect, Some(color)));
    }
}

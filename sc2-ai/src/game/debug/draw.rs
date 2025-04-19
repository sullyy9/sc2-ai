use bevy::ecs::world::Command;

use crate::{
    core::DebugCommands,
    game::{
        geometry::{Cuboid, Line2, Line3, Rect, Sphere, Vec3},
        map::HeightMap,
    },
};

use super::{
    color::Color,
    cuboid::{DrawBox, DrawSurfaceBox, DrawSurfaceRect},
    line::{DrawLine, DrawSurfaceLine},
    sphere::DrawSphere,
    text::DrawText,
};

/// The height given by the heightmap will cause drawing to clip or be just inside the terrain. A
/// slight verticle offset is needed.
pub(super) const SURFACE_Z_OFFSET: f32 = 0.05;

#[derive(Clone, Debug, PartialEq)]
enum Draw {
    Text(DrawText),
    Line(DrawLine),
    Box(DrawBox),
    Sphere(DrawSphere),

    SurfaceLine(DrawSurfaceLine),
    SurfaceRect(DrawSurfaceRect),
    SurfaceBox(DrawSurfaceBox),
}

impl Command for Draw {
    fn apply(mut self, world: &mut bevy::ecs::world::World) {
        // Map any surface drawings to 3D first in order to avoid mutliple borrows later.
        {
            let height_map = world.resource::<HeightMap>();

            self = match self {
                Draw::Text(_) => self,
                Draw::Line(_) => self,
                Draw::Box(_) => self,
                Draw::Sphere(_) => self,

                Draw::SurfaceLine(line) => Draw::Line(line.map_to_surface(height_map)),
                Draw::SurfaceRect(rect) => Draw::Box(rect.map_to_surface(height_map)),
                Draw::SurfaceBox(boxx) => Draw::Box(boxx.map_to_surface(height_map)),
            };
        }

        let mut commands = world.resource_mut::<DebugCommands>();

        let draw_cmd = if let Some(cmd) = commands.iter_mut().find(|cmd| cmd.has_draw()) {
            cmd.mut_draw()
        } else {
            commands.push(sc2_proto::debug::DebugCommand::new());
            commands.last_mut().unwrap().mut_draw()
        };

        match self {
            Draw::Text(text) => draw_cmd.text.push(text.into()),
            Draw::Line(line) => draw_cmd.lines.push(line.into()),
            Draw::Box(boxx) => draw_cmd.boxes.push(boxx.into()),
            Draw::Sphere(sphere) => draw_cmd.spheres.push(sphere.into()),

            Draw::SurfaceLine(_) | Draw::SurfaceRect(_) | Draw::SurfaceBox(_) => unreachable!(),
        }
    }
}

#[allow(unused)]
pub trait DrawCommandsExt {
    fn draw_text(&mut self, text: impl Into<String>, position: Vec3, color: Color);
    fn draw_line(&mut self, line: Line3, color: Color);
    fn draw_box(&mut self, rect: Cuboid, color: Color);
    fn draw_sphere(&mut self, sphere: Sphere, color: Color);

    fn draw_surface_line(&mut self, line: Line2, color: Color);
    fn draw_surface_rect(&mut self, rect: Rect, color: Color);
    fn draw_surface_box(&mut self, base: Rect, height: f32, color: Color);
}

impl DrawCommandsExt for bevy::ecs::system::Commands<'_, '_> {
    fn draw_text(&mut self, text: impl Into<String>, position: Vec3, color: Color) {
        self.queue(Draw::Text(
            DrawText::new(text.into(), position).with_color(color),
        ));
    }

    fn draw_line(&mut self, line: Line3, color: Color) {
        self.queue(Draw::Line(DrawLine::new(line).with_color(color)));
    }

    fn draw_box(&mut self, cuboid: Cuboid, color: Color) {
        self.queue(Draw::Box(DrawBox::new(cuboid).with_color(color)));
    }
    fn draw_sphere(&mut self, sphere: Sphere, color: Color) {
        self.queue(Draw::Sphere(DrawSphere::new(sphere).with_color(color)));
    }

    fn draw_surface_line(&mut self, line: Line2, color: Color) {
        self.queue(Draw::SurfaceLine(
            DrawSurfaceLine::new(line).with_color(color),
        ));
    }

    fn draw_surface_rect(&mut self, rect: Rect, color: Color) {
        self.queue(Draw::SurfaceRect(
            DrawSurfaceRect::new(rect).with_color(color),
        ));
    }

    fn draw_surface_box(&mut self, base: Rect, height: f32, color: Color) {
        self.queue(Draw::SurfaceBox(
            DrawSurfaceBox::new(base, height).with_color(color),
        ));
    }
}

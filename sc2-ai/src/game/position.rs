use bevy::ecs::component::Component;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
pub struct Position {
    x: f32,
    y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<sc2_proto::common::Point> for Position {
    fn from(value: sc2_proto::common::Point) -> Self {
        Self {
            x: value.x(),
            y: value.y(),
        }
    }
}

impl From<Position> for sc2_proto::common::Point2D {
    fn from(value: Position) -> Self {
        let mut point = sc2_proto::common::Point2D::new();
        point.set_x(value.x);
        point.set_y(value.y);
        point
    }
}

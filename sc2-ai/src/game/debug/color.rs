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

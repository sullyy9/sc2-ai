use bevy::ecs::system::{Commands, Res, ResMut, Resource};

use crate::{core::ApiMapInfo, game::geometry::Vec3};

#[derive(Resource, Default, Clone, Debug, PartialEq, Eq)]
pub struct HeightMap(ndarray::Array2<u8>);

impl From<sc2_proto::common::ImageData> for HeightMap {
    fn from(mut value: sc2_proto::common::ImageData) -> Self {
        debug_assert!(value.bits_per_pixel() == 8);

        let inner = ndarray::Array2::from_shape_vec(
            (value.size.y() as usize, value.size.x() as usize),
            value.take_data().into_iter().collect(),
        )
        .expect("Image data incompatible with placement grid");

        Self(inner)
    }
}

impl HeightMap {
    pub fn width(&self) -> usize {
        self.0.dim().1
    }

    pub fn height(&self) -> usize {
        self.0.dim().0
    }

    pub fn height_at(&self, point: Vec3) -> Option<f32> {
        let coords = (point.y.floor() as usize, point.x.floor() as usize);
        self.0
            .get(coords)
            .map(|&z| (((z as f32) * 32.0) / 255.0) - 16.0)
    }
}

impl HeightMap {
    pub fn init(info: Res<ApiMapInfo>, map: Option<ResMut<HeightMap>>, mut commands: Commands) {
        let new_map = HeightMap::from((*info.terrain_height).clone());

        if let Some(mut map) = map {
            *map = new_map;
        } else {
            commands.insert_resource(new_map);
        };
    }
}

use bevy::ecs::system::{Commands, Res, ResMut, Resource};
use duplicate::duplicate_item;

use crate::{core::ApiMapInfo, game::geometry::Vec2};

#[derive(Resource, Default, Clone, Debug, PartialEq)]
pub struct HeightMap(ndarray::Array2<f32>);

impl From<sc2_proto::common::ImageData> for HeightMap {
    fn from(mut value: sc2_proto::common::ImageData) -> Self {
        debug_assert!(value.bits_per_pixel() == 8);

        let inner = ndarray::Array2::from_shape_vec(
            (value.size.y() as usize, value.size.x() as usize),
            value
                .take_data()
                .into_iter()
                .map(|height| (((height as f32) * 32.0) / 255.0) - 16.0)
                .collect(),
        )
        .expect("Image data incompatible with placement grid");

        Self(inner)
    }
}

#[duplicate_item(
    HeightMap;
    [ HeightMap ];
    [ &HeightMap ];

)]
impl std::ops::Index<Vec2> for HeightMap {
    type Output = f32;

    fn index(&self, index: Vec2) -> &Self::Output {
        let coords = (index.y.floor() as usize, index.x.floor() as usize);
        self.0.get(coords).expect("Point should be within the map")
    }
}

impl HeightMap {
    pub fn width(&self) -> usize {
        self.0.dim().1
    }

    pub fn height(&self) -> usize {
        self.0.dim().0
    }

    pub fn height_at(&self, point: Vec2) -> Option<f32> {
        let coords = (point.y.floor() as usize, point.x.floor() as usize);
        self.0.get(coords).cloned()
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

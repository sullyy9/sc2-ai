use bevy::ecs::{
    entity::Entity,
    event::EventReader,
    system::{Commands, Query, Res, ResMut, Resource},
};
use ndarray::s;

use crate::{
    core::ApiMapInfo,
    game::{
        debug::{Color, DrawCommandsExt as _},
        entity::{EntityFound, MapEntity},
        geometry::{Cuboid, Vec3},
    },
};

use super::HeightMap;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
enum GridStatus {
    #[default]
    Invalid,
    Empty,
}

#[derive(Resource, Default, Clone, Debug, PartialEq, Eq)]
pub struct PlacementGrid(ndarray::Array2<GridStatus>);

impl From<sc2_proto::common::ImageData> for PlacementGrid {
    fn from(value: sc2_proto::common::ImageData) -> Self {
        debug_assert!(value.bits_per_pixel() == 1);

        let inner = ndarray::Array2::from_shape_vec(
            (value.size.y() as usize, value.size.x() as usize),
            value
                .data()
                .iter()
                .flat_map(|byte| {
                    (0..8)
                        .rev()
                        .map(|i| {
                            if ((byte >> i) & 1) == 0 {
                                GridStatus::Invalid
                            } else {
                                GridStatus::Empty
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect(),
        )
        .expect("Image data incompatible with placement grid");

        Self(inner)
    }
}

impl PlacementGrid {
    pub fn width(&self) -> usize {
        self.0.dim().1
    }

    pub fn height(&self) -> usize {
        self.0.dim().0
    }

    /// Determine if a single grid space is empty and an entity can be placed there.
    pub fn is_empty(&self, point: Vec3) -> bool {
        let coords = (point.y.floor() as usize, point.x.floor() as usize);
        self.0
            .get(coords)
            .is_some_and(|&status| status == GridStatus::Empty)
    }

    /// Determine if a single grid space is an invalid placement location.
    pub fn is_invalid(&self, point: Vec3) -> bool {
        let coords = (point.y.floor() as usize, point.x.floor() as usize);
        !self
            .0
            .get(coords)
            .is_none_or(|&status| status == GridStatus::Invalid)
    }

    /// Determine if a grid area is empty and an entity can be placed there.
    pub fn is_area_empty(&self, rect: Cuboid) -> bool {
        let (min_x, min_y) = (rect.min().x.floor() as usize, rect.min().y.floor() as usize);
        let (max_x, max_y) = (rect.max().x.floor() as usize, rect.max().y.floor() as usize);

        self.0
            .slice(s![min_y..max_y, min_x..max_x])
            .iter()
            .all(|&status| status == GridStatus::Empty)
    }
}

/// Bevy systems.
impl PlacementGrid {
    pub fn init(map: Res<ApiMapInfo>, grid: Option<ResMut<PlacementGrid>>, mut commands: Commands) {
        let new_grid = PlacementGrid::from((*map.placement_grid).clone());

        if let Some(mut grid) = grid {
            *grid = new_grid;
        } else {
            commands.insert_resource(new_grid);
        };
    }

    pub fn entity_found_handler<T>(
        mut events: EventReader<EntityFound<T>>,
        mut grid: ResMut<PlacementGrid>,
        entities: Query<(Entity, &Vec3)>,
    ) where
        T: MapEntity + Send + Sync + 'static,
    {
        for event in events.read() {
            let Ok((_, position)) = entities.get(event.entity) else {
                continue;
            };

            // Position is the center of the entities base.
            let min_y = (position.y - (T::SIZE.y / 2.0)).floor() as usize;
            let max_y = (position.y + (T::SIZE.y / 2.0)).ceil() as usize;

            let min_x = (position.x - (T::SIZE.x / 2.0)).floor() as usize;
            let max_x = (position.x + (T::SIZE.x / 2.0)).ceil() as usize;

            grid.0
                .slice_mut(s![min_y..max_y, min_x..max_x])
                .iter_mut()
                .for_each(|cell| *cell = GridStatus::Invalid);
        }
    }

    pub fn draw(mut commands: Commands, grid: Res<PlacementGrid>, height_map: Res<HeightMap>) {
        // Every combination of x and y coordinates.
        let coords = (0..grid.width()).flat_map(|x| (0..grid.height()).map(move |y| (x, y)));

        let coords = coords
            .map(|(x, y)| Vec3::new_2d(x as f32, y as f32))
            .filter(|pos| grid.is_empty(*pos));

        for pos in coords {
            let height = height_map.height_at(pos).unwrap_or_default();

            commands.draw_box(
                Cuboid::from_base_center(
                    pos + Vec3::new_3d(0.5, 0.5, height + 0.05),
                    Vec3::new_2d(1.0, 1.0),
                ),
                Color::GREEN,
            );
        }
    }
}

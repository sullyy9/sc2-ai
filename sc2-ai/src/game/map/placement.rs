use bevy::ecs::system::Resource;
use ndarray::s;

use crate::game::geometry::{Rect, Vec3};

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
    pub fn is_area_empty(&self, rect: Rect) -> bool {
        let (min_x, min_y) = (rect.min().x.floor() as usize, rect.min().y.floor() as usize);
        let (max_x, max_y) = (rect.max().x.floor() as usize, rect.max().y.floor() as usize);

        self.0
            .slice(s![min_y..max_y, min_x..max_x])
            .iter()
            .all(|&status| status == GridStatus::Empty)
    }
}

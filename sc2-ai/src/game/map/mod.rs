use bevy::ecs::system::{Res, ResMut};

use crate::core::ApiMapInfo;

mod height;
mod placement;

pub use height::HeightMap;
pub use placement::PlacementGrid;

pub fn map_info_init(
    info: Res<ApiMapInfo>,
    mut placement_grid: ResMut<PlacementGrid>,
    mut height_map: ResMut<HeightMap>,
) {
    *placement_grid = PlacementGrid::from((*info.placement_grid).clone());
    *height_map = HeightMap::from((*info.terrain_height).clone());
}

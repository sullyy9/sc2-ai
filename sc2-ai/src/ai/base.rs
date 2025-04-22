use std::ops::Div;

use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{
        component::Component,
        entity::Entity,
        query::{Has, With},
        system::{Commands, Query},
    },
    hierarchy::{BuildChildren, Children},
};

use crate::game::{
    debug::{Color, DrawCommandsExt},
    entity::{
        GameEntity,
        building::Hatchery,
        map::{MineralPatch, VespeneGeyser},
    },
    geometry::{Line2, Rect, Vec2, Vec3},
};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct BaseAiPlugin;

impl Plugin for BaseAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, BaseSite::spawn_all);
        app.add_systems(Update, BaseSite::draw);
    }
}

/// Tag for the site of a base / expansion.
#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct BaseSite;

/// Tags a base site as unoccupied.
#[derive(Component, Default, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Unoccupied;

enum BaseResource {
    Mineral { entity: Entity, pos: Vec2 },
    Vespene { entity: Entity, pos: Vec2 },
}

impl BaseResource {
    pub fn entity(&self) -> Entity {
        match self {
            Self::Mineral { entity, .. } => *entity,
            Self::Vespene { entity, .. } => *entity,
        }
    }

    pub fn position(&self) -> Vec2 {
        match self {
            Self::Mineral { pos, .. } => *pos,
            Self::Vespene { pos, .. } => *pos,
        }
    }
}

impl BaseSite {
    fn spawn_all(
        mut commands: Commands,
        mineral_query: Query<(Entity, &Vec3), With<MineralPatch>>,
        vespene_query: Query<(Entity, &Vec3), With<VespeneGeyser>>,
        bases: Query<(Entity, &Vec3), With<Hatchery>>,
    ) {
        // Group resources by proximity.
        let minerals = mineral_query
            .into_iter()
            .map(|(e, pos)| (e, pos.without_z()))
            .map(|(entity, pos)| BaseResource::Mineral { entity, pos });

        let vespene = vespene_query
            .into_iter()
            .map(|(e, pos)| (e, pos.without_z()))
            .map(|(entity, pos)| BaseResource::Vespene { entity, pos });

        let resource_clusters = Self::cluster_resources(minerals.chain(vespene).collect(), 15.0);

        for group in resource_clusters {
            // Resource deposits have a perimeter around them in which a base building cannot be
            // placed. This is 3 tiles along each axis from the edge of the resource bounding box.
            // However it's 'curved' around corners, but the curve is drawn as squares on a grid,
            // making it a bit more complicated. Below we return the resource bounding box and the
            // minimum allowed distance^2 the closest point on the bases bounding box is allowed to
            // be. If the closest point is greater than this, it's guaranteed that the base is not
            // within the perimiter.
            let resource_exclusion_areas = group
                .iter()
                .map(|res| match res {
                    BaseResource::Mineral { pos, .. } => {
                        (Rect::from_center(*pos, MineralPatch::FOOTPRINT), 8.0)
                    }
                    BaseResource::Vespene { pos, .. } => {
                        (Rect::from_center(*pos, VespeneGeyser::FOOTPRINT), 5.0)
                    }
                })
                .collect::<Box<_>>();

            // Determine the area in which the base building will be placed. This will be the
            // rectangle bounding the resources plus an extra offset found through experimentation.
            let bbox = {
                let bbox = Rect::bounding_points(group.iter().map(|res| res.position()))
                    .expect("Resource group should contain at least one element");
                let size_offset = Hatchery::FOOTPRINT + Vec2::new(4.0, 4.0);
                let bbox = Rect::from_center(bbox.center(), bbox.size() + size_offset);

                // Align the bounding box to the grid by expanding it.
                let bbox = Rect::from_corners(bbox.min().floor(), bbox.max().ceil());

                // Now shrink the bounding box such that it's only the area in which the center of
                // the building will be placed.
                let center_offset = Hatchery::FOOTPRINT / 2.0;
                Rect::from_corners(bbox.min() + center_offset, bbox.max() - center_offset)
            };

            commands.draw_surface_box(bbox, 10.0, Color::default());

            let x_range = std::iter::successors(Some(bbox.min().x), |&x| {
                (x <= bbox.max().x).then_some(x + 1.0)
            });

            let y_range = std::iter::successors(Some(bbox.min().y), |&y| {
                (y <= bbox.max().y).then_some(y + 1.0)
            });

            // Create an iterator over every grid point within the bounding box.
            let grid_points = x_range
                .flat_map(|x| y_range.clone().map(move |y| (x, y)))
                .map(Vec2::from);

            // Eliminate points where the base building overlaps with resource exclusion boxes.
            let points = grid_points.filter(|&pos| {
                let base_box = Rect::from_center(pos, Hatchery::FOOTPRINT);

                resource_exclusion_areas
                    .iter()
                    .all(|(bbox, min_dist)| bbox.min_distance_squared(&base_box) >= *min_dist)
            });

            // Find the best location from the remaining points.
            let best = points
                .map(|point| {
                    let avg_distance = group
                        .iter()
                        .map(|res| res.position().distance(point))
                        .sum::<f32>()
                        .div(group.len() as f32);

                    (point, avg_distance)
                })
                .min_by(|(_, dist1), (_, dist2)| dist1.partial_cmp(dist2).unwrap());

            if let Some((point, _)) = best {
                let resources = group.iter().map(|res| res.entity()).collect::<Box<_>>();

                if let Some((base, _)) = bases
                    .iter()
                    .find(|(_, pos)| pos.without_z().distance(point) < 1.0)
                {
                    commands
                        .spawn((BaseSite, point))
                        .add_child(base)
                        .add_children(&resources);
                } else {
                    commands
                        .spawn((BaseSite, point, Unoccupied))
                        .add_children(&resources);
                }
            }
        }
    }

    fn cluster_resources(
        mut resources: Vec<BaseResource>,
        threshold: f32,
    ) -> Box<[Box<[BaseResource]>]> {
        let mut resource_groups = Vec::<Box<[BaseResource]>>::new();
        while !resources.is_empty() {
            let mut iter = resources.into_iter();
            let first = match iter.next() {
                Some(patch) => patch,
                None => break,
            };

            let (mut group, remaining) = iter.partition::<Vec<_>, _>(|resource| {
                first.position().distance(resource.position()) <= threshold
            });

            group.push(first);
            resource_groups.push(group.into_boxed_slice());

            resources = remaining;
        }
        resource_groups.into_boxed_slice()
    }

    pub fn draw(
        mut commands: Commands,
        bases: Query<(&Vec2, &Children, Has<Unoccupied>), With<BaseSite>>,
        minerals: Query<&Vec3, With<MineralPatch>>,
        gas: Query<&Vec3, With<VespeneGeyser>>,
    ) {
        for (&base_pos, resources, unoccupied) in bases.iter() {
            commands.draw_surface_box(
                Rect::from_center(base_pos, Hatchery::FOOTPRINT),
                Hatchery::SIZE.z,
                Color::BLUE,
            );

            if unoccupied {
                commands.draw_surface_text("Unoccupied Base Site", base_pos, Color::default());
            }

            for &resource in resources {
                let Ok(resource_pos) = minerals.get(resource).or_else(|_| gas.get(resource)) else {
                    continue;
                };

                let resource_pos = resource_pos.without_z();
                commands.draw_surface_line(Line2::new(base_pos, resource_pos), Color::BLUE);
            }
        }
    }
}

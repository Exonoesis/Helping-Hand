use std::collections::HashSet;

use bevy::prelude::*;

use crate::map::{render::RenderedMap, TileType, XyzCords};

#[derive(Component, Debug, Default)]
pub struct CollisionCollection {
    collision_tiles: HashSet<XyzCords>,
}

impl CollisionCollection {
    pub fn new() -> Self {
        let collision_tiles = HashSet::new();

        Self { collision_tiles }
    }

    pub fn has(&self, xyz_coord: &XyzCords) -> bool {
        // NOTE: Collision should apply to all layers, thus the z value does
        // not make sense, hence it being zeroed out.
        let xy_coord = XyzCords::new(xyz_coord.get_x(), xyz_coord.get_y(), 0);
        self.collision_tiles.contains(&xy_coord)
    }

    pub fn add(&mut self, xyz_coord: &XyzCords) {
        let xy_coord = XyzCords::new(xyz_coord.get_x(), xyz_coord.get_y(), 0);
        self.collision_tiles.insert(xy_coord);
    }
}

pub fn create_collision_collection_from(bevy_map: &RenderedMap) -> CollisionCollection {
    let mut collision_collection = CollisionCollection::new();

    let rendered_tiles = bevy_map.get_bevy_tiles();
    for rendered_tile in rendered_tiles {
        if rendered_tile.get_tile_type() != &TileType::Collision {
            continue;
        }

        let rendered_tile_coord = rendered_tile.get_grid_coordinates();
        collision_collection.add(rendered_tile_coord);
    }

    collision_collection
}

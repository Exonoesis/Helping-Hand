use std::collections::HashSet;

use bevy::prelude::*;

use crate::map::{GridCords3D, Tilemap};

#[derive(Component, Debug, Default)]
pub struct CollisionCollection {
    collision_tiles: HashSet<GridCords3D>,
}

impl CollisionCollection {
    pub fn new() -> Self {
        let collision_tiles = HashSet::new();

        Self { collision_tiles }
    }

    pub fn has(&self, xyz_coord: &GridCords3D) -> bool {
        // NOTE: Collision should apply to all layers, thus the z value does
        // not make sense, hence it being zeroed out.
        let xy_coord = GridCords3D::new(xyz_coord.get_x(), xyz_coord.get_y(), 0);
        self.collision_tiles.contains(&xy_coord)
    }

    pub fn add(&mut self, xyz_coord: &GridCords3D) {
        let xy_coord = GridCords3D::new(xyz_coord.get_x(), xyz_coord.get_y(), 0);
        self.collision_tiles.insert(xy_coord);
    }
}

// pub fn create_collision_collection_from(tiled_tiles: &Tilemap) -> CollisionCollection {
// }
pub fn create_collision_collection_from(tiled_tiles: &Tilemap) -> CollisionCollection {
    let mut collision_collection = CollisionCollection::new();

    let all_tiles = tiled_tiles.get_tiles();
    for tile in all_tiles {
        if let Some(tile_type) = tile.get_properties().get("type") {
            if tile_type != "Collision" {
                continue;
            }
        }

        let rendered_tile_coord = tile.get_grid_coordinates();
        collision_collection.add(rendered_tile_coord);
    }

    collision_collection
}

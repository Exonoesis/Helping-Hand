use bevy::prelude::*;

use cucumber::{given, then, when, World};

use tiled::{Loader, Map};

use std::path::PathBuf;

#[derive(Debug, Default, World)]
#[world(init = Self::new)]
struct GameWorld {
    pub app: App,
    pub map_location: PathBuf,
    pub loaded_map: Option<Map>,
}

struct Tile {
    tile_px_width: u32,
    tile_px_height: u32,
    px_x: u32,
    px_y: u32,
    px_z: u32,
    //layer_number: u32,
    //properties:
}

impl GameWorld {
    pub fn new() -> Self {
        let mut app = App::new();
        let map_location = PathBuf::new();
        let loaded_map = None;

        app.add_plugins(MinimalPlugins);

        Self { app, map_location, loaded_map }
    }
}

fn get_tiles(tiled_map: &Map) -> Vec<Tile> {
    let tile_width = tiled_map.tile_width;
    let tile_height = tiled_map.tile_height;

    let map_width = tiled_map.width;
    let map_height = tiled_map.height;

    let mut tiles = Vec::new();

    for y in 0..map_height {
        for x in 0..map_width {
            let tile = Tile {
                tile_px_width: tile_width,
                tile_px_height: tile_height,
                px_x: x * tile_width,
                px_y: y * tile_height,
                px_z: 0,
            };
            tiles.push(tile);
        }
    }

    tiles
}

/// Returns a Path to the specified Tiled Map
/// for a testing environment.
fn get_tiled_map_location(map_name: String) -> PathBuf {
    let mut tiled_map_path = PathBuf::new();

    // Appends the Manifest Directory which represents the root of the whole project.
    // We need this since we cannot use relative paths for testing purposes.
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        tiled_map_path.push(manifest_dir);
    }
    
    tiled_map_path.push("tests/test-assets/maps");
    tiled_map_path.push(map_name);

    tiled_map_path
}

fn get_map_column_num(tiles: &[Tile]) -> u32 {
    let mut highest_x = 0;
    let tile_width = tiles[0].tile_px_width;

    for tile in tiles {
        if tile.px_x >= highest_x {
            highest_x = tile.px_x;
        } else {
            break;
        }
    }

    (highest_x / tile_width) + 1
}

fn get_map_row_num(tiles: &[Tile]) -> u32 {
    let mut highest_y = 0;
    let tile_height = tiles[0].tile_px_height;

    for tile in tiles {
        if tile.px_y >= highest_y {
            highest_y = tile.px_y;
        } else {
            break;
        }
    }

    (highest_y / tile_height) + 1
}

#[given("a Tiled map called test_map.tmx,")]
fn verify_test_map_exists(world: &mut GameWorld) {
    let unloaded_tiled_map = get_tiled_map_location(String::from("test_map.tmx"));
    assert!(unloaded_tiled_map.exists(), "File does not exist at location {:?}", unloaded_tiled_map.canonicalize().unwrap());
    
    world.map_location = unloaded_tiled_map;
}

#[when("the Tiled map is loaded,")]
fn load_test_map(world: &mut GameWorld) {
    let mut loader = Loader::new();
    let tiled_map = loader.load_tmx_map(world.map_location.clone());
    assert!(tiled_map.is_ok());

    world.loaded_map = Some(tiled_map.unwrap());
}

#[then("there are 16 tiles loaded.")]
fn verify_loaded_tile_amount(world: &mut GameWorld) {
    let tiles = get_tiles(world.loaded_map.as_ref().unwrap());
    assert_eq!(tiles.len(), 16);
}

#[then("the tiles are in a 4x4 grid.")]
fn verify_tiles_are_a_grid(world: &mut GameWorld) {
    let tiles = get_tiles(world.loaded_map.as_ref().unwrap());
    let column_num = get_map_column_num(&tiles);
    let row_num = get_map_row_num(&tiles);
    
    assert_eq!(column_num, 4, "Column count is incorrect");
    assert_eq!(row_num, 4, "Row count is incorrect");
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(GameWorld::run("tests/feature-files/tilemap.feature"));
}
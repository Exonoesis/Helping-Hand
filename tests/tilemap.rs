use bevy::prelude::*;

use cucumber::{given, then, when, World};

use helping_hand::map::*;
use tiled::Loader;
use ldtk_json::LdtkJson;

use std::fs;

#[derive(Debug, Default, World)]
#[world(init = Self::new)]
struct GameWorld {
    pub app: App,
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

        app.add_plugins(MinimalPlugins);

        Self { app }
    }
}

fn spawn_map() {
    let mut loader = Loader::new();
    let tiled_map = loader.load_tmx_map("test-assets/maps/test_map.tmx").unwrap();
    
    let tile_width = tiled_map.tile_width;
    let tile_height = tiled_map.tile_height;

    let map_width = tiled_map.width;
    let map_height = tiled_map.height;

    for y in 0..map_height {
        for x in 0..map_width {
            let tile = Tile {
                tile_px_width: tile_width,
                tile_px_height: tile_height,
                px_x: x * tile_width,
                px_y: y * tile_height,
                px_z: 0,
            };
            spawn_tile(tile);
        }
    }
}

fn spawn_tile(tile: Tile){
    //TO-DO
}

// #[given("a LDtk map called test_map.ldtk,")]
// fn verify_test_map_exists(world: &mut GameWorld) {
//     unimplemented!()
// }

// #[when("the LDtk map is loaded,")]
// fn load_test_map(world: &mut GameWorld) {
//     world.app.add_systems(Startup, spawn_map);
//     world.app.update();
// }

// #[then("there are 4x4 (16) tiles loaded in a grid.")]
// fn verify_loaded_tile_amount(world: &mut GameWorld) {
//     unimplemented!()
// }

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(GameWorld::run("tests/feature-files/tilemap.feature"));
}

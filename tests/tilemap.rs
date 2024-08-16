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
    tile_texture: Option<TileTexture>,
    //layer_number: u32,
    //properties:
}
#[derive(Clone)]
struct TileTexture {
    spritesheet: PathBuf,
    sprite_index: usize,
}

impl GameWorld {
    pub fn new() -> Self {
        let mut app = App::new();
        let map_location = PathBuf::new();
        let loaded_map = None;

        app.add_plugins(MinimalPlugins);

        Self {
            app,
            map_location,
            loaded_map,
        }
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
                tile_texture: get_tile_texture(tiled_map, x, y),
            };
            tiles.push(tile);
        }
    }

    tiles
}

fn get_tile_texture(tiled_map: &Map, x_grid_cord: u32, y_grid_cord: u32) -> Option<TileTexture> {
    //Layer index hardcoded to 0 as we're currently assuming only one layer in map
    let tile_layer = tiled_map.get_layer(0).unwrap().as_tile_layer().unwrap();
    let tile = tile_layer
        .get_tile(x_grid_cord as i32, y_grid_cord as i32)
        .unwrap();

    let sprite_index = tile.id() as usize;
    let spritesheet = tile.get_tileset().image.clone().unwrap().source;

    if tile.id() != 0 {
        Some(TileTexture {
            sprite_index: sprite_index,
            spritesheet,
        })
    } else {
        None
    }
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

fn get_num_columns_from_map(tiles: &[Tile]) -> u32 {
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

fn get_num_rows_from_map(tiles: &[Tile]) -> u32 {
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
    assert!(
        unloaded_tiled_map.exists(),
        "File does not exist at location {:?}",
        unloaded_tiled_map.canonicalize().unwrap()
    );

    world.map_location = unloaded_tiled_map;
}

#[given("a Tiled map called single_sprite_sheet.tmx,")]
fn verify_single_sprite_sheet_exists(world: &mut GameWorld) {
    let unloaded_tiled_map = get_tiled_map_location(String::from("single_sprite_sheet.tmx"));
    assert!(
        unloaded_tiled_map.exists(),
        "File does not exist at location {:?}",
        unloaded_tiled_map.canonicalize().unwrap()
    );

    world.map_location = unloaded_tiled_map;
}

#[given("a Tiled map called multiple_sprite_sheet.tmx,")]
fn verify_multiple_sprite_sheet_exists(world: &mut GameWorld) {
    let unloaded_tiled_map = get_tiled_map_location(String::from("multiple_sprite_sheet.tmx"));
    assert!(
        unloaded_tiled_map.exists(),
        "File does not exist at location {:?}",
        unloaded_tiled_map.canonicalize().unwrap()
    );

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
fn verify_16_loaded_tiles(world: &mut GameWorld) {
    let tiles = get_tiles(world.loaded_map.as_ref().unwrap());
    assert_eq!(tiles.len(), 16);
}

#[then("there are 4 tiles loaded.")]
fn verify_4_loaded_tiles(world: &mut GameWorld) {
    let tiles = get_tiles(world.loaded_map.as_ref().unwrap());
    assert_eq!(tiles.len(), 4);
}

#[then("the tiles are in a 4x4 grid.")]
fn verify_tiles_are_a_grid(world: &mut GameWorld) {
    let tiles = get_tiles(world.loaded_map.as_ref().unwrap());
    let num_columns = get_num_columns_from_map(&tiles);
    let num_rows = get_num_rows_from_map(&tiles);

    assert_eq!(num_columns, 4, "Column count is incorrect");
    assert_eq!(num_rows, 4, "Row count is incorrect");
}

#[then("each tile points to the same sprite sheet.")]
fn verify_tiles_are_same_spritesheet(world: &mut GameWorld) {
    let tiles = get_tiles(world.loaded_map.as_ref().unwrap());
    let spritesheet = world.loaded_map.as_ref().unwrap().tilesets()[0]
        .image
        .clone()
        .unwrap()
        .source;

    for tile in tiles {
        assert_eq!(tile.tile_texture.unwrap().spritesheet, spritesheet);
    }
}

#[then("each tile points to the correct image on that sprite sheet.")]
fn verify_single_sheet_tile_images(world: &mut GameWorld) {
    let tiles = get_tiles(world.loaded_map.as_ref().unwrap());

    assert_eq!(tiles[0].tile_texture.clone().unwrap().sprite_index, 1);
    assert_eq!(tiles[1].tile_texture.clone().unwrap().sprite_index, 5);
    assert_eq!(tiles[2].tile_texture.clone().unwrap().sprite_index, 49);
    assert_eq!(tiles[3].tile_texture.clone().unwrap().sprite_index, 53);
}

#[then("the top two tiles point to one sprite sheet,")]
fn verify_top_two_sprites(world: &mut GameWorld) {
    let tiles = get_tiles(world.loaded_map.as_ref().unwrap());
    let spritesheet = world.loaded_map.as_ref().unwrap().tilesets()[0]
        .image
        .clone()
        .unwrap()
        .source;

    assert_eq!(
        tiles[0].tile_texture.clone().unwrap().spritesheet,
        spritesheet
    );
    assert_eq!(
        tiles[1].tile_texture.clone().unwrap().spritesheet,
        spritesheet
    );
}

#[then("the bottom two tiles point to the other sprite sheet,")]
fn verify_bottom_two_sprites(world: &mut GameWorld) {
    let tiles = get_tiles(world.loaded_map.as_ref().unwrap());
    let spritesheet = world.loaded_map.as_ref().unwrap().tilesets()[1]
        .image
        .clone()
        .unwrap()
        .source;

    assert_eq!(
        tiles[2].tile_texture.clone().unwrap().spritesheet,
        spritesheet
    );
    assert_eq!(
        tiles[3].tile_texture.clone().unwrap().spritesheet,
        spritesheet
    );
}

#[then("each tile points to the correct on its sprite sheet.")]
fn verify_multiple_sheet_tile_images(world: &mut GameWorld) {
    let tiles = get_tiles(world.loaded_map.as_ref().unwrap());

    assert_eq!(tiles[0].tile_texture.clone().unwrap().sprite_index, 131);
    assert_eq!(tiles[1].tile_texture.clone().unwrap().sprite_index, 128);
    assert_eq!(tiles[2].tile_texture.clone().unwrap().sprite_index, 115);
    assert_eq!(tiles[3].tile_texture.clone().unwrap().sprite_index, 164);
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(GameWorld::run("tests/feature-files/tilemap.feature"));
}

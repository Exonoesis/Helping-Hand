use bevy::prelude::*;

use cucumber::{given, then, when, World};

use tiled::{Loader, Map};

use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, Default, World)]
#[world(init = Self::new)]
struct GameWorld {
    pub app: App,
    pub map_location: PathBuf,
    pub loaded_map: Option<Map>,
}

#[derive(Clone)]
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

// TODO: Turn this into TiledMapReader if the map kepts being read?
// Example: tiled_map_reader = TiledMapReader::new(tiled_map);
//          tile_texture = tiled_map_reader.get_tile_texture(x, y);
fn get_tile_texture(tiled_map: &Map, x_grid_cord: u32, y_grid_cord: u32) -> Option<TileTexture> {
    //Layer index hardcoded to 0 as we're currently assuming only one layer in map
    let tile_layer = tiled_map.get_layer(0).unwrap().as_tile_layer().unwrap();

    if let Some(tile) = tile_layer.get_tile(x_grid_cord as i32, y_grid_cord as i32) {
        tile_layer
            .get_tile(x_grid_cord as i32, y_grid_cord as i32)
            .unwrap();

        let sprite_index = tile.id() as usize;
        let spritesheet = tile.get_tileset().image.clone().unwrap().source;

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

fn get_tile_spritesheet_filename(tile: Tile) -> OsString {
    let tile_texture = tile.tile_texture.clone().unwrap();
    let spritesheet = tile_texture.spritesheet.file_name().unwrap();

    spritesheet.into()
}

////////////////////////////

#[given(
    regex = r"a Tiled map called (test_map.tmx|single_sprite_sheet.tmx|multiple_sprite_sheet.tmx|one_blank.tmx),"
)]
fn verify_test_map_exists(world: &mut GameWorld, map_name: String) {
    let unloaded_tiled_map = get_tiled_map_location(map_name);

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

#[then(regex = r"there are (4|16) tiles loaded.")]
fn verify_num_loaded_tiles(world: &mut GameWorld, map_tile_count: String) {
    let tile_num = match map_tile_count.as_str() {
        "4" => 4,
        "16" => 16,
        _ => unreachable!("There's a choice we have not taken care of?"),
    };

    let tiles = get_tiles(world.loaded_map.as_ref().unwrap());
    assert_eq!(tiles.len(), tile_num);
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

    let spritesheet = OsString::from("atlas_64x.png");

    for tile in tiles {
        assert_eq!(get_tile_spritesheet_filename(tile), spritesheet);
    }
}

#[then(
    regex = r"each tile points to the correct image on the (single|multiple) sprite (sheet|sheets)."
)]
fn verify_sprite_sheet_tile_images(world: &mut GameWorld, spritesheet_num: String) {
    let (first_idx, second_idx, third_idx, fourth_idx) = match spritesheet_num.as_str() {
        "single" => (1, 5, 49, 53),
        "multiple" => (131, 128, 115, 164),
        _ => unreachable!(),
    };

    let tiles = get_tiles(world.loaded_map.as_ref().unwrap());

    assert_eq!(
        tiles[0].tile_texture.clone().unwrap().sprite_index,
        first_idx
    );
    assert_eq!(
        tiles[1].tile_texture.clone().unwrap().sprite_index,
        second_idx
    );
    assert_eq!(
        tiles[2].tile_texture.clone().unwrap().sprite_index,
        third_idx
    );
    assert_eq!(
        tiles[3].tile_texture.clone().unwrap().sprite_index,
        fourth_idx
    );
}

#[then(regex = r"the (top|bottom) two tiles point to (one|the other) sprite sheet,")]
fn verify_sprites_are_different_sprite_sheets(
    world: &mut GameWorld,
    tile_placement: String,
    spritesheet_choice: String,
) {
    let spritesheet_filename = match spritesheet_choice.as_str() {
        "one" => OsString::from("!CL_DEMO_64.png"),
        "the other" => OsString::from("atlas_64x.png"),
        _ => unreachable!("There's a choice we have not taken care of?"),
    };

    let (first_tile_idx, second_tile_idx) = match tile_placement.as_str() {
        "top" => (0, 1),
        "bottom" => (2, 3),
        _ => unreachable!("There's a choice we have not taken care of?"),
    };

    let tiles = get_tiles(world.loaded_map.as_ref().unwrap());

    assert_eq!(
        get_tile_spritesheet_filename(tiles[first_tile_idx].clone()),
        spritesheet_filename
    );
    assert_eq!(
        get_tile_spritesheet_filename(tiles[second_tile_idx].clone()),
        spritesheet_filename
    );
}

#[then("the first three tiles contain an image element,")]
fn verify_3_tiles_contain_images(world: &mut GameWorld) {
    let tiles = get_tiles(world.loaded_map.as_ref().unwrap());

    assert!(tiles[0].tile_texture.is_some());
    assert!(tiles[1].tile_texture.is_some());
    assert!(tiles[2].tile_texture.is_some());
}

#[then("the last tile has no image element.")]
fn verify_tile_image_is_empty(world: &mut GameWorld) {
    let tiles = get_tiles(world.loaded_map.as_ref().unwrap());

    assert!(tiles[3].tile_texture.is_none());
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(GameWorld::run("tests/feature-files/tilemap.feature"));
}

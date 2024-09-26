use bevy::prelude::*;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use bevy::sprite::SpritePlugin;

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
    pub bevy_asset_path: PathBuf,
}

#[derive(Clone)]
struct XyzCords {
    px_x: u32,
    px_y: u32,
    px_z: u32,
}

#[derive(Clone)]
struct PxDimensions {
    px_width: u32,
    px_height: u32,
}

#[derive(Clone)]
struct Tile {
    tile_dimensions: PxDimensions,
    px_cords: XyzCords,
    tile_texture: Option<TileTexture>,
    layer_number: usize,
    //properties:
}

#[derive(Clone)]
struct TileTexture {
    spritesheet: PathBuf,
    sprite_index: usize,
    spritesheet_demensions: PxDimensions,
}

#[derive(Bundle)]
struct RenderTileBundle {
    sprite_sheet_bundle: SpriteSheetBundle,
}

impl GameWorld {
    pub fn new() -> Self {
        let mut app = App::new();
        let map_location = PathBuf::new();
        let loaded_map = None;
        let bevy_asset_path = PathBuf::new();

        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(RenderPlugin {
            render_creation: WgpuSettings {
                backends: None,
                ..default()
            }
            .into(),
            ..default()
        });
        app.add_plugins(SpritePlugin::default());
        app.add_plugins(ImagePlugin::default());

        Self {
            app,
            map_location,
            loaded_map,
            bevy_asset_path,
        }
    }
}

fn get_map_tiles(tiled_map: &Map) -> Vec<Tile> {
    let tile_width = tiled_map.tile_width;
    let tile_height = tiled_map.tile_height;

    let map_width = tiled_map.width;
    let map_height = tiled_map.height;

    let mut tiles = Vec::new();

    for z in 0..tiled_map.layers().len() {
        for y in 0..map_height {
            for x in 0..map_width {
                let tile = Tile {
                    tile_dimensions: {
                        PxDimensions {
                            px_width: tile_width,
                            px_height: tile_height,
                        }
                    },
                    px_cords: {
                        XyzCords {
                            px_x: x * tile_width,
                            px_y: y * tile_height,
                            // Z value is currently equal to layer number,
                            // this may change in the future
                            px_z: z as u32,
                        }
                    },
                    layer_number: z,
                    tile_texture: get_tile_texture(tiled_map, x, y, z),
                };
                tiles.push(tile);
            }
        }
    }

    tiles
}

/// Returns a list of RenderTileBundles to be spawned by Bevy for the given list of tiles.
fn get_render_tile_bundles(
    tiles: &[Tile],
    asset_server: &AssetServer,
    texture_atlas_assets: &mut Assets<TextureAtlasLayout>,
) -> Vec<Option<RenderTileBundle>> {
    let mut render_tile_bundles = Vec::new();

    for tile in tiles {
        //Tiles without a texture don't need to be rendered
        if tile.tile_texture.is_none() {
            render_tile_bundles.push(None);
            continue;
        }

        //We have to trim our path from being absolute to having root at assets
        let bevy_path = to_bevy_path(&tile.tile_texture.clone().unwrap().spritesheet);
        let texture = asset_server.load(bevy_path);

        let sprite_sheet_column_count = (tile
            .tile_texture
            .clone()
            .unwrap()
            .spritesheet_demensions
            .px_width
            / tile.tile_dimensions.px_width) as usize;
        let sprite_sheet_row_count = (tile
            .tile_texture
            .clone()
            .unwrap()
            .spritesheet_demensions
            .px_height
            / tile.tile_dimensions.px_height) as usize;

        //This is how the sprite sheet should be cut when creating sprites to render
        let sheet_layout = TextureAtlasLayout::from_grid(
            Vec2::new(
                tile.tile_dimensions.px_width as f32,
                tile.tile_dimensions.px_height as f32,
            ),
            sprite_sheet_column_count,
            sprite_sheet_row_count,
            None,
            None,
        );

        // Conversion to Bevy specific formatting happens right here
        // Our:RenderTileBundle -> Bevy's:SpriteSheetBundle
        let render_tile = Some(RenderTileBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                transform: Transform::from_xyz(
                    tile.px_cords.px_x as f32,
                    tile.px_cords.px_y as f32,
                    tile.px_cords.px_z as f32,
                ),
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_assets.add(sheet_layout),
                    index: tile.tile_texture.clone().unwrap().sprite_index,
                },
                ..Default::default()
            },
        });
        render_tile_bundles.push(render_tile);
    }
    render_tile_bundles
}

// TODO: Turn this into TiledMapReader if the map kepts being read?
// Example: tiled_map_reader = TiledMapReader::new(tiled_map);
//          tile_texture = tiled_map_reader.get_tile_texture(x, y);
fn get_tile_texture(
    tiled_map: &Map,
    x_grid_cord: u32,
    y_grid_cord: u32,
    layer_num: usize,
) -> Option<TileTexture> {
    let tile_layer = tiled_map
        .get_layer(layer_num)
        .unwrap()
        .as_tile_layer()
        .unwrap();

    if let Some(tile) = tile_layer.get_tile(x_grid_cord as i32, y_grid_cord as i32) {
        tile_layer
            .get_tile(x_grid_cord as i32, y_grid_cord as i32)
            .unwrap();

        let sprite_index = tile.id() as usize;
        let spritesheet = tile.get_tileset().image.clone().unwrap().source;
        let spritesheet_px_width = tile.get_tileset().image.clone().unwrap().width as u32;
        let spritesheet_px_height = tile.get_tileset().image.clone().unwrap().height as u32;

        Some(TileTexture {
            sprite_index,
            spritesheet,
            spritesheet_demensions: {
                PxDimensions {
                    px_width: spritesheet_px_width,
                    px_height: spritesheet_px_height,
                }
            },
        })
    } else {
        None
    }
}

fn to_bevy_path(tiled_path: &PathBuf) -> PathBuf {
    let mut trimmed_path = PathBuf::new();
    let mut path_element_stack = Vec::new();

    for path_element in tiled_path.iter().rev() {
        if path_element == "assets" {
            break;
        }

        path_element_stack.push(path_element);
    }

    while let Some(path_element) = path_element_stack.pop() {
        trimmed_path.push(path_element);
    }

    return trimmed_path;
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
    let tile_width = tiles[0].tile_dimensions.px_width;

    for tile in tiles {
        if tile.px_cords.px_x >= highest_x {
            highest_x = tile.px_cords.px_x;
        } else {
            break;
        }
    }

    (highest_x / tile_width) + 1
}

fn get_num_rows_from_map(tiles: &[Tile]) -> u32 {
    let mut highest_y = 0;
    let tile_height = tiles[0].tile_dimensions.px_height;

    for tile in tiles {
        if tile.px_cords.px_y >= highest_y {
            highest_y = tile.px_cords.px_y;
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

#[given(regex = r"a Tiled map called (.+\.tmx),")]
fn verify_test_map_exists(world: &mut GameWorld, map_name: String) {
    let unloaded_tiled_map = get_tiled_map_location(map_name);

    assert!(
        unloaded_tiled_map.exists(),
        "File does not exist at location {:?}",
        unloaded_tiled_map.canonicalize().unwrap()
    );

    world.map_location = unloaded_tiled_map;
}

#[given(regex = r"an absolute asset path of (.+\.png),")]
fn set_absolute_path(world: &mut GameWorld, absolute_asset_path: String) {
    world.bevy_asset_path = PathBuf::from(absolute_asset_path);
}

#[when("the Tiled map is loaded,")]
fn load_test_map(world: &mut GameWorld) {
    let mut loader = Loader::new();
    let tiled_map = loader.load_tmx_map(world.map_location.clone());
    assert!(tiled_map.is_ok());

    world.loaded_map = Some(tiled_map.unwrap());
}

#[when("the absolute path is trimmed,")]
fn trim_to_bevy_path(world: &mut GameWorld) {
    let original_path = &world.bevy_asset_path;
    let trimmed_path = to_bevy_path(original_path);

    world.bevy_asset_path = trimmed_path;
}

#[then(regex = r"there are ([0-9]+) tiles loaded.")]
fn verify_num_loaded_tiles(world: &mut GameWorld, map_tile_count: String) {
    let tile_num = map_tile_count
        .parse::<usize>()
        .expect("verify_num_loaded_tiles: map_tile_count is not a number?");

    let tiles = get_map_tiles(world.loaded_map.as_ref().unwrap());
    assert_eq!(tiles.len(), tile_num);
}

#[then("the tiles are in a 4x4 grid.")]
fn verify_tiles_are_a_grid(world: &mut GameWorld) {
    let tiles = get_map_tiles(world.loaded_map.as_ref().unwrap());
    let num_columns = get_num_columns_from_map(&tiles);
    let num_rows = get_num_rows_from_map(&tiles);

    assert_eq!(num_columns, 4, "Column count is incorrect");
    assert_eq!(num_rows, 4, "Row count is incorrect");
}

#[then("each tile points to the same sprite sheet.")]
fn verify_tiles_are_same_spritesheet(world: &mut GameWorld) {
    let tiles = get_map_tiles(world.loaded_map.as_ref().unwrap());

    let spritesheet = OsString::from("atlas_64x.png");

    for tile in tiles {
        assert_eq!(get_tile_spritesheet_filename(tile.clone()), spritesheet);
        assert_eq!(
            tile.tile_texture
                .clone()
                .unwrap()
                .spritesheet_demensions
                .px_height,
            1024
        );
        assert_eq!(
            tile.tile_texture
                .clone()
                .unwrap()
                .spritesheet_demensions
                .px_width,
            3072
        );
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

    let tiles = get_map_tiles(world.loaded_map.as_ref().unwrap());

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
    let (spritesheet_filename, spritesheet_width, spritesheet_height) =
        match spritesheet_choice.as_str() {
            "one" => (OsString::from("!CL_DEMO_64.png"), 2560, 1984),
            "the other" => (OsString::from("atlas_64x.png"), 3072, 1024),
            _ => unreachable!("There's a choice we have not taken care of?"),
        };

    let (first_tile_idx, second_tile_idx) = match tile_placement.as_str() {
        "top" => (0, 1),
        "bottom" => (2, 3),
        _ => unreachable!("There's a choice we have not taken care of?"),
    };

    let tiles = get_map_tiles(world.loaded_map.as_ref().unwrap());

    assert_eq!(
        get_tile_spritesheet_filename(tiles[first_tile_idx].clone()),
        spritesheet_filename
    );
    assert_eq!(
        get_tile_spritesheet_filename(tiles[second_tile_idx].clone()),
        spritesheet_filename
    );

    assert_eq!(
        tiles[first_tile_idx]
            .clone()
            .tile_texture
            .unwrap()
            .spritesheet_demensions
            .px_height,
        spritesheet_height
    );
    assert_eq!(
        tiles[second_tile_idx]
            .clone()
            .tile_texture
            .unwrap()
            .spritesheet_demensions
            .px_width,
        spritesheet_width
    );
}

#[then("the first three tiles contain an image element,")]
fn verify_3_tiles_contain_images(world: &mut GameWorld) {
    let tiles = get_map_tiles(world.loaded_map.as_ref().unwrap());

    assert!(tiles[0].tile_texture.is_some());
    assert!(tiles[1].tile_texture.is_some());
    assert!(tiles[2].tile_texture.is_some());
}

#[then("the last tile has no image element.")]
fn verify_tile_image_is_empty(world: &mut GameWorld) {
    let tiles = get_map_tiles(world.loaded_map.as_ref().unwrap());

    assert!(tiles[3].tile_texture.is_none());
}

#[then("there exist two layers of tiles,")]
fn verify_two_tile_layers(world: &mut GameWorld) {
    let tiles = get_map_tiles(world.loaded_map.as_ref().unwrap());

    //On a 2x2 map, the first four tiles are layer 0
    for x in 0..=3 {
        assert_eq!(tiles[x].layer_number, 0);
    }

    //On a 2x2 map, the second four tiles are layer 1
    for x in 4..=7 {
        assert_eq!(tiles[x].layer_number, 1);
    }
}

#[then("those two layers are overlapping.")]
fn verify_two_overlapping_tile_layers(world: &mut GameWorld) {
    let tiles = get_map_tiles(world.loaded_map.as_ref().unwrap());

    //Each tile on layer one shares an (x,y) position with a tile on layer 0
    //whose number in the tile list is offset by the number of tiles on layer 1
    for x in 0..=3 {
        let tile_on_layer_0 = (tiles[x].px_cords.px_x, tiles[x].px_cords.px_y);
        let tile_on_layer_1 = (tiles[x + 4].px_cords.px_x, tiles[x + 4].px_cords.px_y);

        assert_eq!(tile_on_layer_0, tile_on_layer_1);
    }
}

#[then("the first three tiles can be converted to RenderTileBundles,")]
fn verify_three_render_tile_bundles(world: &mut GameWorld) {
    let tiles = get_map_tiles(world.loaded_map.as_ref().unwrap());
    let asset_server = world.app.world.resource::<AssetServer>().clone();
    let mut texture_atlas_layout = world.app.world.resource_mut::<Assets<TextureAtlasLayout>>();

    let render_tile_bundles =
        get_render_tile_bundles(&tiles, &asset_server, &mut texture_atlas_layout);

    for idx in 0..=2 {
        assert!(render_tile_bundles[idx].is_some())
    }
}

#[then("the last tile cannot be converted to a RenderTileBundle.")]
fn verify_one_render_tile_bundle_is_none(world: &mut GameWorld) {
    let tiles = get_map_tiles(world.loaded_map.as_ref().unwrap());
    let asset_server = world.app.world.resource::<AssetServer>().clone();
    let mut texture_atlas_layout = world.app.world.resource_mut::<Assets<TextureAtlasLayout>>();

    let render_tile_bundles =
        get_render_tile_bundles(&tiles, &asset_server, &mut texture_atlas_layout);

    assert!(render_tile_bundles[3].is_none())
}

#[then(regex = r"the trimmed path should be (.+\.png).")]
fn verify_trimmed_path(world: &mut GameWorld, desired_asset_path: String) {
    let expected_path = PathBuf::from(desired_asset_path);
    let actual_path = &world.bevy_asset_path;

    assert_eq!(expected_path, *actual_path);
}

fn main() {
    futures::executor::block_on(GameWorld::run("tests/feature-files/tilemap.feature"));
}

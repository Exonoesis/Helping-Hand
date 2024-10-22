use std::fmt::Debug;

use bevy::prelude::*;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use bevy::sprite::SpritePlugin;

use helping_hand::visuals::map::*;

use cucumber::{given, then, when, World};

use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, Default, World)]
#[world(init = Self::new)]
struct GameWorld {
    pub app: App,
    pub map_location: PathBuf,
    pub assets_folder_path: PathBuf,

    pub loaded_map: Tilemap,
    pub bevy_map: RenderedMap,
}

impl GameWorld {
    pub fn new() -> Self {
        let map_location = PathBuf::new();
        let loaded_map = Tilemap::default();
        let bevy_map = RenderedMap::default();

        let absolute_assets_folder_path = PathBuf::new();

        // Testable "game"
        let mut app = App::new();
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
        app.add_plugins(SpritePlugin);
        app.add_plugins(ImagePlugin::default());

        Self {
            app,
            map_location,
            loaded_map,
            bevy_map,
            assets_folder_path: absolute_assets_folder_path,
        }
    }
}

// Returns a Path to the specified Tiled Map
// for a testing environment.
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

//////////////TEST FUNCTIONS//////////////

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
    world.assets_folder_path = PathBuf::from(absolute_asset_path);
}

#[when("the Tiled map is loaded,")]
fn load_test_map(world: &mut GameWorld) {
    world.loaded_map = Tilemap::new(world.map_location.clone());
}

#[when("the absolute path is trimmed,")]
fn trim_to_bevy_path(world: &mut GameWorld) {
    let original_path = &world.assets_folder_path;
    let trimmed_path = to_bevy_path(original_path);

    world.assets_folder_path = trimmed_path;
}

#[when("the Tiled map has been converted to a rendered map,")]
fn tiled_map_to_bevy_tiles(world: &mut GameWorld) {
    let tilemap = &world.loaded_map;
    let asset_server = world.app.world.resource::<AssetServer>().clone();
    let mut texture_atlas_layout = world.app.world.resource_mut::<Assets<TextureAtlasLayout>>();

    let rendered_bevy_map = RenderedMap::new(tilemap, &asset_server, &mut texture_atlas_layout);
    world.bevy_map = rendered_bevy_map;
}

#[then(regex = r"there are ([0-9]+) tiles loaded.")]
fn verify_num_loaded_tiles(world: &mut GameWorld, map_tile_count: String) {
    let expected_num_tiles = map_tile_count
        .parse::<usize>()
        .expect("verify_num_loaded_tiles: map_tile_count is not a number?");
    let actual_num_tiles = world.loaded_map.get_tiles().len();
    assert_eq!(expected_num_tiles, actual_num_tiles);
}

#[then(regex = r"the tiles are in a ([0-9]+)x([0-9]+) grid.")]
fn verify_tiles_are_a_grid(world: &mut GameWorld, column_num: String, row_num: String) {
    let expected_num_rows = column_num
        .parse::<u32>()
        .expect("verify_tiles_are_a_grid: column is not a number?");

    let expected_num_columns = row_num
        .parse::<u32>()
        .expect("verify_tiles_are_a_grid: row is not a number?");

    let actual_map_dimensions = world.loaded_map.get_grid_dimensions();
    let expected_map_dimensions =
        MapGridDimensions::new(expected_num_rows, expected_num_columns, 1);

    assert_eq!(expected_map_dimensions, *actual_map_dimensions);
}

#[then(regex = r"tile ([0-9]+) points to spritesheet (.+\.png).")]
fn verify_tile_spritesheet(world: &mut GameWorld, tile_num: String, spritesheet_name: String) {
    let tile_index = tile_num
        .parse::<usize>()
        .expect("verify_tile_spritesheet: tile_num is not a number?");

    let actual_spritesheet =
        world.loaded_map.get_tiles()[tile_index - 1].get_tile_spritesheet_filename();
    let expected_spritesheet = OsString::from(spritesheet_name);
    assert_eq!(expected_spritesheet, actual_spritesheet);
}

#[then(regex = r"tile ([0-9]+)'s spritesheet has dimensions ([0-9]+) x ([0-9]+).")]
fn verify_spritesheet_dimensions(
    world: &mut GameWorld,
    tile_num: String,
    sheet_height: String,
    sheet_width: String,
) {
    let tile_index = tile_num
        .parse::<usize>()
        .expect("verify_spritesheet_dimensions: tile_num is not a number?");

    let expected_spritesheet_height = sheet_height
        .parse::<u32>()
        .expect("verify_spritesheet_dimensions: sheet_height is not a number?");

    let expected_spritesheet_width = sheet_width
        .parse::<u32>()
        .expect("verify_spritesheet_dimensions: sheet_width is not a number?");

    let actual_dimensions =
        world.loaded_map.get_tiles()[tile_index - 1].get_spritesheet_dimensions();
    let expected_dimensions =
        PxDimensions::new(expected_spritesheet_width, expected_spritesheet_height);
    assert_eq!(expected_dimensions, *actual_dimensions);
}

#[then(regex = r"tile ([0-9]+) points to image number ([0-9]+).")]
fn verify_spritesheet_tile_image(world: &mut GameWorld, tile_num: String, image_num: String) {
    let tile_index = tile_num
        .parse::<usize>()
        .expect("verify_spritesheet_tile_image: tile_num is not a number?");
    let image_index = image_num
        .parse::<usize>()
        .expect("verify_spritesheet_tile_image: image_num is not a number?");

    let actual_image = world.loaded_map.get_tiles()[tile_index - 1].get_sprite_index();
    let expected_image = image_index;
    assert_eq!(expected_image, actual_image);
}

#[then(regex = r"tile ([0-9]+) contains an image element.")]
fn verify_tile_contains_image(world: &mut GameWorld, tile_num: String) {
    let tile_index = tile_num
        .parse::<usize>()
        .expect("verify_tile_contains_image: tile_num is not a number?");

    let tile_image = &world.loaded_map.get_tiles()[tile_index - 1].get_tile_texture();

    assert!(tile_image.is_some());
}

#[then(regex = r"tile ([0-9]+) contains no image element.")]
fn verify_tile_image_is_empty(world: &mut GameWorld, tile_num: String) {
    let tile_index = tile_num
        .parse::<usize>()
        .expect("verify_tile_contains_image: tile_num is not a number?");

    let tile_image = &world.loaded_map.get_tiles()[tile_index - 1].get_tile_texture();
    assert!(tile_image.is_none());
}

#[then(regex = r"there exist ([0-9]+) layers of tiles.")]
fn verify_layer_count(world: &mut GameWorld, num_layers: String) {
    let layer_count = num_layers
        .parse::<u32>()
        .expect("verify_layer_count: num_layers is not a number?");

    let actual_num_layers = world.loaded_map.get_grid_dimensions().get_layers();
    let expected_num_layers = layer_count;
    assert_eq!(expected_num_layers, actual_num_layers);
}

#[then(regex = r"tile ([0-9]+) overlaps tile ([0-9]+).")]
fn verify_overlapping_tiles(world: &mut GameWorld, first_tile: String, second_tile: String) {
    let first_tile_index = first_tile
        .parse::<usize>()
        .expect("verify_overlapping_tiles: first_tile is not a number?");
    let second_tile_index = second_tile
        .parse::<usize>()
        .expect("verify_overlapping_tiles: second_tile is not a number?");

    let is_overlapping = world
        .loaded_map
        .tiles_overlap(first_tile_index - 1, second_tile_index - 1);

    assert!(is_overlapping);
}

#[then(regex = r"Tiled tile ([0-9]+) overlaps Bevy tile ([0-9]+)")]
fn verify_y_axis_flip(world: &mut GameWorld, tiled_tile_num: String, bevy_tile_num: String) {
    let tiled_tile_index = tiled_tile_num
        .parse::<usize>()
        .expect("verify_y_axis_flip: Tiled tile is not a number?");

    let bevy_tile_index = bevy_tile_num
        .parse::<usize>()
        .expect("verify_y_axis_flip: Bevy tile is not a number?");

    let tiled_map = &world.loaded_map;

    let cross_map_overlap =
        world
            .bevy_map
            .tiled_map_overlap(tiled_map, tiled_tile_index - 1, bevy_tile_index - 1);

    assert!(cross_map_overlap);
}

#[then(regex = r"tile ([0-9]+) is in the rendered map.")]
fn verify_render_tile_is_some(world: &mut GameWorld, tile_num: String) {
    let tile_index = tile_num
        .parse::<usize>()
        .expect("verify_render_tile_is_some: tile_num is not a number?");

    let rendered_tile = &world.bevy_map.get_bevy_tiles()[tile_index - 1];
    assert!(rendered_tile.is_some());
}

#[then(regex = r"tile ([0-9]+) is not in the rendered map.")]
fn verify_render_tile_is_none(world: &mut GameWorld, tile_num: String) {
    let tile_index = tile_num
        .parse::<usize>()
        .expect("verify_render_tile_is_non: tile_num is not a number?");

    let rendered_tile = &world.bevy_map.get_bevy_tiles()[tile_index - 1];
    assert!(rendered_tile.is_none())
}

#[then(regex = r"the trimmed path should be (.+\.png).")]
fn verify_trimmed_path(world: &mut GameWorld, desired_asset_path: String) {
    let expected_path = PathBuf::from(desired_asset_path);
    let actual_path = &world.assets_folder_path;

    assert_eq!(expected_path, *actual_path);
}

fn main() {
    futures::executor::block_on(GameWorld::run("tests/feature-files/tilemap.feature"));
}

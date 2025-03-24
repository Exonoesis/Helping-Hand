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
    let tiled_map = load_tiled_map(world.map_location.clone());
    world.loaded_map = Tilemap::from_tiled(&tiled_map);
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
fn verify_num_loaded_tiles(world: &mut GameWorld, expected_num_tiles: usize) {
    let actual_num_tiles = world.loaded_map.get_tiles().len();
    assert_eq!(expected_num_tiles, actual_num_tiles);
}

#[then(regex = r"the tiles are in a ([0-9]+)x([0-9]+) grid.")]
fn verify_tiles_are_a_grid(
    world: &mut GameWorld,
    expected_num_rows: u32,
    expected_num_columns: u32,
) {
    let actual_map_dimensions = world.loaded_map.get_grid_dimensions();
    let expected_map_dimensions = GridDimensions::new(expected_num_columns, expected_num_rows, 1);

    assert_eq!(expected_map_dimensions, *actual_map_dimensions);
}

#[then(regex = r"tile ([0-9]+),([0-9]+),([0-9]+) points to spritesheet (.+\.png).")]
fn verify_tile_spritesheet(
    world: &mut GameWorld,
    tile_x_cord: u32,
    tile_y_cord: u32,
    tile_z_cord: u32,
    spritesheet_name: String,
) {
    let tile = GridDimensions::new(tile_x_cord, tile_y_cord, tile_z_cord);
    let tile_index = three_d_to_one_d_cords(&tile, world.loaded_map.get_grid_dimensions()) as usize;

    let actual_spritesheet =
        world.loaded_map.get_tiles()[tile_index].get_tile_spritesheet_filename();
    let expected_spritesheet = OsString::from(spritesheet_name);
    assert_eq!(expected_spritesheet, actual_spritesheet);
}

#[then(
    regex = r"tile ([0-9]+),([0-9]+),([0-9]+)'s spritesheet has dimensions ([0-9]+) x ([0-9]+)."
)]
fn verify_spritesheet_dimensions(
    world: &mut GameWorld,
    tile_x_cord: u32,
    tile_y_cord: u32,
    tile_z_cord: u32,
    expected_spritesheet_height: u32,
    expected_spritesheet_width: u32,
) {
    let tile = GridDimensions::new(tile_x_cord, tile_y_cord, tile_z_cord);
    let tile_index = three_d_to_one_d_cords(&tile, world.loaded_map.get_grid_dimensions()) as usize;

    let actual_dimensions = world.loaded_map.get_tiles()[tile_index].get_spritesheet_dimensions();
    let expected_dimensions =
        PxDimensions::new(expected_spritesheet_width, expected_spritesheet_height);
    assert_eq!(expected_dimensions, *actual_dimensions);
}

#[then(regex = r"tile ([0-9]+),([0-9]+),([0-9]+) points to image number ([0-9]+).")]
fn verify_spritesheet_tile_image(
    world: &mut GameWorld,
    tile_x_cord: u32,
    tile_y_cord: u32,
    tile_z_cord: u32,
    image_index: usize,
) {
    let tile = GridDimensions::new(tile_x_cord, tile_y_cord, tile_z_cord);
    let tile_index = three_d_to_one_d_cords(&tile, world.loaded_map.get_grid_dimensions()) as usize;

    let actual_image = world.loaded_map.get_tiles()[tile_index].get_sprite_index();
    let expected_image = image_index;
    assert_eq!(expected_image, actual_image);
}

#[then(regex = r"tile ([0-9]+),([0-9]+),([0-9]+) contains an image element.")]
fn verify_tile_contains_image(
    world: &mut GameWorld,
    tile_x_cord: u32,
    tile_y_cord: u32,
    tile_z_cord: u32,
) {
    let tile = GridDimensions::new(tile_x_cord, tile_y_cord, tile_z_cord);
    let tile_index = three_d_to_one_d_cords(&tile, world.loaded_map.get_grid_dimensions()) as usize;

    let tile_image = &world.loaded_map.get_tiles()[tile_index].get_tile_texture();

    assert!(tile_image.is_some());
}

#[then(regex = r"tile ([0-9]+),([0-9]+),([0-9]+) contains no image element.")]
fn verify_tile_image_is_empty(
    world: &mut GameWorld,
    tile_x_cord: u32,
    tile_y_cord: u32,
    tile_z_cord: u32,
) {
    let tile = GridDimensions::new(tile_x_cord, tile_y_cord, tile_z_cord);
    let tile_index = three_d_to_one_d_cords(&tile, world.loaded_map.get_grid_dimensions()) as usize;

    let tile_image = &world.loaded_map.get_tiles()[tile_index].get_tile_texture();
    assert!(tile_image.is_none());
}

#[then(regex = r"there exist ([0-9]+) layers of tiles.")]
fn verify_layer_count(world: &mut GameWorld, layer_count: u32) {
    let actual_num_layers = world.loaded_map.get_grid_dimensions().get_layers();
    let expected_num_layers = layer_count;
    assert_eq!(expected_num_layers, actual_num_layers);
}

#[then(regex = r"tile ([0-9]+),([0-9]+),([0-9]+) overlaps tile ([0-9]+),([0-9]+),([0-9]+).")]
fn verify_overlapping_tiles(
    world: &mut GameWorld,
    tile_x_cord_1: u32,
    tile_y_cord_1: u32,
    tile_z_cord_1: u32,
    tile_x_cord_2: u32,
    tile_y_cord_2: u32,
    tile_z_cord_2: u32,
) {
    let tile_1 = GridDimensions::new(tile_x_cord_1, tile_y_cord_1, tile_z_cord_1);
    let first_tile_index =
        three_d_to_one_d_cords(&tile_1, world.loaded_map.get_grid_dimensions()) as usize;

    let tile_2 = GridDimensions::new(tile_x_cord_2, tile_y_cord_2, tile_z_cord_2);
    let second_tile_index =
        three_d_to_one_d_cords(&tile_2, world.loaded_map.get_grid_dimensions()) as usize;

    let is_overlapping = world
        .loaded_map
        .tiles_overlap(first_tile_index, second_tile_index);

    assert!(is_overlapping);
}

#[then(
    regex = r"Tiled tile ([0-9]+),([0-9]+),([0-9]+) is equivalent to Bevy tile ([0-9]+),([0-9]+),([0-9]+)"
)]
fn verify_y_axis_flip(
    world: &mut GameWorld,
    tiled_tile_x_cord: u32,
    tiled_tile_y_cord: u32,
    tiled_tile_z_cord: u32,
    bevy_tile_x_cord: u32,
    bevy_tile_y_cord: u32,
    bevy_tile_z_cord: u32,
) {
    let tiled_map = &world.loaded_map;

    let tiled_tile = GridDimensions::new(tiled_tile_x_cord, tiled_tile_y_cord, tiled_tile_z_cord);
    let tiled_tile_index =
        three_d_to_one_d_cords(&tiled_tile, tiled_map.get_grid_dimensions()) as usize;

    let bevy_tile = GridDimensions::new(bevy_tile_x_cord, bevy_tile_y_cord, bevy_tile_z_cord);
    let bevy_tile_index =
        three_d_to_one_d_cords(&bevy_tile, tiled_map.get_grid_dimensions()) as usize;

    let cross_map_overlap =
        world
            .bevy_map
            .tiled_map_overlap(tiled_map, tiled_tile_index, bevy_tile_index);

    assert!(cross_map_overlap);
}

#[then(regex = r"there should be ([0-9]+) rendered tiles created.")]
fn verify_number_of_rendered_tiles(world: &mut GameWorld, expected_number_of_tiles: usize) {
    let actual_number_of_tiles = world.bevy_map.get_bevy_tiles().len();
    assert_eq!(expected_number_of_tiles, actual_number_of_tiles);
}

#[then(regex = r"the trimmed path should be (.+\.png).")]
fn verify_trimmed_path(world: &mut GameWorld, desired_asset_path: String) {
    let expected_path = PathBuf::from(desired_asset_path);
    let actual_path = &world.assets_folder_path;

    assert_eq!(expected_path, *actual_path);
}

#[then(regex = r"there is ([0-9]+) players? in the Tiled map.")]
fn verify_number_of_players(world: &mut GameWorld, expected_player_amount: usize) {
    let actual_player_amount = world.loaded_map.get_players().len();

    assert_eq!(expected_player_amount, actual_player_amount)
}

#[then(regex = r"there is ([0-9]+) players? in the Rendered map.")]
fn verify_number_of_players_on_rendered_map(world: &mut GameWorld, expected_player_amount: usize) {
    let actual_player_amount = world
        .bevy_map
        .get_bevy_tiles()
        .iter()
        .filter(|tile| tile.get_tile_type() == &TileType::Player)
        .count();

    assert_eq!(expected_player_amount, actual_player_amount)
}

#[then(regex = r"that player is at tile ([0-9]+),([0-9]+),([0-9]+).")]
fn verify_player_location(
    world: &mut GameWorld,
    tile_x_cord: u32,
    tile_y_cord: u32,
    tile_z_cord: u32,
) {
    let tile = GridDimensions::new(tile_x_cord, tile_y_cord, tile_z_cord);
    let tile_index = three_d_to_one_d_cords(&tile, world.loaded_map.get_grid_dimensions()) as usize;

    let player_tile = &world.loaded_map.get_tiles()[tile_index];
    let player_at_tile = *player_tile.get_tile_type() == TileType::Player;

    assert!(player_at_tile);
}

#[then(regex = r"that player on the Rendered map is at tile ([0-9]+),([0-9]+),([0-9]+).")]
fn verify_player_location_on_render_map(
    world: &mut GameWorld,
    tile_x_cord: u32,
    tile_y_cord: u32,
    tile_z_cord: u32,
) {
    let tile = GridDimensions::new(tile_x_cord, tile_y_cord, tile_z_cord);
    let tile_index = three_d_to_one_d_cords(&tile, world.loaded_map.get_grid_dimensions()) as usize;

    let player_tile = &world.bevy_map.get_bevy_tiles()[tile_index];
    let player_at_tile = *player_tile.get_tile_type() == TileType::Player;

    assert!(player_at_tile);
}

#[then(regex = r"3D cords ([0-9]+),([0-9]+),([0-9]+) point to tile index ([0-9]+).")]
fn verify_cords_convert_from_3d_to_1d(
    world: &mut GameWorld,
    tile_x_cord: u32,
    tile_y_cord: u32,
    tile_z_cord: u32,
    tile_index: u32,
) {
    let map_dimensions = world.loaded_map.get_grid_dimensions();

    let expected_tile_num = tile_index;
    let actual_tile_num = three_d_to_one_d_cords(
        &GridDimensions::new(tile_x_cord, tile_y_cord, tile_z_cord),
        map_dimensions,
    );
    assert_eq!(expected_tile_num, actual_tile_num);
}

fn main() {
    futures::executor::block_on(GameWorld::run("tests/feature-files/tilemap.feature"));
}

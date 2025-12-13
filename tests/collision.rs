use std::fmt::Debug;

use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use bevy::{prelude::*, sprite::SpritePlugin};

use cucumber::{given, then, when, World};
use helping_hand::map::{
    interactions::map_changing::load_tiled_map, movement::collision::*, render::RenderedMap, *,
};

use std::path::PathBuf;

#[derive(Debug, Default, World)]
#[world(init = Self::new)]
struct GameWorld {
    pub app: App,

    pub map_location: PathBuf,
    pub loaded_map: Tilemap,
    pub bevy_map: RenderedMap,
    pub collision_collection: CollisionCollection,
}

impl GameWorld {
    pub fn new() -> Self {
        let map_location = PathBuf::new();
        let loaded_map = Tilemap::default();
        let bevy_map = RenderedMap::default();
        let collision_collection = CollisionCollection::default();

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
        app.add_plugins(SpritePlugin::default());
        app.add_plugins(ImagePlugin::default());

        Self {
            app,
            map_location,
            loaded_map,
            bevy_map,
            collision_collection,
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

pub fn three_d_to_one_d_cords(
    tile_grid_cords: &GridDimensions,
    map_grid_dimensions: &GridDimensions,
) -> u32 {
    let map_area = map_grid_dimensions.get_columns() * map_grid_dimensions.get_rows();
    let map_length = map_grid_dimensions.get_columns();
    let tile_x = tile_grid_cords.get_columns();
    let tile_y = tile_grid_cords.get_rows();
    let tile_z = tile_grid_cords.get_layers();

    (map_area * tile_z) + (map_length * tile_y) + tile_x
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

#[when("the Tiled map is loaded,")]
fn load_test_map(world: &mut GameWorld) {
    let tiled_map = load_tiled_map(world.map_location.clone());
    world.loaded_map = Tilemap::from_tiled(&tiled_map);
}

#[when("the Tiled map has been converted to a rendered map,")]
fn tiled_map_to_bevy_tiles(world: &mut GameWorld) {
    let tilemap = &world.loaded_map;
    let asset_server = world.app.world().resource::<AssetServer>().clone();
    let mut texture_atlas_layout = world
        .app
        .world_mut()
        .resource_mut::<Assets<TextureAtlasLayout>>();

    let rendered_bevy_map = RenderedMap::new(tilemap, &asset_server, &mut texture_atlas_layout);
    world.bevy_map = rendered_bevy_map;
}

#[when("the collision tiles are collected,")]
fn collision_tiles_are_collected(world: &mut GameWorld) {
    let collision_collection = create_collision_collection_from(&world.bevy_map);
    world.collision_collection = collision_collection;
}

#[then(regex = r"there are ([0-9]+) collision tiles in the rendered map.")]
fn verify_number_of_collision_tiles_on_rendered_map(
    world: &mut GameWorld,
    expected_collision_tile_amount: usize,
) {
    let actual_collision_tile_amount = world
        .bevy_map
        .get_bevy_tiles()
        .iter()
        .filter(|tile| tile.get_tile_type() == &TileType::Collision)
        .count();

    assert_eq!(expected_collision_tile_amount, actual_collision_tile_amount);
}

#[then(regex = r"there are (\d+) layers in the rendered map.")]
fn verify_num_layers_in_rendered_map(world: &mut GameWorld, expected_num_layers: usize) {
    let actual_num_layers = world.bevy_map.get_grid_dimensions().get_layers() as usize;
    assert_eq!(expected_num_layers, actual_num_layers);
}

#[then(regex = r"rendered tile ([0-9]+),([0-9]+),([0-9]+) is invisible.")]
fn verify_tile_is_invisible(
    world: &mut GameWorld,
    tile_x_cord: usize,
    tile_y_cord: usize,
    tile_z_cord: usize,
) {
    let expected_tile_coordinates = GridCords::new(tile_x_cord, tile_y_cord, tile_z_cord);

    let tile_is_invisible = world.bevy_map.get_bevy_tiles().iter().any(|tile| {
        tile.get_grid_coordinates() == &expected_tile_coordinates && tile.is_invisible()
    });
    assert!(tile_is_invisible);
}

#[then(regex = r"rendered tile ([0-9]+),([0-9]+),([0-9]+) is labeled as a collision tile.")]
fn verify_tile_is_labeled_collision_tile(
    world: &mut GameWorld,
    tile_x_cord: usize,
    tile_y_cord: usize,
    tile_z_cord: usize,
) {
    let expected_tile_coordinates = GridCords::new(tile_x_cord, tile_y_cord, tile_z_cord);

    let tile_is_collision = world.bevy_map.get_bevy_tiles().iter().any(|tile| {
        tile.get_tile_type() == &TileType::Collision
            && tile.get_grid_coordinates() == &expected_tile_coordinates
    });
    assert!(tile_is_collision);
}

#[then(regex = r"tile ([0-9]+),([0-9]+),([0-9]+) is a collision tile in the collection.")]
fn verify_tile_is_in_collision_collection(
    world: &mut GameWorld,
    tile_x_cord: u32,
    tile_y_cord: u32,
    tile_z_cord: usize,
) {
    let tile_xyz_coords = GridCords::new_u32(tile_x_cord, tile_y_cord, tile_z_cord);
    let tile_is_in_collision_collection = world.collision_collection.has(&tile_xyz_coords);
    assert!(tile_is_in_collision_collection);
}

fn main() {
    futures::executor::block_on(GameWorld::run(
        "tests/feature-files/in-theory/collision.feature",
    ));
}

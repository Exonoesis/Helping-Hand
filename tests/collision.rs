use std::fmt::Debug;

use bevy::prelude::*;
use bevy::render::settings::WgpuSettings;
use bevy::render::RenderPlugin;
use bevy::sprite::SpritePlugin;

use helping_hand::visuals::map::*;

use cucumber::{given, then, when, World};

use std::path::PathBuf;

#[derive(Debug, Default, World)]
#[world(init = Self::new)]
struct GameWorld {
    pub app: App,
    pub map_location: PathBuf,

    pub loaded_map: Tilemap,
    pub bevy_map: RenderedMap,
}

impl GameWorld {
    pub fn new() -> Self {
        let map_location = PathBuf::new();
        let loaded_map = Tilemap::default();
        let bevy_map = RenderedMap::default();

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

#[when("the Tiled map is loaded,")]
fn load_test_map(world: &mut GameWorld) {
    world.loaded_map = Tilemap::new(world.map_location.clone());
}

#[when("the Tiled map has been converted to a rendered map,")]
fn tiled_map_to_bevy_tiles(world: &mut GameWorld) {
    let tilemap = &world.loaded_map;
    let asset_server = world.app.world.resource::<AssetServer>().clone();
    let mut texture_atlas_layout = world.app.world.resource_mut::<Assets<TextureAtlasLayout>>();

    let rendered_bevy_map = RenderedMap::new(tilemap, &asset_server, &mut texture_atlas_layout);
    world.bevy_map = rendered_bevy_map;
}

#[when("the collision tiles are collected,")]
fn collision_tiles_are_collected(world: &mut GameWorld) {
    //TO-DO
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

    assert_eq!(expected_collision_tile_amount, actual_collision_tile_amount)
}

#[then(regex = r"rendered tile ([0-9]+),([0-9]+),([0-9]+) is invisible.")]
fn verify_tile_is_invisible(
    world: &mut GameWorld,
    tile_x_cord: u32,
    tile_y_cord: u32,
    tile_z_cord: u32,
) {
    let tile = GridDimensions::new(tile_x_cord, tile_y_cord, tile_z_cord);
    let tile_index = three_d_to_one_d_cords(&tile, world.loaded_map.get_grid_dimensions()) as usize;

    let tile_is_invisible = world.bevy_map.get_bevy_tiles()[tile_index].is_invisible();
    assert!(tile_is_invisible)
}

fn main() {
    futures::executor::block_on(GameWorld::run("tests/feature-files/collision.feature"));
}

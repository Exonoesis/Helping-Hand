use std::fmt::Debug;

use helping_hand::visuals::map::*;

use cucumber::{given, then, when, World};
use tiled::{Loader, Map};

use std::path::PathBuf;

#[derive(Debug, Default, World)]
#[world(init = Self::new)]
struct GameWorld {
    pub map_location: PathBuf,
    pub loaded_map: Option<Map>,
    pub interactive_collection: InteractiveCollection,
    pub point_of_interest: XyzCords,
    pub found_marker: Option<InteractiveMarker>,
}

impl GameWorld {
    pub fn new() -> Self {
        let map_location = PathBuf::new();
        let loaded_map = None;
        let interactive_collection = InteractiveCollection::default();
        let point_of_interest = XyzCords::new(0, 0, 0);
        let found_marker = None;

        Self {
            map_location,
            loaded_map,
            interactive_collection,
            point_of_interest,
            found_marker,
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

/// Converts a string into a Proximity
fn convert_string_to_proximity(proximity_string: String) -> Proximity {
    let proximity = match proximity_string.as_str() {
        "lower" => Proximity::Lower,
        "higher" => Proximity::Higher,
        "match" => Proximity::Match,
        _ => panic!(
            "convert_string_to_proximity: Invalid proximity given: {}",
            proximity_string
        ),
    };

    proximity
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

#[given(regex = r"a position of ([0-9]+),([0-9]+),")]
fn set_point_of_interest(world: &mut GameWorld, x_cord: usize, y_cord: usize) {
    world.point_of_interest = XyzCords::new(x_cord, y_cord, 0);
}

#[when("the Tiled map is loaded,")]
fn load_test_map(world: &mut GameWorld) {
    let mut loader = Loader::new();
    let tiled_map = loader.load_tmx_map(world.map_location.clone()).unwrap();
    world.loaded_map = Some(tiled_map);
}

#[when("an Interactive Collection is extracted from the Tiled map,")]
fn interactive_tiles_are_collected(world: &mut GameWorld) {
    let interactive_collection =
        create_interactive_collection_from(&world.loaded_map.as_ref().unwrap());
    world.interactive_collection = interactive_collection;
}

#[when("a marker is requested for the position,")]
fn request_marker_from_position(world: &mut GameWorld) {
    let interactive_collection = &world.interactive_collection;
    let position = world.point_of_interest;
    world.found_marker = interactive_collection.get_marker_from_position(&position);
}

#[then(regex = r"there (?:is|are) ([0-9]+) interactive marker(?:s)? in the collection.")]
fn verify_number_of_interactive_markers_in_collection(
    world: &mut GameWorld,
    expected_interactive_marker_amount: usize,
) {
    let actual_interactive_marker_amount = world.interactive_collection.len();

    assert_eq!(
        expected_interactive_marker_amount,
        actual_interactive_marker_amount
    );
}

#[then(regex = r"marker ([0-9]+) has a position of ([0-9]+),([0-9]+),([0-9]+).")]
fn verify_marker_position(
    world: &mut GameWorld,
    index: usize,
    x_cord: usize,
    y_cord: usize,
    z_cord: usize,
) {
    let marker = world.interactive_collection.get_marker_at_index(index - 1);
    let actual_position = marker.get_position();
    let expected_position = XyzCords::new(x_cord, y_cord, z_cord);
    assert_eq!(expected_position, actual_position);
}

#[then(regex = r"marker ([0-9]+) has a size of ([0-9]+)x([0-9]+).")]
fn verify_marker_size(world: &mut GameWorld, index: usize, width: u32, height: u32) {
    let marker = world.interactive_collection.get_marker_at_index(index - 1);
    let actual_size = marker.get_dimensions();
    let expected_size = PxDimensions::new(width, height);
    assert_eq!(expected_size, actual_size);
}

#[then(regex = r"the position reports ([a-zA-Z]+) on the marker.")]
fn verify_marker_contains_position(world: &mut GameWorld, proximity: String) {
    let position_of_interest = world.point_of_interest;
    let marker = world.interactive_collection.get_marker_at_index(0);

    let actual_proximity = marker.containing(&position_of_interest);
    let expected_proximity = convert_string_to_proximity(proximity);
    assert_eq!(expected_proximity, actual_proximity);
}

#[then("the marker has a position of 64,0,2.")]
fn verify_found_marker_positon(world: &mut GameWorld) {
    let actual_position = world.found_marker.unwrap().get_position();
    let expected_position = XyzCords::new(64, 0, 2);
    assert_eq!(expected_position, actual_position);
}

#[then("the marker has a size of 64x64.")]
fn verify_found_marker_dimensions(world: &mut GameWorld) {
    let actual_dimensions = world.found_marker.unwrap().get_dimensions();
    let expected_dimensions = PxDimensions::new(64, 64);
    assert_eq!(expected_dimensions, actual_dimensions);
}

#[then("it reported there is no marker.")]
fn verify_no_marker_at_position(world: &mut GameWorld) {
    let found_marker = world.found_marker;
    assert!(found_marker.is_none())
}

fn main() {
    futures::executor::block_on(GameWorld::run("tests/feature-files/interactives.feature"));
}

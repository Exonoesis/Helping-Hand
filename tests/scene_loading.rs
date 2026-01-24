mod mock_game;

use crate::mock_game::Game;
use bevy::color::Alpha;
use cucumber::{given, then, when, World};

use bevy::prelude::ImageNode;
use helping_hand::map::GridCords2D;
use helping_hand::narrative::act_loading::*;
use helping_hand::narrative::acts::*;
use helping_hand::plugins::acts::CoreActsPlugin;
use std::path::PathBuf;
use std::time::Duration;

#[given("the game is capable of handling acts,")]
fn add_acts_plugin(game: &mut Game) {
    let fade_duration = Duration::from_secs(0);
    game.add_plugin(CoreActsPlugin::new(fade_duration));
}

#[when(regex = r"the act called '(.+)' is loaded,")]
fn load_act(game: &mut Game, act_file_name: String) {
    let act_file_path_name = format!("assets/acts/{}", act_file_name);
    let act_file_path = PathBuf::from(&act_file_path_name);

    assert!(
        act_file_path.exists(),
        "Act file does not exist at location {:?}",
        act_file_path.canonicalize().unwrap()
    );

    game.broadcast_event(LoadAct::new(&act_file_path_name));

    // Since we're manually broadcasting an event, we MUST manually tick for the act to be visible
    game.tick();
}

#[when("the game transitions to the next scene,")]
fn transition_to_next_scene(game: &mut Game) {
    game.broadcast_event(LoadNextScene::new());

    // Since we're manually broadcasting an event, we MUST manually tick for the next scene to be visible
    game.tick();
}

#[when(regex = r"the game transitions to scene ([0-9]+),")]
fn transition_to_given_scene(game: &mut Game, given_scene_num: usize) {
    for _ in 0..(given_scene_num - 1) {
        game.broadcast_event(LoadNextScene::new());
        // Since we're manually broadcasting an event, we MUST manually tick each time for the next scene to be visible
        game.tick();
    }
}

#[then(regex = r"the title of the current scene loaded is called '(.+)'.")]
fn verify_current_scene_title(game: &mut Game, expected_scene_title: String) {
    let current_act = game.get_mut::<Act>();
    let current_scene = current_act.get_current_scene();
    let actual_scene_title = current_scene.get_title();

    assert_eq!(expected_scene_title, actual_scene_title);
}

#[then(regex = r"the image at '(.+)' is displayed on the screen.")]
fn verify_image_loaded(game: &mut Game, expected_image_path: String) {
    game.tick();

    let image_node = game.get_mut::<ImageNode>();
    let image_path = image_node.image.path().unwrap().path().to_str().unwrap();
    let actual_image_path = image_path.to_string();

    assert_eq!(expected_image_path, actual_image_path);
}

#[then("there is only one image loaded.")]
fn verify_num_images_loaded(game: &mut Game) {
    let image_count = game.get_number_of::<ImageNode>();
    assert_eq!(image_count, 1);
}

#[then("the loaded image's opacity is 100%.")]
fn verify_image_opacity(game: &mut Game) {
    let image_node = game.get_mut::<ImageNode>();
    let opacity = image_node.color.alpha();

    // Value is normalized | [1.0 = 100%]
    assert_eq!(1.0, opacity);
}

#[then(regex = r"the map size should be ([0-9]+) x ([0-9]+) tiles.")]
fn verify_map_size(game: &mut Game, expected_map_width: u32, expected_map_height: u32) {
    let map_dimensions = game.get_map_size();

    let actual_map_height = map_dimensions.get_rows();
    let actual_map_width = map_dimensions.get_columns();

    assert_eq!(expected_map_height, actual_map_height);
    assert_eq!(expected_map_width, actual_map_width);
}

#[then(regex = r"there is a location called '(.+)' at tile ([0-9]+), ([0-9]+).")]
fn verify_location_at_tile(game: &mut Game, location_name: String, tile_x: usize, tile_y: usize) {
    let current_act = game.get_mut::<Act>();
    let current_scene = current_act.get_current_scene();
    let actual_tile_cords = current_scene
        .get_scene_contents()
        .get_location_by_name(location_name)
        .get_cords();

    let expected_tile_cords = GridCords2D::new(tile_x, tile_y);

    assert_eq!(expected_tile_cords, *actual_tile_cords);
}

// This runs before everything else, so you can setup things here.
fn main() {
    futures::executor::block_on(Game::run(
        "tests/feature-files/in-practice/scene_loading.feature",
    ));
}

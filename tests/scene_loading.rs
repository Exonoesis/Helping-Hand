mod mock_game;

use crate::mock_game::Game;
use cucumber::{given, then, when, World};

use bevy::prelude::ImageNode;
use helping_hand::narrative::act_loading::*;
use helping_hand::narrative::acts::*;
use helping_hand::plugins::acts::MockActsPlugin;
use std::path::PathBuf;

#[given("the game is capable of handling acts,")]
fn add_acts_plugin(game: &mut Game) {
    game.add_plugin(MockActsPlugin);
}

#[when(regex = r"the act called '(.+)' is loaded,")]
fn load_act(game: &mut Game, act_file_name: String) {
    let act_file_path_name = format!("tests/test-assets/acts/{}", act_file_name);
    let act_file_path = PathBuf::from(&act_file_path_name);

    assert!(
        act_file_path.exists(),
        "Act file does not exist at location {:?}",
        act_file_path.canonicalize().unwrap()
    );

    game.broadcast_event(LoadAct::new(&act_file_path_name));
}

#[when("the game transitions to the next scene,")]
fn transition_to_next_scene(game: &mut Game) {
    game.broadcast_event(LoadNextScene::new());
}

#[when("a request is made to fade the scene,")]
fn fade_to_next_scene(game: &mut Game) {
    let current_act = game.get_mut::<Act>();
    let current_scene = current_act.get_current_scene().clone();
    game.broadcast_event(SceneFade::new(current_scene));
}

#[when("the fade timer has elapsed,")]
fn fade_tick_for(game: &mut Game) {
    // Ticks 15 times (Q: Should this read fade duration from somewhere?)
    for _ in 0..15 {
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
    // Get all image nodes in the scene and ensure there is only one

    let image_count = game.get_number_of::<ImageNode>();
    assert_eq!(image_count, 1);
}

// This runs before everything else, so you can setup things here.
fn main() {
    futures::executor::block_on(Game::run(
        "tests/feature-files/in-practice/scene_loading.feature",
    ));
}

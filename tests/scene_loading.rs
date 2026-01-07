mod mock_game;

use crate::mock_game::Game;
use bevy::color::Alpha;
use cucumber::{given, then, when, World};

use bevy::prelude::ImageNode;
use helping_hand::map::GridCords2D;
use helping_hand::map::GridCords3D;
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
}

#[when("the game transitions to the next scene,")]
fn transition_to_next_scene(game: &mut Game) {
    game.broadcast_event(LoadNextScene::new());
}

#[when("the fade timer has elapsed,")]
fn fade_tick_for(game: &mut Game) {
    let mut fade_timer_num = game.get_number_of::<FadeTimer>();

    assert_eq!(1, fade_timer_num);

    // 15 ticks should be plenty since duration is set to 0 seconds
    for _ in 0..15 {
        game.tick();
    }

    fade_timer_num = game.get_number_of::<FadeTimer>();

    assert_eq!(0, fade_timer_num)
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

#[then(regex = r"there is a location called '(.+)' at tile ([0-9]+), ([0-9]+).")]
fn verify_location_at_tile(game: &mut Game, location_name: String, tile_x: usize, tile_y: usize) {
    let current_act = game.get_mut::<Act>();
    let current_scene = current_act.get_current_scene();
    let actual_tile_cords = current_scene
        .get_scene_contents()
        .get_location_by_name(location_name)
        .get_cords();

    let expected_tile_cords = GridCords2D::new(tile_x, tile_y);

    assert_eq!(expected_tile_cords, actual_tile_cords);
}

// This runs before everything else, so you can setup things here.
fn main() {
    futures::executor::block_on(Game::run(
        "tests/feature-files/in-practice/scene_loading.feature",
    ));
}

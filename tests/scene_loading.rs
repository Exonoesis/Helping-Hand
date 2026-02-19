mod mock_game;

use crate::mock_game::Game;
use bevy::color::Alpha;
use cucumber::{given, then, when, World};

use bevy::prelude::ImageNode;
use helping_hand::map::GridCords2D;
use helping_hand::narrative::act_loading::*;
use helping_hand::narrative::acts::*;
use helping_hand::plugins::acts::CoreActsPlugin;
use helping_hand::plugins::levels::CoreLevelsPlugin;
use std::path::PathBuf;
use std::time::Duration;

fn get_all_instructions(scene_contents: &SceneContents) -> Vec<MapInstruction> {
    let mut all_instructions = Vec::new();

    let map_actions = scene_contents.get_map_actions();
    for action in map_actions {
        let instructions = action.get_instructions();
        for instruction in instructions {
            all_instructions.push(instruction.clone())
        }
    }

    all_instructions
}

fn check_placement_data(
    instructions: Vec<MapInstruction>,
    expected_character_name: String,
    path_or_location_name: String,
) -> bool {
    for instruction in instructions {
        match instruction {
            MapInstruction::Place(character, found_location)
                if *character.get_name() == expected_character_name
                    && *found_location.get_name() == path_or_location_name =>
            {
                return true
            }
            _ => {}
        }
    }

    false
}

fn check_line_path_data(
    instructions: Vec<MapInstruction>,
    expected_character_name: String,
    path_or_location_name: String,
) -> bool {
    for instruction in instructions {
        match instruction {
            MapInstruction::Move(found_character, found_map_path)
                if *found_character.get_name() == expected_character_name
                    && *found_map_path.get_name() == path_or_location_name =>
            {
                return true
            }
            _ => {}
        }
    }

    false
}

fn check_looping_path_data(
    instructions: Vec<MapInstruction>,
    expected_character_name: String,
    path_or_location_name: String,
) -> bool {
    for instruction in instructions {
        match instruction {
            MapInstruction::Loop(found_character, found_map_path)
                if *found_character.get_name() == expected_character_name
                    && *found_map_path.get_name() == path_or_location_name =>
            {
                return true
            }
            _ => {}
        }
    }

    false
}

fn get_location_by_name(instructions: Vec<MapInstruction>, location_name: String) -> GridCords2D {
    for instruction in instructions {
        if let MapInstruction::Place(_, found_location) = instruction {
            if found_location.get_name() == &location_name {
                return found_location.get_cords().clone();
            }
        }
    }
    panic!("Location {} not found", location_name)
}

fn get_path_by_name(instructions: Vec<MapInstruction>, path_name: String) -> Vec<GridCords2D> {
    for instruction in instructions {
        if let MapInstruction::Move(_, found_path) = instruction {
            if found_path.get_name() == &path_name {
                return found_path.get_path().clone();
            }
        }
    }
    panic!("Path {} not found", path_name)
}

#[given("the game is capable of handling acts,")]
fn add_acts_plugin(game: &mut Game) {
    let fade_duration = Duration::from_secs(0);
    game.add_plugin(CoreActsPlugin::new(fade_duration));
    game.add_plugin(CoreLevelsPlugin);
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

#[when(regex = r"the testing act called '(.+)' is loaded,")]
fn load_testing_act(game: &mut Game, act_file_name: String) {
    let act_file_path_name = format!("tests/test-assets/acts/{}", act_file_name);
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
    game.tick();

    let map_dimensions = game.get_map_size();

    let actual_map_height = map_dimensions.get_rows();
    let actual_map_width = map_dimensions.get_columns();

    assert_eq!(expected_map_height, actual_map_height);
    assert_eq!(expected_map_width, actual_map_width);
}

#[then(regex = r"the character '(.+)' will be (.+) '(.+)'.")]
fn verify_instruction_data(
    game: &mut Game,
    expected_character_name: String,
    type_of_path: String,
    path_or_location_name: String,
) {
    let current_act = game.get_mut::<Act>();
    let current_scene = current_act.get_current_scene();
    let scene_contents = current_scene.get_scene_contents();

    let mut character_and_location_or_path_found = false;

    let instructions = get_all_instructions(scene_contents);

    match type_of_path.as_str() {
        "placed at location" => {
            character_and_location_or_path_found =
                check_placement_data(instructions, expected_character_name, path_or_location_name)
        }
        "moved along the line path" => {
            character_and_location_or_path_found =
                check_line_path_data(instructions, expected_character_name, path_or_location_name)
        }
        "moved along the looping path" => {
            character_and_location_or_path_found = check_looping_path_data(
                instructions,
                expected_character_name,
                path_or_location_name,
            )
        }
        _ => {}
    }

    // We cannot directly compare MapInstructions as there is data
    // outside the scope of this test needed to create one
    assert!(character_and_location_or_path_found);
}

#[then(regex = r"the location '(.+)' is at tile ([0-9]+), ([0-9]+).")]
fn verify_location_at_tile(game: &mut Game, location_name: String, tile_x: usize, tile_y: usize) {
    let current_act = game.get_mut::<Act>();
    let current_scene = current_act.get_current_scene();
    let scene_contents = current_scene.get_scene_contents();

    let instructions = get_all_instructions(scene_contents);

    let actual_tile_cords = get_location_by_name(instructions, location_name);
    let expected_tile_cords = GridCords2D::new(tile_x, tile_y);

    assert_eq!(expected_tile_cords, actual_tile_cords);
}

#[then(regex = r"the line path '(.+)' has a path length of ([0-9]+) tiles.")]
fn verify_path_length(game: &mut Game, path_name: String, expected_path_length: usize) {
    let current_act = game.get_mut::<Act>();
    let current_scene = current_act.get_current_scene();
    let scene_contents = current_scene.get_scene_contents();

    let instructions = get_all_instructions(scene_contents);

    let actual_path_length = get_path_by_name(instructions, path_name).len();

    assert_eq!(expected_path_length, actual_path_length);
}

#[then(regex = r"tile ([0-9]+) of line path '(.+)' is tile ([0-9]+), ([0-9]+).")]
fn verify_path_tile_cords(
    game: &mut Game,
    tile_index: usize,
    path_name: String,
    expected_x: usize,
    expected_y: usize,
) {
    let current_act = game.get_mut::<Act>();
    let current_scene = current_act.get_current_scene();
    let scene_contents = current_scene.get_scene_contents();

    let instructions = get_all_instructions(scene_contents);

    let actual_path_tile = get_path_by_name(instructions, path_name);
    let actual_path_tile_cords = actual_path_tile[tile_index - 1].clone();

    let expected_path_tile_cords = GridCords2D::new(expected_x, expected_y);

    assert_eq!(expected_path_tile_cords, actual_path_tile_cords);
}

// This runs before everything else, so you can setup things here.
fn main() {
    futures::executor::block_on(Game::run(
        "tests/feature-files/in-practice/scene_loading.feature",
    ));
}

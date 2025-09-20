use cucumber::{given, then, when, World};
use helping_hand::narrative::acts::*;
use std::path::PathBuf;

#[derive(Debug, World)]
#[world(init = Self::new)]
struct GameWorld {
    pub act_file: PathBuf,
    pub current_act: Act,
}

impl GameWorld {
    pub fn new() -> Self {
        let act_file = PathBuf::new();
        let current_act = Act::new();

        Self {
            act_file,
            current_act,
        }
    }
}

// Returns a Path to the specified Act File
// for a testing environment.
fn get_act_file_location(act_file_name: String) -> PathBuf {
    let mut act_file_path = PathBuf::new();

    // Appends the Manifest Directory which represents the root of the whole project.
    // We need this since we cannot use relative paths for testing purposes.
    if let Ok(project_root) = std::env::var("CARGO_MANIFEST_DIR") {
        act_file_path.push(project_root);
    }

    act_file_path.push("assets/acts");
    act_file_path.push(act_file_name);

    act_file_path
}

#[given(regex = r"an act file called (.+\.json),")]
fn given_some_act_file_name(game: &mut GameWorld, file_name: String) {
    let act_file_path = get_act_file_location(file_name);

    assert!(
        act_file_path.exists(),
        "File does not exist at location {:?}",
        act_file_path.canonicalize().unwrap()
    );

    game.act_file = act_file_path;
}

#[when("the act is read from the act file,")]
fn read_act_file(game: &mut GameWorld) {
    let act_file = game.act_file.clone();
    game.current_act = read_act_from(act_file);
}

#[when("we move to the next scene,")]
fn go_to_next_scene(game: &mut GameWorld) {
    game.current_act.move_to_next_scene();
}

#[then(regex = r"the current scene is '(.+)'.")]
fn verify_current_scene(game: &mut GameWorld, expected_scene_title: String) {
    let actual_scene_title = game.current_act.get_current_scene().get_title();

    assert_eq!(expected_scene_title, actual_scene_title);
}

#[then(regex = r"the scene with the title '(.+)' is scene ([0-9]+) in the current act.")]
fn verify_scene_index_by_title(
    game: &mut GameWorld,
    given_scene_title: String,
    scene_index: usize,
) {
    let expected_index = scene_index - 1;
    let current_scene = game.current_act.get_scene_by_title(&given_scene_title);
    let actual_index = game.current_act.get_scene_idx(&current_scene);

    assert_eq!(expected_index, actual_index);
}

#[then(
    regex = r"the act's scene called '(.+)' is an Image Cutscene pointing to the image (.+\.png)."
)]
fn verify_image_cutscene(game: &mut GameWorld, scene_title: String, image_path: String) {
    let act = &game.current_act;

    let actual_scene = act.get_scene_by_title(&scene_title);
    let actual_contents = actual_scene.get_scene_contents();

    let actual_scene_type = actual_scene.get_scene_type();
    let actual_image_path = actual_contents.get_image_path();

    let expected_scene_type = SceneType::ImageCutscene;
    let expected_image_path = PathBuf::from(image_path);

    assert_eq!(*actual_scene_type, expected_scene_type);
    assert_eq!(*actual_image_path, expected_image_path);
}

#[then(
    regex = r"the act's scene called '(.+)' is a Map Cutscene pointing to the map file called (.+\.tmx)."
)]
fn verify_map_cutscene(game: &mut GameWorld, scene_title: String, map_file_path: String) {
    let act = &game.current_act;

    let actual_scene = act.get_scene_by_title(&scene_title);
    let actual_contents = actual_scene.get_scene_contents();

    let actual_scene_type = actual_scene.get_scene_type();
    let actual_map_path = actual_contents.get_map_path();

    let expected_scene_type = SceneType::MapCutscene;
    let expected_map_path = PathBuf::from(map_file_path);

    assert_eq!(*actual_scene_type, expected_scene_type);
    assert_eq!(*actual_map_path, expected_map_path);
}

#[then(regex = r"scene '(.+)' should connect to scene '(.+)'.")]
fn verify_scene_connection_exists(
    game: &mut GameWorld,
    starting_scene_title: String,
    connected_scene_title: String,
) {
    let act = &game.current_act;
    let starting_scene = act.get_scene_by_title(&starting_scene_title);
    let scene_connections = act.get_scene_connections(starting_scene);
    let connection_has_expected_scene = scene_connections
        .iter()
        .any(|scene| scene.get_title() == connected_scene_title);

    assert!(connection_has_expected_scene);
}

fn main() {
    futures::executor::block_on(GameWorld::run("tests/feature-files/in-theory/acts.feature"));
}

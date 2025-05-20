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
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        act_file_path.push(manifest_dir);
    }

    act_file_path.push("tests/test-assets/acts");
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

#[then(regex = r"the scene with the title '(.+)' is scene ([0-9]+) in the current act.")]
fn verify_scene_index_by_title(game: &mut GameWorld, scene_title: String, scene_index: usize) {
    let expected_index = scene_index - 1;
    let given_title = scene_title;
    let current_scene = game.current_act.get_scene_by_title(&given_title);
    let actual_index = game.current_act.get_scene_idx(&current_scene);

    assert_eq!(expected_index, actual_index);
}

#[then(
    regex = r"the act's scene called '(.+)' is an Image Cutscene pointing to the image (.+\.png)."
)]
fn verify_image_cutscene(game: &mut GameWorld, scene_title: String, image_path: String) {
    let act = &game.current_act;

    let actual_scene = act.get_scene_by_title(&scene_title);

    let given_path = PathBuf::from(image_path);
    let scene_contents = SceneContents::ImageCutscene(given_path);
    let expected_scene = Scene::make_scene(scene_title, scene_contents);

    assert_eq!(expected_scene, *actual_scene);
}

#[then(regex = r"scene '(.+)' should connect to scene '(.+)'.")]
fn verify_scene_connection_exists(
    game: &mut GameWorld,
    scene_title_1: String,
    scene_title_2: String,
) {
    let act = &game.current_act;
    let scene_to_check = act.get_scene_by_title(&scene_title_1);
    let list_of_connections = act.get_scene_connections(scene_to_check);

    let expected_scene = act.get_scene_by_title(&scene_title_2);
    let expected_connection = act.get_scene_idx(expected_scene);

    assert!(list_of_connections.contains(&expected_connection))
}

fn main() {
    futures::executor::block_on(GameWorld::run("tests/feature-files/acts.feature"));
}

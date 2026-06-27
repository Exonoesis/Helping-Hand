mod mock_game;

use std::{path::PathBuf, time::Duration};

use crate::mock_game::Game;

use cucumber::{given, then, when, World};
use helping_hand::{
    narrative::act_loading::LoadAct,
    plugins::{acts::CoreActsPlugin, levels::CoreLevelsPlugin},
    AppState,
};

#[given(regex = r"the game is loaded with the act '(.+)',")]
fn load_plugin_and_act(game: &mut Game, act_file_name: String) {
    let fade_duration = Duration::from_secs(0);
    let maps_folder_path = PathBuf::from("tests/test_assets/maps/npc_movement/");

    game.add_plugin(CoreActsPlugin::new(fade_duration, maps_folder_path));
    game.add_plugin(CoreLevelsPlugin);

    let act_file_path_name = format!("tests/test_assets/acts/{}", act_file_name);
    let act_file_path = PathBuf::from(&act_file_path_name);

    assert!(
        act_file_path.exists(),
        "Act file does not exist at location {:?}",
        act_file_path.canonicalize().unwrap()
    );

    game.write_message(LoadAct::new(&act_file_path_name));
    game.tick();

    game.set_state(AppState::Transitioning);

    while game.get_state() == &AppState::Transitioning {
        game.tick();
    }
}

// This runs before everything else, so you can setup things here.
fn main() {
    futures::executor::block_on(Game::run(
        "tests/feature_files/in-practice/npc_movement.feature",
    ));
}

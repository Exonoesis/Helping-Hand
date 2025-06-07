use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

use helping_hand::{plugins, AppState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build())
        .init_state::<AppState>()
        .add_plugins(AudioPlugin)
        .add_plugins(plugins::acts::ActsPlugin)
        .add_plugins(plugins::levels::LevelsPlugin)
        .add_plugins(plugins::playable_character::PlayableCharacterPlugin)
        .add_plugins(plugins::main_menu::MainMenuPlugin)
        .add_plugins(plugins::settings_menu::SettingsMenuPlugin)
        .run();
}

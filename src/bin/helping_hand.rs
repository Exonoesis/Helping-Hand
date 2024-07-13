use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::AudioPlugin;

use helping_hand::{plugins, AppState};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                // An explanation for this line can be found in the referencing bevy example:
                // https://github.com/bevyengine/bevy/blob/main/examples/asset/custom_asset_io.rs#L69
                .add_before::<bevy::asset::AssetPlugin, _>(
                    plugins::smart_asset_io::SmartAssetReaderPlugin,
                ),
        )
        .init_state::<AppState>()
        .add_plugins(LdtkPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(plugins::levels::LevelsPlugin)
        .add_plugins(plugins::playable_character::PlayableCharacterPlugin)
        .add_plugins(plugins::main_menu::MainMenuPlugin)
        .add_plugins(plugins::settings_menu::SettingsMenuPlugin)
        .run();
}

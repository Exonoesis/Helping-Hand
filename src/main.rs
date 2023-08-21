mod audio;
mod diagnostics;
mod entities;
mod mechanics;
mod plugins;
mod visuals;

use std::time::Duration;

use bevy::{asset::ChangeWatcher, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::AudioPlugin;
use plugins::smart_asset_io::SmartAssetIoPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    //For dev purposes only. REMOVE WHEN GIVING TO PLAYERS!
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                })
                .build()
                // An explanation for this line can be found in the referencing bevy example:
                // https://github.com/bevyengine/bevy/blob/main/examples/asset/custom_asset_io.rs#L69
                .add_before::<bevy::asset::AssetPlugin, _>(SmartAssetIoPlugin),
        )
        .add_state::<AppState>()
        .add_plugins(LdtkPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(plugins::levels::LevelsPlugin)
        .add_plugins(plugins::playable_character::PlayableCharacterPlugin)
        .add_plugins(plugins::main_menu::MainMenuPlugin)
        .run();
}

mod audio;
mod diagnostics;
mod entities;
mod mechanics;
mod plugins;
mod visuals;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::AudioPlugin;
use plugins::smart_asset_io::SmartAssetIoPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    //For dev purposes only. REMOVE WHEN GIVING TO PLAYERS!
                    watch_for_changes: true,
                    ..default()
                })
                .build()
                // An explanation for this line can be found in the referencing bevy example:
                // https://github.com/bevyengine/bevy/blob/main/examples/asset/custom_asset_io.rs#L69
                .add_before::<bevy::asset::AssetPlugin, _>(SmartAssetIoPlugin),
        )
        .add_plugin(LdtkPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(plugins::levels::LevelsPlugin)
        .add_plugin(plugins::playable_character::PlayableCharacterPlugin)
        .run();
}

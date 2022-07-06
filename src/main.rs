mod mechanics;
mod audio;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::AudioPlugin;
use mechanics::{camera::move_camera, input::*};
use audio::music::play_level_music;

#[derive(Default, Component)]
pub struct Player;

#[derive(Bundle, LdtkEntity)]
struct PlayerBundle {
    #[sprite_sheet_bundle("textures/characters/EeveeSprites.png", 64.0, 64.0, 1, 4, 0.0, 0)]
    #[bundle]
    sprite: SpriteSheetBundle,

    player: Player,
}

/// Loads the LDtk test map with a Camera into the game at the origin (0,0,0).
fn spawn_map(mut commands: Commands, asset_spawner: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    asset_spawner
        .watch_for_changes()
        .expect("Hot Reloading is not working."); //For dev purposes only. REMOVE WHEN GIVING TO PLAYERS!
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_spawner.load("maps/hh_test.ldtk"),
        //transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(AudioPlugin)
        .add_startup_system(spawn_map)
        .insert_resource(LevelSelection::Identifier("Test_level".to_string()))
        .add_event::<Movement>()
        .add_system(player_input)
        .add_system(move_player)
        .add_system(move_camera)
        .add_system(play_level_music)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .run();
}

mod audio;
mod entities;
mod mechanics;

use audio::{
    music::{play_level_music, MusicChannel},
    sfx::*,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::{AudioApp, AudioPlugin};
use entities::player::{PlayerBumpChannel, PlayerBundle, PlayerMovementActions, PlayerWalkChannel};
use mechanics::{
    camera::{move_camera, update_camera_on_resolution_change},
    input::*,
};

#[derive(Default, Resource)]
pub struct LevelDimensions {
    pub width: usize,
    pub height: usize,
}

/// Loads the LDtk test map with a Camera into the game at the origin (0,0,0).
fn spawn_map(mut commands: Commands, asset_spawner: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_spawner.load("map/hh_world.ldtk"),
        //transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            //For dev purposes only. REMOVE WHEN GIVING TO PLAYERS!
            watch_for_changes: true,
            ..default()
        }))
        .add_plugin(LdtkPlugin)
        .add_plugin(AudioPlugin)
        .add_startup_system(spawn_map)
        .add_startup_system(load_player_movement_sound)
        .add_startup_system(load_player_bump_sound)
        .insert_resource(LevelSelection::Identifier("Level_0".to_string()))
        .init_resource::<LevelDimensions>()
        .add_audio_channel::<MusicChannel>()
        .add_audio_channel::<PlayerWalkChannel>()
        .add_audio_channel::<PlayerBumpChannel>()
        .add_event::<Movement>()
        .add_event::<PlayerMovementActions>()
        .add_system(update_level_dimensions)
        .add_system(player_input)
        .add_system(move_player)
        .add_system(bound_player_movement)
        .add_system(move_camera)
        .add_system(update_camera_on_resolution_change)
        .add_system(play_level_music)
        .add_system(play_player_movement_sound)
        .add_system(play_player_bump_sound)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .run();
}

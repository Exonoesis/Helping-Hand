use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Resource)]
pub struct LevelDimensions {
    pub width: usize,
    pub height: usize,
}

/// Loads the LDtk test map with a Camera into the game at the origin (0,0,0).
pub fn spawn_map(mut commands: Commands, asset_spawner: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_spawner.load("map/hh_world.ldtk"),
        //transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

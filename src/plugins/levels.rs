use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::AudioApp;

use crate::{
    audio::music::{play_level_music, MusicChannel},
    mechanics::{camera::*, input::*},
    visuals::map::*, 
    AppState,
};

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::InGame)
                .with_system(spawn_map)
            )
            .insert_resource(LevelSelection::Identifier("Level_0".to_string()))
            .init_resource::<LevelDimensions>()
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                .with_system(move_camera)
                .with_system(player_input)
                .with_system(play_level_music)
                .with_system(update_level_dimensions)
                .with_system(update_camera_on_resolution_change)
            )
            .add_audio_channel::<MusicChannel>()
            .add_event::<Movement>();
    }
}

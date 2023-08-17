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
        app.add_systems(OnEnter(AppState::InGame), spawn_map)
            .insert_resource(LevelSelection::Identifier("Level_0".to_string()))
            .init_resource::<LevelDimensions>()
            .add_systems(Update(AppState::InGame),
                (
                    move_camera,
                    player_input,
                    play_level_music,
                    update_level_dimensions,
                    update_camera_on_resolution_change,
                ),
            )
            .add_audio_channel::<MusicChannel>()
            .add_event::<Movement>();
    }
}

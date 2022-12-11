use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::AudioApp;

use crate::{
    mechanics::{camera::*, input::*},
    audio::music::{play_level_music, MusicChannel},
    visuals::map::*
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_map)
            .insert_resource(LevelSelection::Identifier("Level_0".to_string()))
            .init_resource::<LevelDimensions>()
            .add_system(move_camera)
            .add_system(player_input)
            .add_system(play_level_music)
            .add_system(update_level_dimensions)
            .add_system(update_camera_on_resolution_change)
            .add_audio_channel::<MusicChannel>()
            .add_event::<Movement>();
    }
}
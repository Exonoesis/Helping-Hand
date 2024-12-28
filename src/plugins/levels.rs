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
        // Loading the map
        app.add_event::<ChangeLevel>()
            .add_systems(
                OnEnter(AppState::InGame),
                load_starting_map.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                (load_map, move_camera, move_player_on_key_press)
                    .run_if(in_state(AppState::InGame)),
            );
        // Interacting with the map
        //.add_systems(
        //    Update,
        //    (
        //        //move_camera,
        //        //player_input,
        //        //play_level_music,
        //        //update_level_dimensions,
        //        //update_camera_on_resolution_change,
        //    )
        //        .run_if(in_state(AppState::InGame)),
        //);
        //.add_audio_channel::<MusicChannel>();
    }
}

pub struct MockLevelsPlugin;

impl Plugin for MockLevelsPlugin {
    fn build(&self, app: &mut App) {
        // Loading the map
        app.add_event::<ChangeLevel>().add_systems(
            Update,
            (load_map, move_camera, move_player_on_key_press).run_if(in_state(AppState::InGame)),
        );
    }
}

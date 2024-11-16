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
        app.init_resource::<LevelDimensions>()
            // Loading the map
            .add_event::<ChangeLevel>()
            .add_systems(Update, load_map.run_if(in_state(AppState::InGame)));
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

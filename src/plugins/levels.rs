use bevy::prelude::*;
use bevy_kira_audio::AudioApp;

use crate::{
    audio::music::{play_level_music, MusicChannel},
    map::{
        interactions::{interactives::*, map_changing::*},
        movement::grid_based_movement::*,
        player::PlayerInteraction,
    },
    ui::*,
    AppState,
};

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        // Loading the map
        app.add_plugins(CoreLevelsPlugin)
            .add_systems(
                Update,
                (play_level_music, interact_entity).run_if(in_state(AppState::InGame)),
            )
            .add_audio_channel::<MusicChannel>();
    }
}

pub struct CoreLevelsPlugin;

impl Plugin for CoreLevelsPlugin {
    fn build(&self, app: &mut App) {
        // Loading the map
        app.add_event::<LoadLevel>()
            .add_event::<ChangeLevel>()
            .add_event::<PlayerInteraction>()
            .add_systems(
                Update,
                (
                    load_map,
                    change_to_new_level,
                    follow_player.after(move_player_on_key_press),
                    move_player_on_key_press,
                    change_level_from_marker,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

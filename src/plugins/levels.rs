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
        app.add_event::<ChangeLevel>()
            .add_event::<PlayerInteraction>()
            .add_systems(
                Update,
                (
                    load_map,
                    follow_player,
                    move_player_on_key_press,
                    play_level_music,
                    change_level_from_marker,
                    interact_entity,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_audio_channel::<MusicChannel>();
    }
}

pub struct MockLevelsPlugin;

impl Plugin for MockLevelsPlugin {
    fn build(&self, app: &mut App) {
        // Loading the map
        app.add_event::<ChangeLevel>()
            .add_event::<PlayerInteraction>()
            .add_systems(
                Update,
                (
                    load_map,
                    follow_player.after(move_player_on_key_press),
                    move_player_on_key_press,
                    change_level_from_marker,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::AudioApp;

use crate::{
    audio::sfx::*,
    map::{movement::grid_based_movement::*, player::*},
    AppState,
};

pub struct PlayableCharacterPlugin;

impl Plugin for PlayableCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementDirection>();
        app.insert_resource(ArrivalTime::new(Duration::from_secs_f32(0.15)));

        app.add_systems(
            Update,
            (set_player_target, move_entity_to_target).run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (
                play_player_movement_sound.after(set_player_target),
                play_player_bump_sound.after(set_player_target),
            ),
        )
        .add_systems(
            OnEnter(AppState::InGame),
            (load_player_movement_sound, load_player_bump_sound),
        )
        .add_audio_channel::<PlayerWalkChannel>()
        .add_audio_channel::<PlayerBumpChannel>()
        .add_event::<PlayerMovementActions>();
    }
}

pub struct PlayableCharacterTestingPlugin;

impl Plugin for PlayableCharacterTestingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementDirection>();
        app.insert_resource(ArrivalTime::new(Duration::from_secs_f32(0.15)));

        app.add_systems(
            Update,
            (set_player_target, move_entity_to_target).run_if(in_state(AppState::InGame)),
        )
        .add_event::<PlayerMovementActions>();
    }
}

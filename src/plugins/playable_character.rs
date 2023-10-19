use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::AudioApp;

use crate::{audio::sfx::*, entities::player::*, mechanics::input::*, AppState};

pub struct PlayableCharacterPlugin;

impl Plugin for PlayableCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (load_player_movement_sound, load_player_bump_sound),
        )
        .add_systems(
            Update,
            (
                move_entity,
                animate_entity,
                interact_entity,
                display_interactive_message,
                transition_level,
                bound_player_movement,
                play_player_movement_sound,
                play_player_bump_sound,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_audio_channel::<PlayerWalkChannel>()
        .add_audio_channel::<PlayerBumpChannel>()
        .add_event::<PlayerMovementActions>()
        .add_event::<InteractionEvent>()
        .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

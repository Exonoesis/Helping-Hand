use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::AudioApp;

use crate::{audio::sfx::*, entities::player::*, mechanics::input::*, AppState};

pub struct PlayableCharacterPlugin;

impl Plugin for PlayableCharacterPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(
            OnEnter(AppState::InGame),
            (load_player_movement_sound, load_player_bump_sound),
        )
        .add_systems(Update(AppState::InGame),
            (
                move_player,
                bound_player_movement,
                play_player_movement_sound,
                play_player_bump_sound
            ),
        )
        .add_audio_channel::<PlayerWalkChannel>()
        .add_audio_channel::<PlayerBumpChannel>()
        .add_event::<PlayerMovementActions>()
        .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

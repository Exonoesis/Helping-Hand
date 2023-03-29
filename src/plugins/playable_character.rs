use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::AudioApp;

use crate::{audio::sfx::*, entities::player::*, mechanics::input::*, AppState};

pub struct PlayableCharacterPlugin;

impl Plugin for PlayableCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame)
                .with_system(load_player_movement_sound)
                .with_system(load_player_bump_sound),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(move_player)
                .with_system(bound_player_movement)
                .with_system(play_player_movement_sound)
                .with_system(play_player_bump_sound),
        )
        .add_audio_channel::<PlayerWalkChannel>()
        .add_audio_channel::<PlayerBumpChannel>()
        .add_event::<PlayerMovementActions>()
        .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

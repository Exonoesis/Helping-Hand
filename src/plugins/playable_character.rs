use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::AudioApp;

use crate::{audio::sfx::*, entities::player::*, mechanics::input::*};

pub struct PlayableCharacterPlugin;

impl Plugin for PlayableCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_player_movement_sound)
            .add_startup_system(load_player_bump_sound)
            .add_system(move_player)
            .add_system(bound_player_movement)
            .add_system(play_player_movement_sound)
            .add_system(play_player_bump_sound)
            .add_audio_channel::<PlayerWalkChannel>()
            .add_audio_channel::<PlayerBumpChannel>()
            .add_event::<PlayerMovementActions>()
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}

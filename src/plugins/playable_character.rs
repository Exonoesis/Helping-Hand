use std::time::Duration;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::AudioApp;

use crate::{audio::sfx::*, entities::player::*, mechanics::input::*, AppState};

pub struct PlayableCharacterPlugin;

impl Plugin for PlayableCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementDirection>();
        app.insert_resource(ArrivalTime::new(Duration::from_secs_f32(0.5)));

        app.add_systems(
            Update,
            (set_player_target, move_entity_to_target).run_if(in_state(AppState::InGame)),
        );
        //app.add_systems(
        //    OnEnter(AppState::InGame),
        //    (load_player_movement_sound, load_player_bump_sound),
        //)
        //.add_systems(
        //    Update,
        //    (
        //        move_entity,
        //        animate_entity,
        //        interact_entity,
        //        display_interactive_message.after(interact_entity),
        //        transition_level.after(interact_entity),
        //        bound_player_movement,
        //        play_player_movement_sound.after(move_entity),
        //        play_player_bump_sound.after(move_entity),
        //    )
        //        .run_if(in_state(AppState::InGame)),
        //)
        //.add_audio_channel::<PlayerWalkChannel>()
        //.add_audio_channel::<PlayerBumpChannel>()
        //.add_event::<PlayerMovementActions>()
        //.add_event::<InteractionEvent>()
        //.register_ldtk_entity::<PlayerBundle>("Player");
    }
}

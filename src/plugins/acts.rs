use crate::narrative::act_loading::*;
use crate::AppState;
use bevy::prelude::*;

pub struct ActsPlugin;

impl Plugin for ActsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadAct>()
            .add_systems(
                Update,
                (load_act, render_current_scene).run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                OnEnter(AppState::InGame),
                load_starting_act.run_if(in_state(AppState::InGame)),
            );
    }
}

pub struct MockActsPlugin;

impl Plugin for MockActsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadAct>().add_systems(
            Update,
            (load_act, render_current_scene).run_if(in_state(AppState::InGame)),
        );
    }
}

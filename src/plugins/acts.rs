use crate::narrative::act_loading::*;
use bevy::prelude::*;

pub struct MockActsPlugin;

use crate::AppState;

impl Plugin for MockActsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadAct>().add_systems(
            Update,
            (load_act, render_current_scene).run_if(in_state(AppState::InGame)),
        );
    }
}

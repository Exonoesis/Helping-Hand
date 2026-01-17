use std::time::Duration;

use crate::narrative::act_loading::*;
use crate::AppState;
use bevy::prelude::*;

pub struct ActsPlugin;

impl Plugin for ActsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CoreActsPlugin::new(Duration::from_secs(3)))
            .add_systems(
                Update,
                (load_next_scene_on_player_input,).run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                OnEnter(AppState::InGame),
                load_starting_act.run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Resource, Clone, Copy)]
pub struct FadeDuration {
    duration: Duration,
}

impl FadeDuration {
    pub fn new(duration: Duration) -> Self {
        Self { duration }
    }

    pub fn get_duration(&self) -> Duration {
        self.duration
    }
}

pub struct CoreActsPlugin {
    time_to_fade: FadeDuration,
}

impl CoreActsPlugin {
    pub fn new(fade_duration: Duration) -> Self {
        let time_to_fade = FadeDuration::new(fade_duration);
        Self { time_to_fade }
    }
}

impl Plugin for CoreActsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.time_to_fade);

        app.add_event::<LoadAct>()
            .add_event::<LoadNextScene>()
            .add_event::<ImageDespawn>()
            .add_systems(
                Update,
                (
                    load_act,
                    load_starting_scene.after(load_act),
                    fade_into.after(load_starting_scene),
                    despawn_image.after(fade_into),
                    load_next_scene.after(despawn_image),
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

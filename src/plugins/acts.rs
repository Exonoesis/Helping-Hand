use std::path::PathBuf;
use std::time::Duration;

use crate::map::interactions::map_changing::ChangeLevel;
use crate::narrative::act_loading::SpawnScene as SpawnOurScene;
use crate::narrative::act_loading::*;
use crate::AppState;
use bevy::prelude::*;

pub struct ActsPlugin;

impl Plugin for ActsPlugin {
    fn build(&self, app: &mut App) {
        let map_folder_path = PathBuf::from("assets/map/");

        app.add_plugins(CoreActsPlugin::new(Duration::from_secs(2), map_folder_path))
            .add_systems(
                Update,
                (load_next_scene_on_player_input,).run_if(in_state(AppState::InScene)),
            )
            .add_systems(
                OnExit(AppState::MainMenu),
                load_starting_act.run_if(in_state(AppState::Transitioning)),
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

#[derive(Resource, Clone)]
pub struct MapsFolderPath {
    folder_path: PathBuf,
}

impl MapsFolderPath {
    pub fn new(folder_path: PathBuf) -> Self {
        Self { folder_path }
    }

    pub fn get_path(&self) -> PathBuf {
        self.folder_path.clone()
    }
}

pub struct CoreActsPlugin {
    time_to_fade: FadeDuration,
    maps_path_folder: MapsFolderPath,
}

impl CoreActsPlugin {
    pub fn new(fade_duration: Duration, maps_folder_path: PathBuf) -> Self {
        let time_to_fade = FadeDuration::new(fade_duration);
        let maps_path_folder = MapsFolderPath::new(maps_folder_path);
        Self {
            time_to_fade,
            maps_path_folder,
        }
    }
}

impl Plugin for CoreActsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NextSceneToLoad::default());
        app.insert_resource(self.time_to_fade);
        app.insert_resource(self.maps_path_folder.clone());

        app.add_message::<LoadAct>()
            .add_message::<LoadNextScene>()
            .add_message::<CurtainIsDown>()
            .add_message::<DespawnScene>()
            .add_message::<DespawnDone>()
            .add_message::<SpawnOurScene>()
            .add_message::<SpawnDone>()
            .add_message::<ChangeLevel>()
            .add_systems(OnEnter(AppState::Transitioning), spawn_curtain)
            .add_systems(Update, load_next_scene.run_if(in_state(AppState::InScene)))
            .add_systems(
                Update,
                (
                    load_act,
                    curtain_down,
                    despawn_old_scene,
                    despawn_image,
                    despawn_map_cutscene,
                    spawn_new_scene,
                    render_image_cutscene,
                    render_map_cutscene,
                    wait_until_assets_loaded,
                    set_curtain_to_raise,
                    curtain_up,
                )
                    .run_if(in_state(AppState::Transitioning)),
            )
            .add_systems(OnExit(AppState::Transitioning), despawn_curtain);
    }
}

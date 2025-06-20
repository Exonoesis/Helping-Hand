use crate::{map::interactions::map_changing::CameraBundle, ui::menus::ImageNodeBundle};
use bevy::prelude::*;
use std::path::PathBuf;

use super::acts::read_act_from;
use crate::narrative::acts::*;

// Q: Is there a better way to do this? Should Scene be renamed to be less generic and
// not conflict with Bevy's Scene type?
use crate::narrative::acts::Scene as HelpingHandScene;

#[derive(Component)]
pub struct SceneUI;

#[derive(Event)]
pub struct LoadAct {
    act_path_name: String,
}

impl LoadAct {
    pub fn new(act_path_name: &str) -> Self {
        Self {
            act_path_name: String::from(act_path_name),
        }
    }

    pub fn get_act_file_path(&self) -> &str {
        &self.act_path_name
    }
}

#[derive(Event)]
pub struct SceneFade {
    previous_scene: HelpingHandScene,
}

impl SceneFade {
    pub fn new(previous_scene: HelpingHandScene) -> Self {
        Self { previous_scene }
    }
    pub fn get_previous_scene(&self) -> &HelpingHandScene {
        &self.previous_scene
    }
}

#[derive(Event)]
pub struct SceneTransition {
    previous_scene: HelpingHandScene,
}

impl SceneTransition {
    pub fn new(previous_scene: HelpingHandScene) -> Self {
        Self { previous_scene }
    }
    pub fn get_previous_scene(&self) -> &HelpingHandScene {
        &self.previous_scene
    }
}

// Q: Should this hold a reference to the current act?
#[derive(Event)]
pub struct LoadNextScene {}

impl LoadNextScene {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn load_starting_act(mut load_act_broadcaster: EventWriter<LoadAct>) {
    let starting_act = LoadAct::new("assets/acts/introductory_act.json");
    load_act_broadcaster.send(starting_act);
}

pub fn load_act(
    mut load_act_requests: EventReader<LoadAct>,
    mut commands: Commands,
    loaded_act: Query<Entity, With<Act>>,
) {
    if load_act_requests.is_empty() {
        return;
    }

    // If an act is already loaded, despawn it
    if loaded_act.iter().next().is_some() {
        for entity in loaded_act.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    let load_act_request = load_act_requests.read().next().unwrap();

    let act_file_path = PathBuf::from(load_act_request.get_act_file_path());
    let loaded_act = read_act_from(act_file_path);

    commands.spawn(loaded_act);

    let the_camera = CameraBundle::default();
    commands.spawn(the_camera);
}

pub fn render_current_scene(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    current_act: Query<&Act>,
    scene_ui: Query<Entity, With<SceneUI>>,
) {
    let found_loaded_act = current_act.iter().next();

    if found_loaded_act.is_none() {
        return;
    }

    // If there's already a scene loaded, do nothing
    if !scene_ui.is_empty() {
        return;
    }

    let node = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_direction: FlexDirection::Column,
        ..Default::default()
    };

    let current_scene = found_loaded_act.unwrap().get_current_scene();
    let scene_contents = current_scene.get_scene_contents();
    let scene_image = scene_contents.get_image_path().to_str().unwrap();

    let image = asset_server
        .load(format!("acts/images/{}", scene_image))
        .into();

    let ui_container = (ImageNodeBundle::from_nodes(node, image), SceneUI);

    commands.spawn(ui_container);
}

pub fn load_next_scene(
    load_next_scene_requests: EventReader<LoadNextScene>,
    mut current_act_query: Query<&mut Act>,
    mut scene_transition_broadcaster: EventWriter<SceneTransition>,
) {
    if load_next_scene_requests.is_empty() {
        return;
    }

    let mut current_act = current_act_query.single_mut();

    let scene_to_transition_from = current_act.get_current_scene().clone();

    if !current_act.has_more_scenes() {
        return;
    }

    current_act.move_to_next_scene();

    scene_transition_broadcaster.send(SceneTransition::new(scene_to_transition_from));
}

pub fn transition_from(
    mut transition_requests: EventReader<SceneTransition>,
    mut scene_fade_broadcaster: EventWriter<SceneFade>,
) {
    if transition_requests.is_empty() {
        return;
    }

    let transition_request = transition_requests.read().next().unwrap();
    let scene_to_transition_from = transition_request.get_previous_scene().clone();

    // TODO: Should check what scene type we are transitioning from (and to?)
    scene_fade_broadcaster.send(SceneFade::new(scene_to_transition_from));
}

pub fn fade() {
    // TODO: fade one scene in over another
    // Sends a DespawnImage event
}

pub fn despawn_image() {
    // TODO: Despawn image in response to fade event
}

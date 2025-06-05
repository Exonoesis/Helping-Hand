use crate::ui::menus::ImageNodeBundle;
use bevy::prelude::*;
use std::path::PathBuf;

use super::acts::read_act_from;
use crate::narrative::acts::Act;

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

/// TODO: Load a predetermined Act when clicking the "Play" button?
/// Or just load the first one at the end of load_act?

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

    // If there's already a scene loaded, despawn it
    if scene_ui.iter().next().is_some() {
        for entity in scene_ui.iter() {
            commands.entity(entity).despawn_recursive();
        }
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

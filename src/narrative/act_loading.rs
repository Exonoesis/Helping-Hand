use crate::{map::interactions::map_changing::CameraBundle, ui::menus::ImageNodeBundle};
use bevy::input::*;
use bevy::prelude::*;
use std::path::PathBuf;
use std::time::Duration;

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

#[derive(Component)]
pub struct FadeTimer {
    timer: Timer,
}

impl FadeTimer {
    pub fn new(timer: Timer) -> Self {
        Self { timer }
    }
}

// TODO: Is this an Entity? ImageNode? What am I looking for?
#[derive(Event)]
pub struct ImageDespawn {
    image_to_despawn: Entity,
}

impl ImageDespawn {
    pub fn new(image_to_despawn: Entity) -> Self {
        Self { image_to_despawn }
    }
    pub fn get_image_to_despawn(&self) -> &Entity {
        &self.image_to_despawn
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

    let node = create_full_screen_node();

    let current_scene = found_loaded_act.unwrap().get_current_scene();
    let scene_contents = current_scene.get_scene_contents();
    let scene_image = scene_contents.get_image_path().to_str().unwrap();

    let image = asset_server
        .load(format!("acts/images/{}", scene_image))
        .into();

    let ui_container = (ImageNodeBundle::from_nodes(node, image), SceneUI);

    commands.spawn(ui_container).insert(ZIndex(0));
}

// This is the spawning function
pub fn load_next_scene(
    mut load_next_scene_requests: EventReader<LoadNextScene>,
    mut current_act_query: Query<&mut Act>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if load_next_scene_requests.is_empty() {
        return;
    }

    load_next_scene_requests.read().next();
    let mut current_act = current_act_query.single_mut();

    if !current_act.has_more_scenes() {
        return;
    }

    current_act.move_to_next_scene();

    let node = create_full_screen_node();

    let current_scene = current_act.get_current_scene();
    let scene_contents = current_scene.get_scene_contents();
    let scene_image = scene_contents.get_image_path().to_str().unwrap();

    let image = asset_server
        .load(format!("acts/images/{}", scene_image))
        .into();

    let mut image_node = ImageNode::default();
    image_node.image = image;
    image_node.color.set_alpha(0.0);

    let ui_container = (ImageNodeBundle::from_nodes(node, image_node), SceneUI);

    // Attach Timer Component here
    let duration = Duration::new(3, 0);
    let timer = Timer::new(duration, TimerMode::Once);
    let fade_timer = FadeTimer::new(timer);

    commands
        .spawn(ui_container)
        .insert(ZIndex(1))
        .insert(fade_timer);
}

// This is the fading function
pub fn fade_into(mut despawn_image_broadcaster: EventWriter<ImageDespawn>) {
    // TODO: fade one scene in over another

    // let timer = set_timer(15)
    // while timer > 0

    // Sends a DespawnImage event

    // despawn_image(entity) <- Event broadcast here
}

pub fn despawn_image() {
    // TODO: Despawn image in response to fade function
    //
    // remove Node 2's timer component
    // set Node 2's ZIndex to 0
}

pub fn create_full_screen_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..Default::default()
    }
}

// DELETE THIS?
pub fn load_next_scene_on_key_press(
    input: Res<ButtonInput<KeyCode>>,
    mut load_next_scene_broadcaster: EventWriter<LoadNextScene>,
) {
    if input.just_released(KeyCode::KeyN) {
        load_next_scene_broadcaster.send(LoadNextScene::new());
    }
}

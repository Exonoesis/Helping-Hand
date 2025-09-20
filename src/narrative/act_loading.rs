use crate::plugins::acts::FadeDuration;
use crate::{map::interactions::map_changing::CameraBundle, ui::menus::ImageNodeBundle};
use bevy::prelude::*;
use std::path::{Path, PathBuf};

use super::acts::read_act_from;
use crate::narrative::acts::*;

/// Identifies components created for a single scene
#[derive(Component)]
pub struct SceneUI;

/// Timer for fading Image Cutscenes
#[derive(Component)]
pub struct FadeTimer {
    timer: Timer,
}

impl FadeTimer {
    pub fn new(timer: Timer) -> Self {
        Self { timer }
    }

    pub fn get_timer(&mut self) -> &mut Timer {
        &mut self.timer
    }
}

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
pub struct ImageDespawn {}

impl ImageDespawn {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Event)]
pub struct LoadNextScene {}

impl LoadNextScene {
    pub fn new() -> Self {
        Self {}
    }
}

/// Loads initial Act of the game
pub fn load_starting_act(mut load_act_broadcaster: EventWriter<LoadAct>) {
    let starting_act = LoadAct::new("assets/acts/introductory_act.json");
    load_act_broadcaster.write(starting_act);
}

pub fn load_act(
    mut load_act_requests: EventReader<LoadAct>,
    mut commands: Commands,
    loaded_act: Query<Entity, With<Act>>,
) {
    if load_act_requests.is_empty() {
        return;
    }

    if loaded_act.iter().next().is_some() {
        for entity in loaded_act.iter() {
            commands.entity(entity).despawn();
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

pub fn load_next_scene(
    mut load_next_scene_requests: EventReader<LoadNextScene>,
    mut current_act_query: Query<&mut Act>,
    asset_server: Res<AssetServer>,
    fade_duration: Res<FadeDuration>,
    mut commands: Commands,
) {
    if load_next_scene_requests.is_empty() {
        return;
    }

    load_next_scene_requests.read().next();
    let mut current_act = current_act_query.single_mut().unwrap();

    if !current_act.has_more_scenes() {
        return;
    }

    current_act.move_to_next_scene();

    let node = create_full_screen_node();

    let current_scene = current_act.get_current_scene();
    let scene_contents = current_scene.get_scene_contents();
    let scene_image_path = scene_contents.get_image_path();
    let scene_image = scene_image_path.to_str().unwrap();

    // Check image path is correct
    let image = check_image_path(asset_server, scene_image);

    // Set image to be invisible
    let mut image_node = ImageNode::default();
    image_node.image = image;
    image_node.color.set_alpha(0.0);

    let ui_container = (ImageNodeBundle::from_nodes(node, image_node), SceneUI);

    // Create Timer Component
    let duration = fade_duration.get_duration();
    let timer = Timer::new(duration, TimerMode::Once);
    let fade_timer = FadeTimer::new(timer);

    commands
        .spawn(ui_container)
        .insert(ZIndex(1))
        .insert(fade_timer);
}

pub fn fade_into(
    mut query: Query<(&mut ImageNode, &mut FadeTimer)>,
    time: Res<Time>,
    mut despawn_image_broadcaster: EventWriter<ImageDespawn>,
) {
    for (mut image_node, mut fade_timer) in query.iter_mut() {
        fade_timer.get_timer().tick(time.delta());

        image_node
            .color
            .set_alpha(fade_timer.get_timer().fraction());

        if fade_timer.get_timer().finished() {
            despawn_image_broadcaster.write(ImageDespawn::new());
        }
    }
}

pub fn despawn_image(
    mut despawn_image_requests: EventReader<ImageDespawn>,
    scene_to_remove_query: Query<Entity, (With<SceneUI>, Without<FadeTimer>)>,
    mut current_scene_query: Query<Entity, (With<SceneUI>, With<FadeTimer>)>,
    mut commands: Commands,
) {
    if despawn_image_requests.is_empty()
        || scene_to_remove_query.is_empty()
        || current_scene_query.is_empty()
    {
        return;
    }

    despawn_image_requests.read().next();

    // Despawn previous image
    for entity in scene_to_remove_query.iter() {
        commands.entity(entity).despawn();
    }

    // Remove new images Timer Componenet and set ZIndex to 0
    for entity in current_scene_query.iter_mut() {
        commands
            .entity(entity)
            .remove::<FadeTimer>()
            .insert(ZIndex(0));
    }
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

/// Progresses to the next image cutscene on any key or mouse button press
pub fn load_next_scene_on_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    current_act_query: Query<&mut Act>,
    mut load_next_scene_broadcaster: EventWriter<LoadNextScene>,
) {
    let found_loaded_act = current_act_query.iter().next();

    if found_loaded_act.is_none() {
        return;
    }

    let current_act = found_loaded_act.unwrap();
    let current_scene = current_act.get_current_scene();
    let current_scene_type = current_scene.get_scene_contents();

    if matches!(current_scene_type, SceneContents::ImageCutscene(_)) {
        if keyboard_input.get_just_pressed().next().is_some()
            || mouse_button_input.get_just_pressed().next().is_some()
        {
            load_next_scene_broadcaster.write(LoadNextScene::new());
        }
    }
}

// So this always panics, even though the path is fine
pub fn check_image_path(asset_server: Res<AssetServer>, scene_image: &str) -> Handle<Image> {
    let image: Handle<Image> = asset_server
        .load(format!("acts/images/{}", scene_image))
        .into();

    let image_path: &Path = image.path().unwrap().path();

    let mut asset_path = PathBuf::new();
    // Bevy will not report the folder of the asset server. By default,
    // this is the `assets` folder at the root of the project.
    //
    // This has to be hardcoded in the meantime. Otherwise, no image would
    // ever be found, since it would look in the `acts` folder at root, but
    // it should really be in `assets/acts` for example.
    asset_path.push("assets/");
    asset_path.push(image_path);
    if !asset_path.exists() {
        panic!(
            "check_image_path: Check the file path for asset: {}",
            asset_path.display()
        )
    }

    image
}

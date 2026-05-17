use crate::map::interactions::map_changing::ChangeLevel;
use crate::narrative::acts::{Act, SceneContents, SceneType};
use crate::plugins::acts::{FadeDuration, MapsFolderPath};
use crate::ui::menus::ImageNodeBundle;
use crate::AppState;
use bevy::color::palettes::css::BLACK;
use bevy::prelude::*;
use std::path::{Path, PathBuf};

use super::acts::ActLoader;

/// Identifies components created for a single scene
#[derive(Component)]
pub struct ImageCutscene;

/// Timer for fading the transition curtain in
/// [Transparent -> Opaque]
#[derive(Component)]
pub struct CurtainDownTimer {
    timer: Timer,
}

impl CurtainDownTimer {
    pub fn new(fade_duration: &FadeDuration) -> Self {
        let timer = Timer::new(fade_duration.get_duration() / 2, TimerMode::Once);

        Self { timer }
    }

    pub fn get_timer(&mut self) -> &mut Timer {
        &mut self.timer
    }
}

/// Timer for fading the transition curtain out
/// [Opaque -> Transparent]
#[derive(Component)]
pub struct CurtainUpTimer {
    timer: Timer,
}

impl CurtainUpTimer {
    pub fn new(fade_duration: &FadeDuration) -> Self {
        let timer = Timer::new(fade_duration.get_duration() / 2, TimerMode::Once);

        Self { timer }
    }

    pub fn get_timer(&mut self) -> &mut Timer {
        &mut self.timer
    }
}

#[derive(Message)]
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

#[derive(Message)]
pub struct LoadNextScene;

/// Loads initial act of the game
pub fn load_starting_act(mut load_act_broadcaster: MessageWriter<LoadAct>) {
    let starting_act = LoadAct::new("assets/acts/introductory_act.json");
    load_act_broadcaster.write(starting_act);
}

pub fn load_act(
    mut load_act_requests: MessageReader<LoadAct>,
    mut commands: Commands,
    loaded_act: Query<Entity, With<Act>>,
    maps_path_folder: Res<MapsFolderPath>,
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
    let maps_folder = maps_path_folder.get_path();

    let act_loader = ActLoader::new(act_file_path, maps_folder);
    let loaded_act = act_loader.read_act_from();

    commands.spawn(loaded_act);
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

/// Renders the next scene into the game from the current act
pub fn load_next_scene(
    mut load_next_scene_requests: MessageReader<LoadNextScene>,
    mut current_act_query: Query<&mut Act>,
    mut next_state: ResMut<NextState<AppState>>,
    mut next_scene_to_load: ResMut<NextSceneToLoad>,
) {
    if load_next_scene_requests.is_empty() {
        return;
    }

    load_next_scene_requests.read().next();

    let current_act = current_act_query.single_mut().unwrap();

    if !current_act.has_more_scenes() {
        return;
    }

    let new_scene_id = current_act.get_current_scene_id() + 1;
    next_scene_to_load.set_scene_id(new_scene_id);

    next_state.set(AppState::Transitioning);
}

#[derive(Component)]
pub struct Curtain;

// On Enter Transitioning -> Starting Point
pub fn spawn_curtain(fade_duration: Res<FadeDuration>, mut commands: Commands) {
    let node = create_full_screen_node();
    let black_image = ImageNode::solid_color(BLACK.into());

    let curtain = (ImageNodeBundle::from_nodes(node, black_image), Curtain);
    let curtain_z_index = ZIndex(i32::MAX);
    let curtain_down_timer = CurtainDownTimer::new(&fade_duration);

    commands
        .spawn(curtain)
        .insert(curtain_z_index)
        .insert(curtain_down_timer);
}

#[derive(Message)]
pub struct CurtainIsDown;

pub fn curtain_down(
    mut curtain_query: Query<(Entity, &mut ImageNode, &mut CurtainDownTimer)>,
    time: Res<Time>,
    mut despawn_image_broadcaster: MessageWriter<CurtainIsDown>,
    mut commands: Commands,
) {
    for (curtain_entity, mut curtain_image, mut curtain_down_timer) in curtain_query.iter_mut() {
        curtain_down_timer.get_timer().tick(time.delta());

        curtain_image
            .color
            .set_alpha(curtain_down_timer.get_timer().fraction());

        if curtain_down_timer.get_timer().is_finished() {
            commands.entity(curtain_entity).remove::<CurtainDownTimer>();
            despawn_image_broadcaster.write(CurtainIsDown);
        }
    }
}

#[derive(Message, PartialEq)]
pub enum DespawnScene {
    Image,
    Map,
}

pub fn despawn_old_scene(
    mut curtain_is_down_requests: MessageReader<CurtainIsDown>,
    current_act: Single<&Act>,
    mut despawn_notification: MessageWriter<DespawnScene>,
) {
    if curtain_is_down_requests.is_empty() {
        return;
    }

    curtain_is_down_requests.read().next();

    let current_scene = current_act.get_current_scene();
    let scene_type = current_scene.get_scene_type();

    match scene_type {
        SceneType::ImageCutscene => {
            despawn_notification.write(DespawnScene::Image);
        }
        SceneType::MapCutscene => {
            despawn_notification.write(DespawnScene::Map);
        }
    }
}

#[derive(Message)]
pub struct DespawnDone;

// TODO: Refactor
pub fn despawn_image(
    mut despawn_image_requests: MessageReader<DespawnScene>,
    scene_to_remove: Query<Entity, With<ImageCutscene>>,
    mut commands: Commands,
    mut despawning_done_notification: MessageWriter<DespawnDone>,
) {
    if despawn_image_requests.is_empty() {
        return;
    }

    let despawn_event = despawn_image_requests.read().next().unwrap();

    for scene_entity in scene_to_remove {
        if *despawn_event == DespawnScene::Image {
            commands.entity(scene_entity).despawn();
        }
    }

    despawning_done_notification.write(DespawnDone);
}

pub fn despawn_map_cutscene() {
    // TODO: Nearly the same as despawn_image
    // Need to split change level into despawn_map and load_next_map (name pending)
}

#[derive(Message, PartialEq)]
pub enum SpawnScene {
    Image,
    Map,
}

#[derive(Resource, Default)]
pub struct NextSceneToLoad {
    next_scene_id: usize,
}

impl NextSceneToLoad {
    pub fn get_scene_id(&self) -> usize {
        self.next_scene_id
    }

    pub fn set_scene_id(&mut self, new_scene_id: usize) {
        self.next_scene_id = new_scene_id;
    }
}

pub fn spawn_new_scene(
    mut despawn_done_requests: MessageReader<DespawnDone>,
    mut current_act: Single<&mut Act>,
    mut spawn_notification: MessageWriter<SpawnScene>,
    next_scene_to_load: Res<NextSceneToLoad>,
) {
    if despawn_done_requests.is_empty() {
        return;
    }

    despawn_done_requests.read().next();

    let next_scene_id = next_scene_to_load.get_scene_id();
    current_act.set_to_scene(next_scene_id);

    let current_scene = current_act.get_current_scene();
    let scene_type = current_scene.get_scene_type();

    match scene_type {
        SceneType::ImageCutscene => {
            spawn_notification.write(SpawnScene::Image);
        }
        SceneType::MapCutscene => {
            spawn_notification.write(SpawnScene::Map);
        }
    }
}

#[derive(Message)]
pub struct SpawnDone;

/// Render an Image Cutscene into the game
pub fn render_image_cutscene(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    current_act: Single<&Act>,
    mut spawn_image_requests: MessageReader<SpawnScene>,
    mut spawning_done_notification: MessageWriter<SpawnDone>,
) {
    if spawn_image_requests.is_empty() {
        return;
    }

    let spawn_event = spawn_image_requests.read().next().unwrap();

    if *spawn_event != SpawnScene::Image {
        return;
    }

    let current_scene = current_act.get_current_scene();

    if let SceneContents::ImageCutscene(image_path) = current_scene.get_scene_contents() {
        let node = create_full_screen_node();
        let scene_image = image_path.to_str().unwrap();

        // Check image path is correct
        let image = check_image_path(&asset_server, scene_image);

        let ui_container = (ImageNodeBundle::from_nodes(node, image), ImageCutscene);

        commands.spawn(ui_container);
        spawning_done_notification.write(SpawnDone);
    }
}

/// Render a Map Cutscene into the game
pub fn render_map_cutscene(
    current_act: Single<&Act>,
    mut load_level_broadcaster: MessageWriter<ChangeLevel>,
    mut spawn_map_requests: MessageReader<SpawnScene>,
) {
    if spawn_map_requests.is_empty() {
        return;
    }

    let spawn_event = spawn_map_requests.read().next().unwrap();

    if *spawn_event != SpawnScene::Map {
        return;
    }

    let current_scene = current_act.get_current_scene();

    if let SceneContents::MapCutscene(map_path, map_actions) = current_scene.get_scene_contents() {
        let level_name = map_path.to_str().unwrap();
        load_level_broadcaster.write(ChangeLevel::new(level_name));

        //TODO: Load path objects
    }
}

pub fn set_curtain_to_raise(
    mut spawning_done_requests: MessageReader<SpawnDone>,
    curtain_entity: Single<Entity, With<Curtain>>,
    fade_duration: Res<FadeDuration>,
    mut commands: Commands,
) {
    if spawning_done_requests.is_empty() {
        return;
    }

    spawning_done_requests.read().next();

    let curtain_up_timer = CurtainUpTimer::new(&fade_duration);
    commands.entity(*curtain_entity).insert(curtain_up_timer);
}

pub fn curtain_up(
    mut curtain_query: Query<(Entity, &mut ImageNode, &mut CurtainUpTimer)>,
    time: Res<Time>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (curtain_entity, mut curtain_image, mut curtain_up_timer) in curtain_query.iter_mut() {
        curtain_up_timer.get_timer().tick(time.delta());

        let curtain_raised_percentage = 1.0 - curtain_up_timer.get_timer().fraction();

        curtain_image.color.set_alpha(curtain_raised_percentage);

        if curtain_up_timer.get_timer().is_finished() {
            commands.entity(curtain_entity).remove::<CurtainUpTimer>();
            next_state.set(AppState::InScene);
        }
    }
}

// On Exit Transitioning -> Ending Point
pub fn despawn_curtain(curtain_entity: Single<Entity, With<Curtain>>, mut commands: Commands) {
    commands.entity(*curtain_entity).despawn();
}

/// Progresses to the next image cutscene on any key or mouse button press
pub fn load_next_scene_on_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    current_act_query: Query<&mut Act>,
    mut load_next_scene_broadcaster: MessageWriter<LoadNextScene>,
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
            load_next_scene_broadcaster.write(LoadNextScene);
        }
    }
}

pub fn check_image_path(asset_server: &AssetServer, scene_image: &str) -> ImageNode {
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

    image.into()
}

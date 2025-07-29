use std::path::PathBuf;

use bevy::prelude::*;
use tiled::{Loader, Map};

use crate::map::{
    movement::{
        collision::{create_collision_collection_from, CollisionCollection},
        grid_based_movement::{set_physical_destination, MovementDirection},
    },
    player::*,
    render::RenderedMap,
    GridDimensions, PxDimensions, TileType, Tilemap, XyzCords,
};

use super::interactives::{
    flip_interactives_on_y_axis, get_interactives_from, InteractiveCollection,
};

#[derive(Event)]
pub struct LoadLevel {
    level_path: PathBuf,
}

impl LoadLevel {
    pub fn new(desired_level_name: &str) -> Self {
        Self {
            level_path: PathBuf::from(desired_level_name),
        }
    }

    pub fn from(change_level_request: &ChangeLevel) -> Self {
        Self {
            level_path: change_level_request.get_level_path().clone(),
        }
    }

    pub fn get_level_path(&self) -> &PathBuf {
        &self.level_path
    }

    pub fn get_level_name(&self) -> String {
        let level_name = self
            .level_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        level_name
    }
}

#[derive(Event)]
pub struct ChangeLevel {
    level_path: PathBuf,
}

impl ChangeLevel {
    pub fn new(desired_level_name: &str) -> Self {
        Self {
            level_path: PathBuf::from(desired_level_name),
        }
    }

    pub fn get_level_path(&self) -> &PathBuf {
        &self.level_path
    }
}

/// Loads some predetermined map when clicking the "Play" button.
pub fn load_starting_map(mut change_level_requester: EventWriter<LoadLevel>) {
    let tiled_map_name = "test_map_with_collision.tmx";
    let map_path = format!("tests/test-assets/maps/{}", tiled_map_name);
    change_level_requester.write(LoadLevel::new(&map_path));
}

/// Loads the Tiled test map with a Camera into the game at the center of the map.
pub fn load_map(
    mut change_level_requests: EventReader<LoadLevel>,
    mut commands: Commands,
    asset_spawner: Res<AssetServer>,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlasLayout>>,
) {
    if change_level_requests.is_empty() {
        return;
    }

    let change_level_request = change_level_requests.read().next().unwrap();
    let tiled_map = load_tiled_map(PathBuf::from(change_level_request.get_level_path()));
    let map = Tilemap::from_tiled(&tiled_map);
    let bevy_map = RenderedMap::new(&map, &asset_spawner, &mut texture_atlas_assets);

    let rendered_tiles = bevy_map.get_bevy_tiles();

    for render_tile in rendered_tiles {
        let render_tile = render_tile.clone();
        if render_tile.get_tile_type() == &TileType::Player {
            commands.spawn((render_tile, Player, MovementDirection::Left));
            continue;
        }

        commands.spawn(render_tile);
    }

    let camera_centered_to_map = create_centered_camera(&map);
    commands.spawn(camera_centered_to_map);

    // This section represents the Physical properties of the map.
    let map_size_in_px = *bevy_map.get_px_dimensions();
    let map_grid_dimenions = *bevy_map.get_grid_dimensions();
    let mut interactives = get_interactives_from(&tiled_map);
    // We have to flip the y-axis of all tiles, since they're physical coordinates.
    interactives = flip_interactives_on_y_axis(interactives, map_size_in_px, map_grid_dimenions);
    let interactive_collection = InteractiveCollection::from_markers(interactives);
    let physical_properties = (map_size_in_px, interactive_collection);

    // This section represents all of the Logical properties of the map.
    let collision_collection = create_collision_collection_from(&bevy_map);
    let map_size_in_tiles = *bevy_map.get_grid_dimensions();
    let logical_properties = (collision_collection, map_size_in_tiles);

    commands.spawn((physical_properties, logical_properties));
}

pub fn change_to_new_level(
    mut change_level_requests: EventReader<ChangeLevel>,
    mut load_level_broadcaster: EventWriter<LoadLevel>,
    loaded_level_tiles: Query<(Entity, &XyzCords, &TileType, &PxDimensions)>,
    map_properties: Query<
        Entity,
        (
            With<PxDimensions>,
            With<InteractiveCollection>,
            With<CollisionCollection>,
            With<GridDimensions>,
        ),
    >,
    camera: Query<Entity, With<Camera2d>>,
    mut commands: Commands,
) {
    if change_level_requests.is_empty() {
        return;
    }

    for loaded_tile in &loaded_level_tiles {
        let loaded_tile_entity = loaded_tile.0;
        commands.entity(loaded_tile_entity).despawn();
    }

    let camera_entity = camera.single().unwrap();
    commands.entity(camera_entity).despawn();

    let map_properties_entity = map_properties.single().unwrap();
    commands.entity(map_properties_entity).despawn();

    let change_level_request = change_level_requests.read().next().unwrap();
    let load_level_request = LoadLevel::from(change_level_request);

    load_level_broadcaster.write(load_level_request);
}

/// Returns a loaded Tiled map.
pub fn load_tiled_map(map_location: PathBuf) -> Map {
    let mut loader = Loader::new();
    loader.load_tmx_map(map_location).unwrap()
}

#[derive(Bundle)]
pub struct CameraBundle {
    camera: Camera2d,
    transform: Transform,
}

impl Default for CameraBundle {
    fn default() -> Self {
        Self {
            camera: Camera2d::default(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        }
    }
}

/// Returns a camera centered to some map.
fn create_centered_camera(map: &Tilemap) -> CameraBundle {
    let mut the_camera = CameraBundle::default();
    let camera_position = &mut the_camera.transform;

    let horizontal_center = (map.get_px_dimensions().get_width() / 2) as f32;
    let vertical_center = (map.get_px_dimensions().get_height() / 2) as f32;

    *camera_position = Transform::from_xyz(horizontal_center, vertical_center, 999.0);

    the_camera
}

/// Changes the level if there's a marker present in front of the player and it is transitional.
pub fn change_level_from_marker(
    mut requests_to_interact: EventReader<PlayerInteraction>,
    player: Query<(&Transform, &PxDimensions, &MovementDirection), With<Player>>,
    map_markers: Query<(&InteractiveCollection, &PxDimensions)>,
    mut change_level_requests: EventWriter<ChangeLevel>,
) {
    if player.is_empty() {
        return;
    }

    if requests_to_interact.is_empty() {
        return;
    }

    if map_markers.is_empty() {
        return;
    }

    let (current_player_position, player_dimensions, player_direction) = player.single().unwrap();

    // We use _ as a placeholder since there is currently only one type
    // of PlayerInteraction, therefore we don't need to read the type
    for _ in requests_to_interact.read() {
        let (marker_collection, map_dimensions_in_px) = map_markers.single().unwrap();

        let found_inspected_point = set_physical_destination(
            current_player_position,
            player_dimensions,
            map_dimensions_in_px,
            player_direction,
        );

        if found_inspected_point.is_none() {
            continue;
        }

        let inspected_point = found_inspected_point.unwrap();
        let inspected_cords = transform_to_xyzcord(inspected_point);
        let found_marker = marker_collection.get_marker_from_position(&inspected_cords);

        if found_marker.is_none() {
            return;
        }

        let marker = found_marker.unwrap();
        if marker.get_type_name() != "Transition".to_string() {
            return;
        }

        let level_name = ChangeLevel::new(&marker.get_path().to_str().unwrap());
        change_level_requests.write(level_name);
    }
}

// This function loses floating point accuracy
pub fn transform_to_xyzcord(transform: Transform) -> XyzCords {
    XyzCords::new(
        transform.translation.x as usize,
        transform.translation.y as usize,
        transform.translation.z as usize,
    )
}

use std::path::PathBuf;

use bevy::prelude::*;
use tiled::{Loader, Map};

use crate::map::{
    movement::{
        collision::{create_collision_collection_from, CollisionCollection},
        grid_based_movement::MovementDirection,
    },
    npc::NPC,
    player::*,
    render::{get_sprite_bundle, EnvironmentalTile, NPCTile, PlayerTile},
    GridCords3D, GridDimensions, PxCords, PxDimensions, Tile, Tilemap,
};

use super::interactives::{
    flip_interactives_on_y_axis, get_interactives_from, InteractiveCollection,
};

#[derive(Message)]
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

#[derive(Message)]
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

fn is_environmental(tile: &Tile) -> bool {
    tile.get_properties().get("type").unwrap() == "Environmental"
}

fn render_environmental_tile(
    tile: &Tile,
    tilemap_dimensions: &PxDimensions,
    asset_server: &AssetServer,
    texture_atlas_assets: &mut Assets<TextureAtlasLayout>,
) -> EnvironmentalTile {
    let tile_size = tile.get_tile_dimensions();
    let tile_location = tile.get_grid_coordinates();
    let tile_sprite =
        get_sprite_bundle(tile, tilemap_dimensions, asset_server, texture_atlas_assets);
    let parsed_environmental_tile = EnvironmentalTile::new(*tile_size, *tile_location, tile_sprite);
    return parsed_environmental_tile;
}

fn is_player(tile: &Tile) -> bool {
    tile.get_properties().get("type").unwrap() == "Player"
}

fn render_player_tile(
    tile: &Tile,
    tilemap_dimensions: &PxDimensions,
    asset_server: &AssetServer,
    texture_atlas_assets: &mut Assets<TextureAtlasLayout>,
) -> PlayerTile {
    let tile_size = tile.get_tile_dimensions();
    let tile_location = tile.get_grid_coordinates();
    let tile_sprite =
        get_sprite_bundle(tile, tilemap_dimensions, asset_server, texture_atlas_assets);
    let player = Player::new(tile.get_properties().get("name").unwrap().clone());
    let movement_direction = MovementDirection::Left;
    let parsed_player_tile = PlayerTile::new(
        *tile_size,
        *tile_location,
        tile_sprite,
        player,
        movement_direction,
    );
    return parsed_player_tile;
}

fn is_npc(tile: &Tile) -> bool {
    tile.get_properties().get("type").unwrap() == "NPC"
}

fn render_npc_tile(
    tile: &Tile,
    tilemap_dimensions: &PxDimensions,
    asset_server: &AssetServer,
    texture_atlas_assets: &mut Assets<TextureAtlasLayout>,
) -> NPCTile {
    let tile_size = tile.get_tile_dimensions();
    let tile_location = tile.get_grid_coordinates();
    let tile_sprite =
        get_sprite_bundle(tile, tilemap_dimensions, asset_server, texture_atlas_assets);
    let npc = NPC::new(tile.get_properties().get("name").unwrap().clone());
    let parsed_npc_tile = NPCTile::new(*tile_size, *tile_location, tile_sprite, npc);
    return parsed_npc_tile;
}

/// Loads the Tiled test map with a Camera into the game at the center of the map.
pub fn load_map(
    mut change_level_requests: MessageReader<LoadLevel>,
    mut commands: Commands,
    asset_spawner: Res<AssetServer>,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlasLayout>>,
    mut camera_position: Single<&mut Transform, With<Camera2d>>,
) {
    if change_level_requests.is_empty() {
        return;
    }

    let change_level_request = change_level_requests.read().next().unwrap();
    let tiled_map = load_tiled_map(PathBuf::from(change_level_request.get_level_path()));
    let map = Tilemap::from_tiled(&tiled_map);
    let map_dimensions = map.get_px_dimensions();

    let map_tiles = map.get_tiles();

    /*
     * let environmental_tiles: Vec<EnvironmentalTile> = render_environment_tiles(map_tiles, &map, &asset_spawner, &mut texture_atlas_assets)
     * commands.spawn_batch(environment_tiles)
     *
     * let npc_tiles: Vec<NPCTile> = render_npc_tiles(...)
     * commands.spawn_batch(npc_tiles)
     */
    for tile in map_tiles {
        if is_environmental(tile) {
            let rendered_environment_tile: EnvironmentalTile = render_environmental_tile(
                tile,
                &map_dimensions,
                &asset_spawner,
                &mut texture_atlas_assets,
            );
            commands.spawn(rendered_environment_tile);
            continue;
        }

        if is_player(tile) {
            let rendered_player_tile: PlayerTile = render_player_tile(
                tile,
                &map_dimensions,
                &asset_spawner,
                &mut texture_atlas_assets,
            );
            commands.spawn(rendered_player_tile);
            continue;
        }

        if is_npc(tile) {
            let rendered_npc_tile: NPCTile = render_npc_tile(
                tile,
                &map_dimensions,
                &asset_spawner,
                &mut texture_atlas_assets,
            );
            commands.spawn(rendered_npc_tile);
            continue;
        }
    }

    center_camera_on_map(&map, &mut camera_position);

    // This section represents the Physical properties of the map.
    let map_size_in_px = *map.get_px_dimensions();
    let map_grid_dimenions = *map.get_grid_dimensions();
    let mut interactives = get_interactives_from(&tiled_map);
    // We have to flip the y-axis of all tiles, since they're physical coordinates.
    interactives = flip_interactives_on_y_axis(interactives, map_size_in_px, map_grid_dimenions);
    let interactive_collection = InteractiveCollection::from_markers(interactives);
    let physical_properties = (map_size_in_px, interactive_collection);

    // This section represents all of the Logical properties of the map.
    let collision_collection = create_collision_collection_from(&map);
    let map_size_in_tiles = *map.get_grid_dimensions();
    let logical_properties = (collision_collection, map_size_in_tiles);

    commands.spawn((physical_properties, logical_properties));
}

pub fn change_to_new_level(
    mut change_level_requests: MessageReader<ChangeLevel>,
    mut load_level_broadcaster: MessageWriter<LoadLevel>,
    loaded_level_tiles: Query<(Entity, &GridCords3D, &PxDimensions)>,
    map_properties: Query<
        Entity,
        (
            With<PxDimensions>,
            With<InteractiveCollection>,
            With<CollisionCollection>,
            With<GridDimensions>,
        ),
    >,
    mut commands: Commands,
) {
    if change_level_requests.is_empty() {
        return;
    }

    for loaded_tile in &loaded_level_tiles {
        let loaded_tile_entity = loaded_tile.0;
        commands.entity(loaded_tile_entity).despawn();
    }

    for map_properties_entity in &map_properties {
        commands.entity(map_properties_entity).despawn();
    }

    let change_level_request = change_level_requests.read().next().unwrap();
    let load_level_request = LoadLevel::from(change_level_request);

    load_level_broadcaster.write(load_level_request);
}

/// Returns a loaded Tiled map.
pub fn load_tiled_map(map_location: PathBuf) -> Map {
    let mut loader = Loader::new();
    loader.load_tmx_map(map_location).unwrap()
}

/// Centers the camera on a given map.
fn center_camera_on_map(map: &Tilemap, camera_position: &mut Transform) {
    let horizontal_center = (map.get_px_dimensions().get_width() / 2) as f32;
    let vertical_center = (map.get_px_dimensions().get_height() / 2) as f32;

    camera_position.translation.x = horizontal_center;
    camera_position.translation.y = vertical_center;
}

/// Changes the level if there's a marker present in front of the player and it is transitional.
pub fn change_level_from_marker(
    mut requests_to_interact: MessageReader<PlayerInteraction>,
    player: Query<(&Transform, &PxDimensions, &MovementDirection), With<Player>>,
    map_markers: Query<(&InteractiveCollection, &PxDimensions)>,
    mut change_level_requests: MessageWriter<ChangeLevel>,
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

/// Returns a new pixel position shifted away from a starting position in a given direction
/// | Returns None if the new position would be out of bounds
pub fn set_physical_destination(
    current_position: &Transform,
    tile_dimensions: &PxDimensions,
    map_px_dimensions: &PxDimensions,
    direction: &MovementDirection,
) -> Option<Transform> {
    // We need to get the pixel location where we currently are to
    // do any sort of bounds checking
    let current_px_position = current_position.translation;
    let mut current_x = current_px_position.x;
    let mut current_y = current_px_position.y;
    let current_z = current_px_position.z;

    // Since we're checking the bounds of the map, we need the map dimensions
    let level_width = map_px_dimensions.get_width() as f32;
    let level_height = map_px_dimensions.get_height() as f32;

    // We also need to know what direction to look in from our current position
    // Each branch checks if the move we're about to do would go outside the map
    //
    // If so then it denies it by not giving back a new pixel location
    //
    // Otherwise it gives a new pixel location shifted in the given direction
    match direction {
        MovementDirection::Left => {
            if current_x == 0.0 {
                return None;
            }

            current_x -= tile_dimensions.get_width() as f32;
        }
        MovementDirection::Right => {
            if current_x == level_width - 1.0 {
                return None;
            }

            current_x += tile_dimensions.get_width() as f32;
        }
        MovementDirection::Up => {
            if current_y == level_height - 1.0 {
                return None;
            }

            current_y += tile_dimensions.get_height() as f32;
        }
        MovementDirection::Down => {
            if current_y == 0.0 {
                return None;
            }

            current_y -= tile_dimensions.get_height() as f32;
        }
    }
    Some(Transform::from_xyz(current_x, current_y, current_z))
}

// This function loses floating point accuracy
pub fn transform_to_xyzcord(transform: Transform) -> PxCords {
    PxCords::new(
        transform.translation.x as usize,
        transform.translation.y as usize,
        transform.translation.z as usize,
    )
}

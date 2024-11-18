use std::time::Duration;

use crate::entities::player::MovementIntent;
use crate::visuals::map::PxDimensions;
use crate::{
    entities::player::{DirectionFacing, Player, PlayerMovementActions},
    visuals::map::LevelDimensions,
};
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;
use bevy_ecs_ldtk::ldtk::FieldValue::String as StringType;
use bevy_ecs_ldtk::LevelSelection;
use bevy_ecs_ldtk::{prelude::*, EntityInstance, LevelIid};

#[derive(Event)]
pub struct InteractionEvent(String, String);

#[derive(Event)]
pub enum MovementDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
pub struct StartingPosition {
    position: Transform,
}

impl StartingPosition {
    pub fn new(px_position: Transform) -> Self {
        Self {
            position: px_position,
        }
    }

    pub fn get_position(&self) -> &Transform {
        &self.position
    }
}

#[derive(Component)]
pub struct Target {
    position: Transform,
}

impl Target {
    pub fn new(px_position: Transform) -> Self {
        Self {
            position: px_position,
        }
    }

    pub fn get_position(&self) -> &Transform {
        &self.position
    }
}

#[derive(Resource)]
pub struct ArrivalTime {
    time: Duration,
}

impl ArrivalTime {
    pub fn new(time: Duration) -> Self {
        Self { time }
    }

    pub fn get_duration(&self) -> &Duration {
        &self.time
    }
}

#[derive(Component)]
pub struct ArrivalTimer {
    timer: Timer,
}

impl ArrivalTimer {
    pub fn new(timer: Timer) -> Self {
        Self { timer }
    }

    pub fn elapsed(&self) -> Duration {
        self.timer.elapsed()
    }

    pub fn total(&self) -> Duration {
        self.timer.duration()
    }

    pub fn advance(&mut self, time_passed: Duration) {
        self.timer.tick(time_passed);
    }
}

/// Returns a new position shifted away from a starting position in a given direction
pub fn set_destination(
    current_position: &Transform,
    tile_dimensions: &PxDimensions,
    direction: &MovementDirection,
) -> Transform {
    let current_px_position = current_position.translation;
    match direction {
        MovementDirection::Left => Transform::from_xyz(
            current_px_position.x - tile_dimensions.get_width() as f32,
            current_px_position.y,
            current_px_position.z,
        ),
        MovementDirection::Right => Transform::from_xyz(
            current_px_position.x + tile_dimensions.get_width() as f32,
            current_px_position.y,
            current_px_position.z,
        ),
        MovementDirection::Up => Transform::from_xyz(
            current_px_position.x,
            current_px_position.y + tile_dimensions.get_height() as f32,
            current_px_position.z,
        ),
        MovementDirection::Down => Transform::from_xyz(
            current_px_position.x,
            current_px_position.y - tile_dimensions.get_height() as f32,
            current_px_position.z,
        ),
    }
}

/// Sets the target location of the player on the map.
pub fn set_player_target(
    mut requests_to_move: EventReader<MovementDirection>,
    mut commands: Commands,
    player: Query<
        (Entity, &PxDimensions, &Transform),
        (With<Player>, Without<Target>, Without<ArrivalTimer>),
    >,
    arrival_time: Res<ArrivalTime>,
) {
    if player.is_empty() {
        return;
    }

    if requests_to_move.is_empty() {
        return;
    }

    let (player_entity, player_tile_dimensions, current_player_position) = player.single();
    let direction = requests_to_move
        .read()
        .next()
        .expect("set_player_target: There are no requests to move.");

    let new_target_position =
        set_destination(current_player_position, player_tile_dimensions, direction);
    let starting_position = StartingPosition::new(*current_player_position);
    let new_target = Target::new(new_target_position);

    let timer = Timer::new(*arrival_time.get_duration(), TimerMode::Once);
    let arrival_timer = ArrivalTimer::new(timer);

    commands
        .entity(player_entity)
        .insert((starting_position, new_target, arrival_timer));
}

/// Returns a direction for some starting and target position.
fn get_direction(position: &Transform, target: &Target) -> MovementDirection {
    let x_difference = target.get_position().translation.x - position.translation.x;
    let y_difference = target.get_position().translation.y - position.translation.y;

    if x_difference != 0.0 {
        return match x_difference.is_sign_positive() {
            true => MovementDirection::Right,
            false => MovementDirection::Left,
        };
    }

    if y_difference != 0.0 {
        return match y_difference.is_sign_positive() {
            true => MovementDirection::Up,
            false => MovementDirection::Down,
        };
    }

    panic!("get_direction: There's no difference in the starting and ending position.");
}

/// Returns the current distance relative to the current time elapsed.
///
/// This calculates the following ratio:
///
/// (total_distance * elapsed_time) / total_time = current_distance
fn calculate_current_distance(
    total_distance: usize,
    time_to_reach_destination: &ArrivalTimer,
) -> f32 {
    let elapsed_time = time_to_reach_destination.elapsed();
    let total_time = time_to_reach_destination.total();

    let current_distance = if total_time.is_zero() || time_to_reach_destination.timer.finished() {
        total_distance as f32
    } else {
        (total_distance as f32 * elapsed_time.as_secs_f32()) / total_time.as_secs_f32()
    };

    current_distance
}

/// Moves some entities's position towards a target in a given amount of time.
fn move_towards(
    starting_position: &Transform,
    target: &Target,
    distance: &PxDimensions,
    time_to_reach_destination: &ArrivalTimer,
) -> Transform {
    let mut new_position = *starting_position;

    let direction_facing = get_direction(starting_position, target);

    match direction_facing {
        MovementDirection::Left => {
            let new_position_x =
                -calculate_current_distance(distance.get_width(), time_to_reach_destination);
            new_position.translation.x += new_position_x;
        }
        MovementDirection::Right => {
            let new_position_x =
                calculate_current_distance(distance.get_width(), time_to_reach_destination);
            new_position.translation.x += new_position_x;
        }
        MovementDirection::Up => {
            let new_position_y =
                calculate_current_distance(distance.get_height(), time_to_reach_destination);
            new_position.translation.y += new_position_y;
        }
        MovementDirection::Down => {
            let new_position_y =
                -calculate_current_distance(distance.get_height(), time_to_reach_destination);
            new_position.translation.y += new_position_y;
        }
    }

    new_position
}

/// Moves some entity towards a Target position.
pub fn move_entity_to_target(
    mut movable_entities: Query<(
        Entity,
        &mut Transform,
        &StartingPosition,
        &PxDimensions,
        &Target,
        &mut ArrivalTimer,
    )>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (
        entity,
        mut entity_position,
        entity_starting_position,
        entity_dimensions,
        entity_target,
        mut time_to_reach_destination,
    ) in &mut movable_entities
    {
        if entity_position.as_ref() == entity_target.get_position() {
            commands.entity(entity).remove::<Target>();
            commands.entity(entity).remove::<ArrivalTimer>();
            commands.entity(entity).remove::<StartingPosition>();
            continue;
        }

        time_to_reach_destination.advance(time.delta());

        *entity_position = move_towards(
            entity_starting_position.get_position(),
            entity_target,
            entity_dimensions,
            time_to_reach_destination.as_ref(),
        );

        if time_to_reach_destination.timer.finished() {
            *entity_position = *entity_target.get_position();
        }
    }
}

pub fn move_player_on_key_press(
    input: Res<ButtonInput<KeyCode>>,
    mut move_player_requester: EventWriter<MovementDirection>,
) {
    if input.pressed(KeyCode::KeyW) {
        move_player_requester.send(MovementDirection::Up);
    } else if input.pressed(KeyCode::KeyS) {
        move_player_requester.send(MovementDirection::Down);
    } else if input.pressed(KeyCode::KeyA) {
        move_player_requester.send(MovementDirection::Left);
    } else if input.pressed(KeyCode::KeyD) {
        move_player_requester.send(MovementDirection::Right);
    }
}

pub fn update_level_dimensions(
    level_query: Query<&LevelIid, Changed<LevelIid>>,
    projects: Query<&Handle<LdtkProject>>,
    project_assets: Res<Assets<LdtkProject>>,
    mut level_dimension: ResMut<LevelDimensions>,
) {
    if project_assets.is_empty() || level_query.is_empty() {
        return;
    }

    let level_id = level_query.single();
    let level_project = project_assets
        .get(projects.single())
        .expect("update_level_dimensions: Could not find project for map. Is it loaded?");

    let level_info = level_project
        .as_standalone()
        .get_loaded_level_by_iid(level_id.get())
        .expect(
            "update_level_dimensions: Could not find Loaded Level in project. Is the map loaded?",
        );

    let level_height = *level_info.px_hei() as usize;
    let level_width = *level_info.px_wid() as usize;

    level_dimension.width = level_width;
    level_dimension.height = level_height;
}

pub fn bound_player_movement(
    level_dimension: Res<LevelDimensions>,
    mut player_query: Query<&mut Transform, (Changed<Transform>, With<Player>)>,
) {
    if player_query.is_empty() {
        return;
    }

    if level_dimension.height == 0 || level_dimension.width == 0 {
        return;
    }

    let mut player_transform = player_query
        .get_single_mut()
        .expect("bound_player_movement: The player does not exist, but they should");

    let tile_side_length = 64.0;
    let tile_mid_point = tile_side_length / 2.0;

    player_transform.translation.x = player_transform.translation.x.clamp(
        tile_mid_point,
        level_dimension.width as f32 - tile_mid_point,
    );

    player_transform.translation.y = player_transform.translation.y.clamp(
        tile_mid_point,
        level_dimension.height as f32 - tile_mid_point,
    );
}

pub fn animate_entity(
    mut entity_query: Query<(&mut TextureAtlas, &DirectionFacing), Changed<DirectionFacing>>,
) {
    if entity_query.is_empty() {
        return;
    }

    for (mut sprite, facing) in entity_query.iter_mut() {
        match facing {
            DirectionFacing::Up => {
                sprite.index = 0;
            }
            DirectionFacing::Down => {
                sprite.index = 1;
            }
            DirectionFacing::Left => {
                sprite.index = 2;
            }
            DirectionFacing::Right => {
                sprite.index = 3;
            }
        }
    }
}

pub fn move_entity(
    mut entity_query: Query<
        (&mut Transform, &DirectionFacing, &mut MovementIntent),
        Changed<MovementIntent>,
    >,
    tile_query: Query<&EntityInstance>,
    level_dimension: Res<LevelDimensions>,
    mut entity_movement_broadcast: EventWriter<PlayerMovementActions>,
) {
    if entity_query.is_empty() {
        return;
    }

    let collision_tiles = tile_query
        .iter()
        .filter(|&tile| !tile.field_instances.is_empty())
        .filter(|&tile| {
            tile.field_instances
                .iter()
                .any(|field_instance| field_instance.identifier == "Traversable")
        })
        .collect::<Vec<&EntityInstance>>();

    for (mut entity_transform, facing, mut moving) in entity_query.iter_mut() {
        let pixel_distance = 3.0;
        let mut direction = Vec3::ZERO;

        if *moving != MovementIntent::Moving {
            return;
        }

        match facing {
            DirectionFacing::Up => {
                direction += Vec3::new(0.0, pixel_distance, 0.0);
            }
            DirectionFacing::Down => {
                direction -= Vec3::new(0.0, pixel_distance, 0.0);
            }
            DirectionFacing::Left => {
                direction -= Vec3::new(pixel_distance, 0.0, 0.0);
            }
            DirectionFacing::Right => {
                direction += Vec3::new(pixel_distance, 0.0, 0.0);
            }
        }

        let tile_side_length = 64.0;
        let projected_position = entity_transform.translation + direction;

        for &collision_tile in collision_tiles.iter() {
            let tile_position = Vec3::new(
                collision_tile.px.x as f32,
                (level_dimension.height as i32 - (collision_tile.px.y)) as f32,
                0.0,
            );

            let projected_dimensions = Vec2::new(tile_side_length, tile_side_length);
            let tile_dimensions =
                Vec2::new(collision_tile.width as f32, collision_tile.height as f32);

            let has_collided =
                Aabb2d::new(projected_position.truncate(), projected_dimensions / 2.0).intersects(
                    &Aabb2d::new(tile_position.truncate(), tile_dimensions / 2.0),
                );

            if has_collided {
                entity_movement_broadcast.send(PlayerMovementActions::Bumping);
                *moving = MovementIntent::Idle;
                return;
            }
        }

        entity_transform.translation = projected_position;
        entity_movement_broadcast.send(PlayerMovementActions::Walking);
        *moving = MovementIntent::Idle;
    }
}

pub fn interact_entity(
    input: Res<ButtonInput<KeyCode>>,
    tile_query: Query<&EntityInstance>,
    player_query: Query<(&Transform, &DirectionFacing), With<Player>>,
    level_dimension: Res<LevelDimensions>,
    mut interactible_event_writer: EventWriter<InteractionEvent>,
) {
    if player_query.is_empty() {
        return;
    }

    if !input.just_pressed(KeyCode::KeyE) {
        return;
    }

    let interactive_tiles = tile_query
        .iter()
        .filter(|&tile| !tile.field_instances.is_empty())
        .filter(|&tile| {
            tile.field_instances
                .iter()
                .any(|field_instance| field_instance.identifier == "Interactable")
        })
        .collect::<Vec<&EntityInstance>>();

    let (player_transform, facing) = player_query
        .get_single()
        .expect("interact_entity: The player does not exist, but they should");

    let pixel_distance = 3.0;
    let mut direction = Vec3::ZERO;

    match facing {
        DirectionFacing::Up => {
            direction += Vec3::new(0.0, pixel_distance, 0.0);
        }
        DirectionFacing::Down => {
            direction -= Vec3::new(0.0, pixel_distance, 0.0);
        }
        DirectionFacing::Left => {
            direction -= Vec3::new(pixel_distance, 0.0, 0.0);
        }
        DirectionFacing::Right => {
            direction += Vec3::new(pixel_distance, 0.0, 0.0);
        }
    }

    let tile_side_length = 64.0;
    let projected_position = player_transform.translation + direction;

    for &interactive_tile in interactive_tiles.iter() {
        let tile_position = Vec3::new(
            interactive_tile.px.x as f32,
            (level_dimension.height as i32 - (interactive_tile.px.y)) as f32,
            0.0,
        );

        let projected_dimensions = Vec2::new(tile_side_length, tile_side_length);
        let tile_dimensions = Vec2::new(
            interactive_tile.width as f32,
            interactive_tile.height as f32,
        );

        let has_collided =
            Aabb2d::new(projected_position.truncate(), projected_dimensions / 2.0).intersects(
                &Aabb2d::new(tile_position.truncate(), tile_dimensions / 2.0),
            );

        if has_collided {
            let text = interactive_tile.field_instances().get(1).expect(
                "interact_entity: Could not find Interactive command text in Interactive Tile",
            );

            if let StringType(message) = &text.value {
                let raw_string = message
                    .as_ref()
                    .expect("interact_entity: Could not display message");
                let split_string: Vec<&str> = raw_string.split(':').collect();

                let command = split_string[0];
                let arg = split_string[1];

                interactible_event_writer
                    .send(InteractionEvent(command.to_string(), arg.to_string()));
            }
        }
    }
}

pub fn display_interactive_message(mut interactible_event_reader: EventReader<InteractionEvent>) {
    for interaction_command in interactible_event_reader.read() {
        let command = &interaction_command.0;
        if command != "message" {
            continue;
        }

        let arg = &interaction_command.1;
        println!("{}", arg);
    }
}

pub fn transition_level(
    mut interactible_event_reader: EventReader<InteractionEvent>,
    mut level: ResMut<LevelSelection>,
) {
    for interaction_command in interactible_event_reader.read() {
        let command = &interaction_command.0;
        if command != "transition" {
            continue;
        }

        let arg = &interaction_command.1;
        *level = LevelSelection::Identifier(arg.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LEVEL_WIDTH: usize = 1344;
    const TEST_LEVEL_HEIGHT: usize = 1472;

    const TEST_LEVEL_WIDTH_IN_BOUNDS: f32 = 500.0;
    const TEST_LEVEL_WIDTH_OUT_LBOUNDS: f32 = -500.0;
    const TEST_LEVEL_WIDTH_OUT_RBOUNDS: f32 = 1500.0;

    const TEST_LEVEL_HEIGHT_IN_BOUNDS: f32 = 500.0;
    const TEST_LEVEL_HEIGHT_OUT_TBOUNDS: f32 = 1600.0;
    const TEST_LEVEL_HEIGHT_OUT_BBOUNDS: f32 = -500.0;

    const PLAYER_MIDPOINT: usize = 32;

    fn setup_app_bounds_checking() -> App {
        let mut app = App::new();

        app.insert_resource(LevelDimensions {
            width: TEST_LEVEL_WIDTH,
            height: TEST_LEVEL_HEIGHT,
        });

        app.add_systems(Update, bound_player_movement);

        app
    }

    #[test]
    fn within_bounds() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn_empty()
            .insert((
                Player,
                Transform::from_xyz(TEST_LEVEL_WIDTH_IN_BOUNDS, TEST_LEVEL_HEIGHT_IN_BOUNDS, 0.0),
            ))
            .id();

        app.update();

        let player_query = app.world.get::<Transform>(player_id);
        assert!(player_query.is_some());

        let expected_transform =
            Transform::from_xyz(TEST_LEVEL_WIDTH_IN_BOUNDS, TEST_LEVEL_HEIGHT_IN_BOUNDS, 0.0);
        let actual_transform =
            *player_query.expect("within_bound [test]: Player could not be found");

        assert_eq!(expected_transform, actual_transform);
    }

    #[test]
    fn out_of_bounds_left() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn_empty()
            .insert((
                Player,
                Transform::from_xyz(
                    TEST_LEVEL_WIDTH_OUT_LBOUNDS,
                    TEST_LEVEL_HEIGHT_IN_BOUNDS,
                    0.0,
                ),
            ))
            .id();

        app.update();

        let player_query = app.world.get::<Transform>(player_id);
        assert!(player_query.is_some());

        let expected_transform =
            Transform::from_xyz(PLAYER_MIDPOINT as f32, TEST_LEVEL_HEIGHT_IN_BOUNDS, 0.0);
        let actual_transform =
            *player_query.expect("out_of_bounds_left [test]: Player could not be found");

        assert_eq!(expected_transform, actual_transform);
    }

    #[test]
    fn out_of_bounds_topleft() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn_empty()
            .insert((
                Player,
                Transform::from_xyz(
                    TEST_LEVEL_WIDTH_OUT_LBOUNDS,
                    TEST_LEVEL_HEIGHT_OUT_TBOUNDS,
                    0.0,
                ),
            ))
            .id();

        app.update();

        let player_query = app.world.get::<Transform>(player_id);
        assert!(player_query.is_some());

        let expected_transform = Transform::from_xyz(
            PLAYER_MIDPOINT as f32,
            (TEST_LEVEL_HEIGHT - PLAYER_MIDPOINT) as f32,
            0.0,
        );
        let actual_transform =
            *player_query.expect("out_of_bounds_topleft [test]: Player could not be found");

        assert_eq!(expected_transform, actual_transform);
    }

    #[test]
    fn out_of_bounds_bottomleft() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn_empty()
            .insert((
                Player,
                Transform::from_xyz(
                    TEST_LEVEL_WIDTH_OUT_LBOUNDS,
                    TEST_LEVEL_HEIGHT_OUT_BBOUNDS,
                    0.0,
                ),
            ))
            .id();

        app.update();

        let player_query = app.world.get::<Transform>(player_id);
        assert!(player_query.is_some());

        let expected_transform =
            Transform::from_xyz(PLAYER_MIDPOINT as f32, PLAYER_MIDPOINT as f32, 0.0);
        let actual_transform =
            *player_query.expect("out_of_bounds_bottomleft [test]: Player could not be found");

        assert_eq!(expected_transform, actual_transform);
    }

    #[test]
    fn out_of_bounds_right() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn_empty()
            .insert((
                Player,
                Transform::from_xyz(
                    TEST_LEVEL_WIDTH_OUT_RBOUNDS,
                    TEST_LEVEL_HEIGHT_IN_BOUNDS,
                    0.0,
                ),
            ))
            .id();

        app.update();

        let player_query = app.world.get::<Transform>(player_id);
        assert!(player_query.is_some());

        let expected_transform = Transform::from_xyz(
            (TEST_LEVEL_WIDTH - PLAYER_MIDPOINT) as f32,
            TEST_LEVEL_HEIGHT_IN_BOUNDS,
            0.0,
        );
        let actual_transform =
            *player_query.expect("out_of_bounds_right [test]: Player could not be found");

        assert_eq!(expected_transform, actual_transform);
    }

    #[test]
    fn out_of_bounds_topright() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn_empty()
            .insert((
                Player,
                Transform::from_xyz(
                    TEST_LEVEL_WIDTH_OUT_RBOUNDS,
                    TEST_LEVEL_HEIGHT_OUT_TBOUNDS,
                    0.0,
                ),
            ))
            .id();

        app.update();

        let player_query = app.world.get::<Transform>(player_id);
        assert!(player_query.is_some());

        let expected_transform = Transform::from_xyz(
            (TEST_LEVEL_WIDTH - PLAYER_MIDPOINT) as f32,
            (TEST_LEVEL_HEIGHT - PLAYER_MIDPOINT) as f32,
            0.0,
        );
        let actual_transform =
            *player_query.expect("out_of_bounds_topright [test]: Player could not be found");

        assert_eq!(expected_transform, actual_transform);
    }

    #[test]
    fn out_of_bounds_bottomright() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn_empty()
            .insert((
                Player,
                Transform::from_xyz(
                    TEST_LEVEL_WIDTH_OUT_RBOUNDS,
                    TEST_LEVEL_HEIGHT_OUT_BBOUNDS,
                    0.0,
                ),
            ))
            .id();

        app.update();

        let player_query = app.world.get::<Transform>(player_id);
        assert!(player_query.is_some());

        let expected_transform = Transform::from_xyz(
            (TEST_LEVEL_WIDTH - PLAYER_MIDPOINT) as f32,
            PLAYER_MIDPOINT as f32,
            0.0,
        );
        let actual_transform =
            *player_query.expect("out_of_bounds_bottomright [test]: Player could not be found");

        assert_eq!(expected_transform, actual_transform);
    }

    #[test]
    fn out_of_bounds_top() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn_empty()
            .insert((
                Player,
                Transform::from_xyz(
                    TEST_LEVEL_WIDTH_IN_BOUNDS,
                    TEST_LEVEL_HEIGHT_OUT_TBOUNDS,
                    0.0,
                ),
            ))
            .id();

        app.update();

        let player_query = app.world.get::<Transform>(player_id);
        assert!(player_query.is_some());

        let expected_transform = Transform::from_xyz(
            TEST_LEVEL_WIDTH_IN_BOUNDS,
            (TEST_LEVEL_HEIGHT - PLAYER_MIDPOINT) as f32,
            0.0,
        );
        let actual_transform =
            *player_query.expect("out_of_bounds_top [test]: Player could not be found");

        assert_eq!(expected_transform, actual_transform);
    }

    #[test]
    fn out_of_bounds_bottom() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn_empty()
            .insert((
                Player,
                Transform::from_xyz(
                    TEST_LEVEL_WIDTH_IN_BOUNDS,
                    TEST_LEVEL_HEIGHT_OUT_BBOUNDS,
                    0.0,
                ),
            ))
            .id();

        app.update();

        let player_query = app.world.get::<Transform>(player_id);
        assert!(player_query.is_some());

        let expected_transform =
            Transform::from_xyz(TEST_LEVEL_WIDTH_IN_BOUNDS, PLAYER_MIDPOINT as f32, 0.0);
        let actual_transform =
            *player_query.expect("out_of_bounds_bottom [test]: Player could not be found");

        assert_eq!(expected_transform, actual_transform);
    }
}

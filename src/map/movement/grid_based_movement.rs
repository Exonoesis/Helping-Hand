use std::time::Duration;

use bevy::prelude::*;

use crate::map::{player::*, GridDimensions, PxDimensions, XyzCords};

use super::collision::CollisionCollection;

#[derive(Event, Copy, Clone, Debug, PartialEq)]
pub enum MovementDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
pub struct Target {
    position: Transform,
    grid_coordinate: XyzCords,
}

impl Target {
    pub fn new(px_position: Transform, grid_coordinate: XyzCords) -> Self {
        Self {
            position: px_position,
            grid_coordinate,
        }
    }

    pub fn get_position(&self) -> &Transform {
        &self.position
    }

    pub fn get_grid_coordinate(&self) -> &XyzCords {
        &self.grid_coordinate
    }
}

/// Sets the target location of the player on the map.
pub fn set_player_target(
    mut requests_to_move: EventReader<MovementDirection>,
    mut movement_notifications: EventWriter<PlayerMovementActions>,
    mut commands: Commands,
    mut player: Query<
        (
            Entity,
            &PxDimensions,
            &Transform,
            &XyzCords,
            &mut MovementDirection,
        ),
        (With<Player>, Without<Target>, Without<ArrivalTimer>),
    >,
    world: Query<(&CollisionCollection, &GridDimensions, &PxDimensions)>,
    arrival_time: Res<ArrivalTime>,
) {
    if player.is_empty() {
        return;
    }

    if requests_to_move.is_empty() {
        return;
    }

    if world.is_empty() {
        return;
    }

    let (collision_tiles, map_grid_dimensions, map_px_dimensions) = world.single();

    let (
        player_entity,
        player_tile_dimensions,
        current_player_position,
        current_player_grid_coordinate,
        mut player_direction,
    ) = player.single_mut();

    let direction = requests_to_move
        .read()
        .next()
        .expect("set_player_target: There are no requests to move.");

    *player_direction = *direction;

    let found_new_physical_position = set_physical_destination(
        current_player_position,
        player_tile_dimensions,
        map_px_dimensions,
        direction,
    );
    if found_new_physical_position.is_none() {
        movement_notifications.send(PlayerMovementActions::Bumping);
        return;
    }
    let new_physical_position = found_new_physical_position.unwrap();

    let found_new_logical_position = set_logical_destination(
        current_player_grid_coordinate,
        map_grid_dimensions,
        direction,
    );
    if found_new_logical_position.is_none() {
        movement_notifications.send(PlayerMovementActions::Bumping);
        return;
    }
    let new_logical_position = found_new_logical_position.unwrap();

    if collision_tiles.has(&new_logical_position) {
        movement_notifications.send(PlayerMovementActions::Bumping);
        return;
    }
    let starting_position = StartingPosition::new(*current_player_position);
    let new_target = Target::new(new_physical_position, new_logical_position);

    movement_notifications.send(PlayerMovementActions::Walking);
    let timer = Timer::new(*arrival_time.get_duration(), TimerMode::Once);
    let arrival_timer = ArrivalTimer::new(timer);

    commands
        .entity(player_entity)
        .insert((starting_position, new_target, arrival_timer));
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

/// Returns a new pixel position shifted away from a starting position in a given direction
pub fn set_physical_destination(
    current_position: &Transform,
    tile_dimensions: &PxDimensions,
    map_px_dimensions: &PxDimensions,
    direction: &MovementDirection,
) -> Option<Transform> {
    let current_px_position = current_position.translation;
    let mut current_x = current_px_position.x;
    let mut current_y = current_px_position.y;
    let current_z = current_px_position.z;

    let level_width = map_px_dimensions.get_width() as f32;
    let level_height = map_px_dimensions.get_height() as f32;

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

/// Returns a new grid coordinate shifted away from a starting coordinate in a given direction,
/// or None if the grid coordinate would be out of bounds
pub fn set_logical_destination(
    current_grid_coordinate: &XyzCords,
    map_grid_dimensions: &GridDimensions,
    direction: &MovementDirection,
) -> Option<XyzCords> {
    let mut current_x = current_grid_coordinate.get_x();
    let mut current_y = current_grid_coordinate.get_y();
    let current_z = current_grid_coordinate.get_z();

    let level_width = map_grid_dimensions.get_columns() as usize;
    let level_height = map_grid_dimensions.get_rows() as usize;

    match direction {
        MovementDirection::Left => {
            if current_x == 0 {
                return None;
            }
            current_x -= 1
        }
        MovementDirection::Right => {
            if current_x == level_width - 1 {
                return None;
            }
            current_x += 1
        }
        MovementDirection::Up => {
            if current_y == 0 {
                return None;
            }
            current_y -= 1
        }
        MovementDirection::Down => {
            if current_y == level_height - 1 {
                return None;
            }
            current_y += 1
        }
    }

    Some(XyzCords::new(current_x, current_y, current_z))
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

/// Moves some entity towards a Target position.
pub fn move_entity_to_target(
    mut movable_entities: Query<(
        Entity,
        &mut Transform,
        &mut XyzCords,
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
        mut entity_physical_position,
        mut entity_logical_position,
        entity_starting_position,
        entity_dimensions,
        entity_target,
        mut time_to_reach_destination,
    ) in &mut movable_entities
    {
        time_to_reach_destination.advance(time.delta());

        if time_to_reach_destination.timer.finished() {
            *entity_physical_position = *entity_target.get_position();
            *entity_logical_position = *entity_target.get_grid_coordinate();

            commands.entity(entity).remove::<Target>();
            commands.entity(entity).remove::<ArrivalTimer>();
            commands.entity(entity).remove::<StartingPosition>();
            continue;
        }

        *entity_physical_position = move_towards(
            entity_starting_position.get_position(),
            entity_target,
            entity_dimensions,
            time_to_reach_destination.as_ref(),
        );
    }
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

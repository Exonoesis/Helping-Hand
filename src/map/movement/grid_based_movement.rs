use super::collision::CollisionCollection;
use crate::map::{player::*, GridCords3D, GridDimensions, PxDimensions};
use bevy::{
    ecs::query::{QueryData, QueryFilter},
    prelude::*,
};
use std::time::Duration;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct PlayerInformation<'world> {
    entity: Entity,
    size: &'world PxDimensions,
    physical_position: &'world Transform,
    grid_position: &'world GridCords3D,
    face_direction: &'world mut MovementDirection,
}

#[derive(QueryFilter)]
pub struct IdlePlayerIdentifiers {
    filters: (With<Player>, Without<Target>, Without<ArrivalTimer>),
}

#[derive(QueryData)]
pub struct WorldCollisionLocations<'world> {
    map_collision_locations: &'world CollisionCollection,
    map_grid_dimensions: &'world GridDimensions,
    map_pixel_dimensions: &'world PxDimensions,
}

struct MovementPotential<'world> {
    movement_direction: &'world MovementDirection,
    world_collision: &'world CollisionCollection,
}

impl<'world> MovementPotential<'world> {
    pub fn new(
        movement_direction: &'world MovementDirection,
        map_info: &'world WorldCollisionLocationsItem,
    ) -> Self {
        let world_collision = map_info.map_collision_locations;

        Self {
            movement_direction,
            world_collision,
        }
    }
}

struct PlayerToMove<'world> {
    pixel_position: &'world Transform,
    grid_coordinates: &'world GridCords3D,
}

impl<'world> PlayerToMove<'world> {
    pub fn new(player: &'world PlayerInformationItem) -> Self {
        let pixel_position = player.physical_position;
        let grid_coordinates = player.grid_position;

        Self {
            pixel_position,
            grid_coordinates,
        }
    }
}

struct WorldSpace<'world> {
    grid_dimensions: &'world GridDimensions,
}

impl<'world> WorldSpace<'world> {
    pub fn new(world_size: &'world WorldCollisionLocationsItem) -> Self {
        let grid_dimensions = world_size.map_grid_dimensions;
        Self { grid_dimensions }
    }
}

struct PositionalData<'world> {
    player_to_move: PlayerToMove<'world>,
    world_space: WorldSpace<'world>,
}

impl<'world> PositionalData<'world> {
    pub fn new(player_to_move: PlayerToMove<'world>, world_space: WorldSpace<'world>) -> Self {
        Self {
            player_to_move,
            world_space,
        }
    }
}

struct ReferencedPlayerToMove<'world> {
    entity: Entity,
    player_to_move: PlayerToMove<'world>,
    new_position: GridCords3D,
}

impl<'world> ReferencedPlayerToMove<'world> {
    pub fn new(player: &'world PlayerInformationItem, new_position: GridCords3D) -> Self {
        let entity = player.entity;
        let player_to_move = PlayerToMove::new(&player);

        Self {
            entity,
            player_to_move,
            new_position,
        }
    }
}

struct MovementSetter<'world> {
    arrival_time: Res<'world, ArrivalTime>,
}

impl<'world> MovementSetter<'world> {
    pub fn new(arrival_time: Res<'world, ArrivalTime>) -> Self {
        Self { arrival_time }
    }
}

//
// Pre-refactor Structs
//

#[derive(Message, Copy, Clone, Debug, PartialEq, Component)]
pub enum MovementDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
pub struct Target {
    position: GridCords3D,
}

impl Target {
    pub fn new(position: GridCords3D) -> Self {
        Self { position }
    }

    pub fn get_grid_coordinate(&self) -> &GridCords3D {
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

/// Sends a request to move the player the direction corresponding to the key pressed
pub fn move_player_on_key_press(
    input: Res<ButtonInput<KeyCode>>,
    mut move_player_requester: MessageWriter<MovementDirection>,
) {
    if input.pressed(KeyCode::KeyW) {
        move_player_requester.write(MovementDirection::Up);
    } else if input.pressed(KeyCode::KeyS) {
        move_player_requester.write(MovementDirection::Down);
    } else if input.pressed(KeyCode::KeyA) {
        move_player_requester.write(MovementDirection::Left);
    } else if input.pressed(KeyCode::KeyD) {
        move_player_requester.write(MovementDirection::Right);
    }
}

/// Sets the target location of the player on the map.
pub fn set_player_target(
    mut requests_to_move: MessageReader<MovementDirection>,
    mut movement_notifications: MessageWriter<PlayerMovementActions>,
    commands: Commands,
    mut player: Single<PlayerInformation, IdlePlayerIdentifiers>,
    world: Single<WorldCollisionLocations>,
    arrival_time: Res<ArrivalTime>,
) {
    if !time_to_move(&requests_to_move) {
        return;
    }

    let movement_direction = requests_to_move.read().next().unwrap();
    *player.face_direction = *movement_direction;

    let player_information = &player.into_inner();
    let world_collisions_locations = &world.into_inner();

    let movement_potential =
        MovementPotential::new(&movement_direction, world_collisions_locations);
    let player_to_move = PlayerToMove::new(player_information);
    let world_space = WorldSpace::new(world_collisions_locations);
    let positional_data = PositionalData::new(player_to_move, world_space);
    // let positional_data = PositionalData::from(&player, &world)
    // -> let world_space = WorldSpace::new(&world)
    // -> let player_to_move = PlayerToMove::new(&player)

    let projected_position = able_to_move(movement_potential, positional_data);
    let is_able_to_move = projected_position.is_some();
    if !is_able_to_move {
        movement_notifications.write(PlayerMovementActions::Bumping);
        return;
    }

    let new_position = projected_position.unwrap();
    let referenced_player_to_move = ReferencedPlayerToMove::new(player_information, new_position);
    let movement_setter = MovementSetter::new(arrival_time);
    movement_notifications.write(PlayerMovementActions::Walking);
    do_the_move(referenced_player_to_move, movement_setter, commands);
}

//
// Pre-refactor Main Functions
//

/// Moves some entity towards a Target position.
pub fn move_entity_to_target(
    mut movable_entities: Query<(
        Entity,
        &mut Transform,
        &mut GridCords3D,
        &StartingPosition,
        &PxDimensions,
        &Target,
        &mut ArrivalTimer,
        &MovementDirection,
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
        movement_direction,
    ) in &mut movable_entities
    {
        time_to_reach_destination.advance(time.delta());

        if time_to_reach_destination.timer.is_finished() {
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
            *movement_direction,
        );
    }
}

// TODO: Description
fn time_to_move(requests_to_move: &MessageReader<MovementDirection>) -> bool {
    if requests_to_move.is_empty() {
        return false;
    }
    true
}

// TODO: Description
fn able_to_move(
    movement_potential: MovementPotential,
    positional_data: PositionalData,
) -> Option<GridCords3D> {
    let player = positional_data.player_to_move;
    let movement_direction = movement_potential.movement_direction;
    let world_space = positional_data.world_space;

    let found_projected_position = get_projected_position(&player, movement_direction, world_space);

    if let Some(projected_position) = found_projected_position {
        let world_collisions = movement_potential.world_collision;

        if is_going_to_collide(&projected_position, world_collisions) {
            return None;
        }
        return Some(projected_position);
    }

    None
}

// TODO: Description
fn do_the_move(
    referenced_player_to_move: ReferencedPlayerToMove,
    movement_setter: MovementSetter,
    mut commands: Commands,
) {
    let player = referenced_player_to_move.player_to_move;
    let new_position = referenced_player_to_move.new_position;
    let player_entity = referenced_player_to_move.entity;

    let starting_position = StartingPosition::new(*player.pixel_position);
    let new_target = Target::new(new_position);

    let arrival_time = movement_setter.arrival_time;
    let timer = Timer::new(*arrival_time.get_duration(), TimerMode::Once);
    let arrival_timer = ArrivalTimer::new(timer);

    commands
        .entity(player_entity)
        .insert((starting_position, new_target, arrival_timer));
}

/// Returns a new grid coordinate shifted away from a starting coordinate in a given direction
/// if valid to do so
fn get_projected_position(
    player: &PlayerToMove,
    movement_direction: &MovementDirection,
    world_space: WorldSpace,
) -> Option<GridCords3D> {
    let current_grid_coordinate = player.grid_coordinates;
    let mut current_x = current_grid_coordinate.get_x();
    let mut current_y = current_grid_coordinate.get_y();
    let current_z = current_grid_coordinate.get_z();

    if will_be_out_of_map_bounds(current_grid_coordinate, world_space, movement_direction) {
        return None;
    }

    match movement_direction {
        MovementDirection::Left => current_x -= 1,
        MovementDirection::Right => current_x += 1,
        MovementDirection::Up => current_y -= 1,
        MovementDirection::Down => current_y += 1,
    }

    Some(GridCords3D::new(current_x, current_y, current_z))
}

// TODO: Function Description
fn will_be_out_of_map_bounds(
    current_position: &GridCords3D,
    world_space: WorldSpace,
    movement_direction: &MovementDirection,
) -> bool {
    let current_x = current_position.get_x();
    let current_y = current_position.get_y();

    let level_width = world_space.grid_dimensions.get_columns() as usize;
    let level_height = world_space.grid_dimensions.get_rows() as usize;

    if current_x == 0 && *movement_direction == MovementDirection::Left {
        return true;
    }

    if current_x == level_width - 1 && *movement_direction == MovementDirection::Right {
        return true;
    }

    if current_y == 0 && *movement_direction == MovementDirection::Up {
        return true;
    }

    if current_y == level_height - 1 && *movement_direction == MovementDirection::Down {
        return true;
    }

    false
}

// TODO: Function Description
fn is_going_to_collide(
    projected_position: &GridCords3D,
    world_collisions: &CollisionCollection,
) -> bool {
    if world_collisions.has(projected_position) {
        return true;
    }
    false
}

//
// Pre-refactor Helper Functions
//

/// Moves some entities's position towards a target in a given amount of time.
fn move_towards(
    starting_position: &Transform,
    target: &Target,
    distance: &PxDimensions,
    time_to_reach_destination: &ArrivalTimer,
    movement_direction: MovementDirection,
) -> Transform {
    let mut new_position = *starting_position;

    // I think we do this because we need to know what direction this entity is supposed to be moving
    // However, the player has a facing direction we can directly access (as will NPCs)
    //let direction_facing = get_direction(starting_position, target);

    match movement_direction {
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

/*
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
*/

/// Returns the current distance relative to the current time elapsed.
///
/// This calculates the following ratio:
/// (total_distance * elapsed_time) / total_time = current_distance
fn calculate_current_distance(
    total_distance: usize,
    time_to_reach_destination: &ArrivalTimer,
) -> f32 {
    let elapsed_time = time_to_reach_destination.elapsed();
    let total_time = time_to_reach_destination.total();

    let current_distance = if total_time.is_zero() || time_to_reach_destination.timer.is_finished()
    {
        total_distance as f32
    } else {
        (total_distance as f32 * elapsed_time.as_secs_f32()) / total_time.as_secs_f32()
    };

    current_distance
}

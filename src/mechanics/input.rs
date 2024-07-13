use crate::entities::player::MovementIntent;
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

pub fn player_input(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut DirectionFacing, &mut MovementIntent), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut facing, mut moving) = player_query.single_mut();

    if input.pressed(KeyCode::KeyW) {
        *facing = DirectionFacing::Up;
        *moving = MovementIntent::Moving;
    } else if input.pressed(KeyCode::KeyS) {
        *facing = DirectionFacing::Down;
        *moving = MovementIntent::Moving;
    } else if input.pressed(KeyCode::KeyA) {
        *facing = DirectionFacing::Left;
        *moving = MovementIntent::Moving;
    } else if input.pressed(KeyCode::KeyD) {
        *facing = DirectionFacing::Right;
        *moving = MovementIntent::Moving;
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

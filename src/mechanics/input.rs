use crate::FieldValue::String;
use crate::{
    entities::player::{DirectionFacing, Player, PlayerMovementActions},
    visuals::map::LevelDimensions,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_ecs_ldtk::{prelude::LdtkFields, EntityInstance, LdtkLevel};

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct MovementIntent;

pub fn player_input(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(Entity, &mut DirectionFacing), With<Player>>,
    mut commands: Commands,
) {
    if player_query.is_empty() {
        return;
    }

    let (entity, mut facing) = player_query.single_mut();

    if input.pressed(KeyCode::W) {
        *facing = DirectionFacing::Up;
        commands.entity(entity).insert(MovementIntent);
    } else if input.pressed(KeyCode::S) {
        *facing = DirectionFacing::Down;
        commands.entity(entity).insert(MovementIntent);
    } else if input.pressed(KeyCode::A) {
        *facing = DirectionFacing::Left;
        commands.entity(entity).insert(MovementIntent);
    } else if input.pressed(KeyCode::D) {
        *facing = DirectionFacing::Right;
        commands.entity(entity).insert(MovementIntent);
    }
}

pub fn update_level_dimensions(
    level_query: Query<&Handle<LdtkLevel>>,
    loaded_levels: Res<Assets<LdtkLevel>>,
    mut level_dimension: ResMut<LevelDimensions>,
) {
    if !(loaded_levels.is_added() || loaded_levels.is_changed()) {
        return;
    }

    if loaded_levels.is_empty() || level_query.is_empty() {
        return;
    }

    let level_info = &loaded_levels
        .get(level_query.single())
        .expect("The level should exist by now.")
        .level;
    let level_height = level_info.px_hei as usize;
    let level_width = level_info.px_wid as usize;

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

    let mut player_transform = player_query.get_single_mut().expect("bound_player_movement: The player does not exist, but they should");

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
    mut entity_query: Query<(&mut TextureAtlasSprite, &DirectionFacing), Changed<DirectionFacing>>,
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
    mut entity_query: Query<(Entity, &mut Transform, &DirectionFacing), Added<MovementIntent>>,
    tile_query: Query<&EntityInstance>,
    level_dimension: Res<LevelDimensions>,
    mut entity_movement_broadcast: EventWriter<PlayerMovementActions>,
    mut commands: Commands,
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

    for (entity, mut entity_transform, facing) in entity_query.iter_mut() {
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
        let projected_position = entity_transform.translation + direction;

        for &collision_tile in collision_tiles.iter() {
            let tile_position = Vec3::new(
                collision_tile.px.x as f32,
                (level_dimension.height as i32 - (collision_tile.px.y)) as f32,
                0.0,
            );

            if collide(
                projected_position,
                Vec2::new(tile_side_length, tile_side_length),
                tile_position,
                Vec2::new(collision_tile.width as f32, collision_tile.height as f32),
            )
            .is_some()
            {
                entity_movement_broadcast.send(PlayerMovementActions::Bumping);
                commands.entity(entity).remove::<MovementIntent>();
                return;
            }
        }

        entity_transform.translation = projected_position;
        entity_movement_broadcast.send(PlayerMovementActions::Walking);
        commands.entity(entity).remove::<MovementIntent>();
    }
}

pub fn interact_entity(
    input: Res<Input<KeyCode>>,
    tile_query: Query<&EntityInstance>,
    player_query: Query<(&Transform, &DirectionFacing), With<Player>>,
    level_dimension: Res<LevelDimensions>,
) {
    if player_query.is_empty() {
        return;
    }

    if !input.just_pressed(KeyCode::E) {
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

        if collide(
            projected_position,
            Vec2::new(tile_side_length, tile_side_length),
            tile_position,
            Vec2::new(
                interactive_tile.width as f32,
                interactive_tile.height as f32,
            ),
        )
        .is_some()
        {
            let text = interactive_tile
                .field_instances()
                .get(1)
                .expect("interact_entity: Could not find Interactive text in Interactive Tile");

            if let String(message) = &text.value {
                println!("{}", message.as_ref().expect("interact_entity: Could not display message"));
            }
        }
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
        let actual_transform = *player_query.expect("within_bound [test]: Player could not be found");

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
        let actual_transform = *player_query.expect("out_of_bounds_left [test]: Player could not be found");

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
        let actual_transform = *player_query.expect("out_of_bounds_topleft [test]: Player could not be found");

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
        let actual_transform = *player_query.expect("out_of_bounds_bottomleft [test]: Player could not be found");

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
        let actual_transform = *player_query.expect("out_of_bounds_right [test]: Player could not be found");

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
        let actual_transform = *player_query.expect("out_of_bounds_topright [test]: Player could not be found");

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
        let actual_transform = *player_query.expect("out_of_bounds_bottomright [test]: Player could not be found");

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
        let actual_transform = *player_query.expect("out_of_bounds_top [test]: Player could not be found");

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
        let actual_transform = *player_query.expect("out_of_bounds_bottom [test]: Player could not be found");

        assert_eq!(expected_transform, actual_transform);
    }
}

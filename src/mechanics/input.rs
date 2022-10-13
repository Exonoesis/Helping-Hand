use crate::{
    entities::player::{Player, PlayerMovementActions},
    LevelDimensions,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_ecs_ldtk::{EntityInstance, LdtkLevel};

pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}

pub fn player_input(input: Res<Input<KeyCode>>, mut input_broadcast: EventWriter<Movement>) {
    if input.pressed(KeyCode::W) {
        input_broadcast.send(Movement::Up);
    } else if input.pressed(KeyCode::S) {
        input_broadcast.send(Movement::Down);
    } else if input.pressed(KeyCode::A) {
        input_broadcast.send(Movement::Left);
    } else if input.pressed(KeyCode::D) {
        input_broadcast.send(Movement::Right);
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

    let mut player_transform = player_query.get_single_mut().unwrap();

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

pub fn move_player(
    mut input_receiver: EventReader<Movement>,
    mut player_query: Query<(&mut Transform, &mut TextureAtlasSprite), With<Player>>,
    tile_query: Query<&EntityInstance>,
    level_dimension: Res<LevelDimensions>,
    mut player_movement_broadcast: EventWriter<PlayerMovementActions>,
) {
    for movement_action in input_receiver.iter() {
        let (mut player_transform, mut sprite) = player_query.single_mut();

        let pixel_distance = 3.0;
        let mut direction = Vec3::ZERO;
        match movement_action {
            Movement::Up => {
                direction += Vec3::new(0.0, pixel_distance, 0.0);
                sprite.index = 0;
            }
            Movement::Down => {
                direction -= Vec3::new(0.0, pixel_distance, 0.0);
                sprite.index = 1;
            }
            Movement::Left => {
                direction -= Vec3::new(pixel_distance, 0.0, 0.0);
                sprite.index = 2;
            }
            Movement::Right => {
                direction += Vec3::new(pixel_distance, 0.0, 0.0);
                sprite.index = 3;
            }
        }

        let tile_side_length = 64.0;

        let projected_position = player_transform.translation + direction;

        let collision_tiles = tile_query
            .iter()
            .filter(|&tile| !tile.field_instances.is_empty())
            .filter(|&tile| {
                tile.field_instances
                    .iter()
                    .any(|field_instance| field_instance.identifier == "Traversable")
            })
            .collect::<Vec<&EntityInstance>>();

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
                player_movement_broadcast.send(PlayerMovementActions::Bumping);
                return;
            }
        }
        player_transform.translation = projected_position;
        player_movement_broadcast.send(PlayerMovementActions::Walking);
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

        app.add_system(bound_player_movement);

        app
    }

    #[test]
    fn within_bounds() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn()
            .insert_bundle((
                Player,
                Transform::from_xyz(TEST_LEVEL_WIDTH_IN_BOUNDS, TEST_LEVEL_HEIGHT_IN_BOUNDS, 0.0),
            ))
            .id();

        app.update();

        let player_query = app.world.get::<Transform>(player_id);
        assert!(player_query.is_some());

        let expected_transform =
            Transform::from_xyz(TEST_LEVEL_WIDTH_IN_BOUNDS, TEST_LEVEL_HEIGHT_IN_BOUNDS, 0.0);
        let actual_transform = *player_query.unwrap();

        assert_eq!(expected_transform, actual_transform);
    }

    #[test]
    fn out_of_bounds_left() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn()
            .insert_bundle((
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
        let actual_transform = *player_query.unwrap();

        assert_eq!(expected_transform, actual_transform);
    }

    #[test]
    fn out_of_bounds_topleft() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn()
            .insert_bundle((
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
        let actual_transform = *player_query.unwrap();

        assert_eq!(expected_transform, actual_transform);
    }

    #[test]
    fn out_of_bounds_bottomleft() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn()
            .insert_bundle((
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
        let actual_transform = *player_query.unwrap();

        assert_eq!(expected_transform, actual_transform);
    }

    #[test]
    fn out_of_bounds_right() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn()
            .insert_bundle((
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
        let actual_transform = *player_query.unwrap();

        assert_eq!(expected_transform, actual_transform);
    }

    #[test]
    fn out_of_bounds_topright() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn()
            .insert_bundle((
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
        let actual_transform = *player_query.unwrap();

        assert_eq!(expected_transform, actual_transform);
    }

    #[test]
    fn out_of_bounds_bottomright() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn()
            .insert_bundle((
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
        let actual_transform = *player_query.unwrap();

        assert_eq!(expected_transform, actual_transform);
    }

    #[test]
    fn out_of_bounds_top() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn()
            .insert_bundle((
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
        let actual_transform = *player_query.unwrap();

        assert_eq!(expected_transform, actual_transform);
    }

    #[test]
    fn out_of_bounds_bottom() {
        let mut app = setup_app_bounds_checking();

        let player_id = app
            .world
            .spawn()
            .insert_bundle((
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
        let actual_transform = *player_query.unwrap();

        assert_eq!(expected_transform, actual_transform);
    }
}

use crate::{entities::player::Player, LevelDimensions};
use bevy::prelude::*;

pub fn move_camera(
    level_dimension: Res<LevelDimensions>,
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut camera_query: Query<
        (&mut Transform, &OrthographicProjection),
        (With<Camera2d>, Without<Player>),
    >,
) {
    if camera_query.is_empty() {
        return;
    }

    if player_query.is_empty() {
        return;
    }

    if level_dimension.height == 0 || level_dimension.width == 0 {
        return;
    }

    let (mut camera_transform, camera_bounds) = camera_query.get_single_mut().unwrap();
    let player_transform = player_query.get_single().unwrap();

    let camera_width = (camera_bounds.right - camera_bounds.left).abs() + 1.0;
    let camera_height = (camera_bounds.top - camera_bounds.bottom).abs() + 1.0;

    if camera_width > level_dimension.width as f32 {
        camera_transform.translation.x = level_dimension.width as f32 / 2.0;
    } else {
        camera_transform.translation.x = player_transform.translation.x.clamp(
            camera_width / 2.0,
            level_dimension.width as f32 - (camera_width / 2.0),
        );
    }

    if camera_height > level_dimension.height as f32 {
        camera_transform.translation.y = level_dimension.height as f32 / 2.0;
    } else {
        camera_transform.translation.y = player_transform.translation.y.clamp(
            camera_height / 2.0,
            level_dimension.height as f32 - (camera_height / 2.0),
        );
    }
}

pub fn update_camera_on_resolution_change(
    camera_query: Query<
        &OrthographicProjection,
        (
            With<Camera2d>,
            Without<Player>,
            Changed<OrthographicProjection>,
        ),
    >,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if camera_query.is_empty() {
        return;
    }

    if player_query.is_empty() {
        return;
    }

    let mut player_position = player_query.get_single_mut().unwrap();

    //Camera updates its position based on changes to player position, thus we add 0 to force a change to player position
    player_position.translation.x += 0.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LEVEL_WIDTH: usize = 500;
    const TEST_LEVEL_HEIGHT: usize = 500;

    const CAMERA_HEIGHT: usize = 100;
    const CAMERA_WIDTH: usize = 100;

    const CAMERA_MIDPOINT: usize = 50;

    const TEST_LEVEL_WIDTH_IN_BOUNDS: f32 = 250.0;
    const TEST_LEVEL_WIDTH_OUT_LBOUNDS: f32 = -500.0;
    const TEST_LEVEL_WIDTH_OUT_RBOUNDS: f32 = 1000.0;

    const TEST_LEVEL_HEIGHT_IN_BOUNDS: f32 = 250.0;
    const TEST_LEVEL_HEIGHT_OUT_TBOUNDS: f32 = 1000.0;
    const TEST_LEVEL_HEIGHT_OUT_BBOUNDS: f32 = -500.0;

    fn setup_app_bounds_checking() -> App {
        let mut app = App::new();

        app.insert_resource(LevelDimensions {
            width: TEST_LEVEL_WIDTH,
            height: TEST_LEVEL_HEIGHT,
        });

        app.add_system(move_camera);

        app
    }

    fn spawn_camera(app: &mut App) -> Entity {
        let camera_id = app
            .world
            .spawn()
            .insert_bundle(Camera2dBundle {
                projection: OrthographicProjection {
                    left: 0.0,
                    right: (CAMERA_WIDTH - 1) as f32,
                    bottom: 0.0,
                    top: (CAMERA_HEIGHT - 1) as f32,
                    ..default()
                },
                ..default()
            })
            .id();

        camera_id
    }

    #[test]
    fn within_bounds() {
        let mut app = setup_app_bounds_checking();

        // The camera's position is dependent off of the Player's position whenever it changes, so we need to
        // spawn the Player to trigger the camera to move.
        app.world.spawn().insert_bundle((
            Player,
            Transform::from_xyz(TEST_LEVEL_WIDTH_IN_BOUNDS, TEST_LEVEL_HEIGHT_IN_BOUNDS, 0.0),
        ));

        let camera_id = spawn_camera(&mut app);

        app.update();

        let camera_query = app.world.get::<Transform>(camera_id);
        assert!(camera_query.is_some());

        let expected_transform_x = TEST_LEVEL_WIDTH_IN_BOUNDS;
        let expected_transform_y = TEST_LEVEL_HEIGHT_IN_BOUNDS;

        let actual_transform = *camera_query.unwrap();
        let actual_transform_x = actual_transform.translation.x;
        let actual_transform_y = actual_transform.translation.y;

        assert_eq!(expected_transform_x, actual_transform_x);
        assert_eq!(expected_transform_y, actual_transform_y);
    }

    #[test]
    fn out_of_bounds_left() {
        let mut app = setup_app_bounds_checking();

        // The camera's position is dependent off of the Player's position whenever it changes, so we need to
        // spawn the Player to trigger the camera to move.
        app.world.spawn().insert_bundle((
            Player,
            Transform::from_xyz(
                TEST_LEVEL_WIDTH_OUT_LBOUNDS,
                TEST_LEVEL_HEIGHT_IN_BOUNDS,
                0.0,
            ),
        ));

        let camera_id = spawn_camera(&mut app);

        app.update();

        let camera_query = app.world.get::<Transform>(camera_id);
        assert!(camera_query.is_some());

        let expected_transform_x = CAMERA_MIDPOINT as f32;
        let expected_transform_y = TEST_LEVEL_HEIGHT_IN_BOUNDS;

        let actual_transform = *camera_query.unwrap();

        let actual_transform_x = actual_transform.translation.x;
        let actual_transform_y = actual_transform.translation.y;

        assert_eq!(expected_transform_x, actual_transform_x);
        assert_eq!(expected_transform_y, actual_transform_y);
    }

    #[test]
    fn out_of_bounds_topleft() {
        let mut app = setup_app_bounds_checking();

        // The camera's position is dependent off of the Player's position whenever it changes, so we need to
        // spawn the Player to trigger the camera to move.
        app.world.spawn().insert_bundle((
            Player,
            Transform::from_xyz(
                TEST_LEVEL_WIDTH_OUT_LBOUNDS,
                TEST_LEVEL_HEIGHT_OUT_TBOUNDS,
                0.0,
            ),
        ));

        let camera_id = spawn_camera(&mut app);

        app.update();

        let camera_query = app.world.get::<Transform>(camera_id);
        assert!(camera_query.is_some());

        let expected_transform_x = CAMERA_MIDPOINT as f32;
        let expected_transform_y = (TEST_LEVEL_HEIGHT - CAMERA_MIDPOINT) as f32;

        let actual_transform = *camera_query.unwrap();
        let actual_transform_x = actual_transform.translation.x;
        let actual_transform_y = actual_transform.translation.y;

        assert_eq!(expected_transform_x, actual_transform_x);
        assert_eq!(expected_transform_y, actual_transform_y);
    }

    #[test]
    fn out_of_bounds_bottomleft() {
        let mut app = setup_app_bounds_checking();

        // The camera's position is dependent off of the Player's position whenever it changes, so we need to
        // spawn the Player to trigger the camera to move.
        app.world.spawn().insert_bundle((
            Player,
            Transform::from_xyz(
                TEST_LEVEL_WIDTH_OUT_LBOUNDS,
                TEST_LEVEL_HEIGHT_OUT_BBOUNDS,
                0.0,
            ),
        ));

        let camera_id = spawn_camera(&mut app);

        app.update();

        let camera_query = app.world.get::<Transform>(camera_id);
        assert!(camera_query.is_some());

        let expected_transform_x = CAMERA_MIDPOINT as f32;
        let expected_transform_y = CAMERA_MIDPOINT as f32;

        let actual_transform = *camera_query.unwrap();
        let actual_transform_x = actual_transform.translation.x;
        let actual_transform_y = actual_transform.translation.y;

        assert_eq!(expected_transform_x, actual_transform_x);
        assert_eq!(expected_transform_y, actual_transform_y);
    }

    #[test]
    fn out_of_bounds_right() {
        let mut app = setup_app_bounds_checking();

        // The camera's position is dependent off of the Player's position whenever it changes, so we need to
        // spawn the Player to trigger the camera to move.
        app.world.spawn().insert_bundle((
            Player,
            Transform::from_xyz(
                TEST_LEVEL_WIDTH_OUT_RBOUNDS,
                TEST_LEVEL_HEIGHT_IN_BOUNDS,
                0.0,
            ),
        ));

        let camera_id = spawn_camera(&mut app);

        app.update();

        let camera_query = app.world.get::<Transform>(camera_id);
        assert!(camera_query.is_some());

        let expected_transform_x = (TEST_LEVEL_WIDTH - CAMERA_MIDPOINT) as f32;
        let expected_transform_y = TEST_LEVEL_HEIGHT_IN_BOUNDS;

        let actual_transform = *camera_query.unwrap();
        let actual_transform_x = actual_transform.translation.x;
        let actual_transform_y = actual_transform.translation.y;

        assert_eq!(expected_transform_x, actual_transform_x);
        assert_eq!(expected_transform_y, actual_transform_y);
    }

    #[test]
    fn out_of_bounds_topright() {
        let mut app = setup_app_bounds_checking();

        // The camera's position is dependent off of the Player's position whenever it changes, so we need to
        // spawn the Player to trigger the camera to move.
        app.world.spawn().insert_bundle((
            Player,
            Transform::from_xyz(
                TEST_LEVEL_WIDTH_OUT_RBOUNDS,
                TEST_LEVEL_HEIGHT_OUT_TBOUNDS,
                0.0,
            ),
        ));

        let camera_id = spawn_camera(&mut app);

        app.update();

        let camera_query = app.world.get::<Transform>(camera_id);
        assert!(camera_query.is_some());

        let expected_transform_x = (TEST_LEVEL_WIDTH - CAMERA_MIDPOINT) as f32;
        let expected_transform_y = (TEST_LEVEL_HEIGHT - CAMERA_MIDPOINT) as f32;

        let actual_transform = *camera_query.unwrap();
        let actual_transform_x = actual_transform.translation.x;
        let actual_transform_y = actual_transform.translation.y;

        assert_eq!(expected_transform_x, actual_transform_x);
        assert_eq!(expected_transform_y, actual_transform_y);
    }

    #[test]
    fn out_of_bounds_bottomright() {
        let mut app = setup_app_bounds_checking();

        // The camera's position is dependent off of the Player's position whenever it changes, so we need to
        // spawn the Player to trigger the camera to move.
        app.world.spawn().insert_bundle((
            Player,
            Transform::from_xyz(
                TEST_LEVEL_WIDTH_OUT_RBOUNDS,
                TEST_LEVEL_HEIGHT_OUT_BBOUNDS,
                0.0,
            ),
        ));

        let camera_id = spawn_camera(&mut app);

        app.update();

        let camera_query = app.world.get::<Transform>(camera_id);
        assert!(camera_query.is_some());

        let expected_transform_x = (TEST_LEVEL_WIDTH - CAMERA_MIDPOINT) as f32;
        let expected_transform_y = CAMERA_MIDPOINT as f32;

        let actual_transform = *camera_query.unwrap();
        let actual_transform_x = actual_transform.translation.x;
        let actual_transform_y = actual_transform.translation.y;

        assert_eq!(expected_transform_x, actual_transform_x);
        assert_eq!(expected_transform_y, actual_transform_y);
    }

    #[test]
    fn out_of_bounds_top() {
        let mut app = setup_app_bounds_checking();

        // The camera's position is dependent off of the Player's position whenever it changes, so we need to
        // spawn the Player to trigger the camera to move.
        app.world.spawn().insert_bundle((
            Player,
            Transform::from_xyz(
                TEST_LEVEL_WIDTH_IN_BOUNDS,
                TEST_LEVEL_HEIGHT_OUT_TBOUNDS,
                0.0,
            ),
        ));

        let camera_id = spawn_camera(&mut app);

        app.update();

        let camera_query = app.world.get::<Transform>(camera_id);
        assert!(camera_query.is_some());

        let expected_transform_x = TEST_LEVEL_HEIGHT_IN_BOUNDS;
        let expected_transform_y = (TEST_LEVEL_HEIGHT - CAMERA_MIDPOINT) as f32;

        let actual_transform = *camera_query.unwrap();
        let actual_transform_x = actual_transform.translation.x;
        let actual_transform_y = actual_transform.translation.y;

        assert_eq!(expected_transform_x, actual_transform_x);
        assert_eq!(expected_transform_y, actual_transform_y);
    }

    #[test]
    fn out_of_bounds_bottom() {
        let mut app = setup_app_bounds_checking();

        // The camera's position is dependent off of the Player's position whenever it changes, so we need to
        // spawn the Player to trigger the camera to move.
        app.world.spawn().insert_bundle((
            Player,
            Transform::from_xyz(
                TEST_LEVEL_WIDTH_IN_BOUNDS,
                TEST_LEVEL_HEIGHT_OUT_BBOUNDS,
                0.0,
            ),
        ));

        let camera_id = spawn_camera(&mut app);

        app.update();

        let camera_query = app.world.get::<Transform>(camera_id);
        assert!(camera_query.is_some());

        let expected_transform_x = TEST_LEVEL_WIDTH_IN_BOUNDS;
        let expected_transform_y = CAMERA_MIDPOINT as f32;

        let actual_transform = *camera_query.unwrap();
        let actual_transform_x = actual_transform.translation.x;
        let actual_transform_y = actual_transform.translation.y;

        assert_eq!(expected_transform_x, actual_transform_x);
        assert_eq!(expected_transform_y, actual_transform_y);
    }
}

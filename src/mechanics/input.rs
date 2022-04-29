use crate::Player;
use bevy::prelude::*;

pub enum Movement {
    Up,
    Down,
    Left,
    Right,

    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

pub fn player_input(input: Res<Input<KeyCode>>, mut input_broadcast: EventWriter<Movement>) {
    if input.pressed(KeyCode::W) && input.pressed(KeyCode::A) {
        input_broadcast.send(Movement::UpLeft);
    } else if input.pressed(KeyCode::W) && input.pressed(KeyCode::D) {
        input_broadcast.send(Movement::UpRight);
    } else if input.pressed(KeyCode::S) && input.pressed(KeyCode::A) {
        input_broadcast.send(Movement::DownLeft);
    } else if input.pressed(KeyCode::S) && input.pressed(KeyCode::D) {
        input_broadcast.send(Movement::DownRight);
    } else if input.pressed(KeyCode::W) {
        input_broadcast.send(Movement::Up);
    } else if input.pressed(KeyCode::S) {
        input_broadcast.send(Movement::Down);
    } else if input.pressed(KeyCode::A) {
        input_broadcast.send(Movement::Left);
    } else if input.pressed(KeyCode::D) {
        input_broadcast.send(Movement::Right);
    }
}

/*pub fn movement_logger(mut input_receiver: EventReader<Movement>) {
    for movement_action in input_receiver.iter() {
        let input_direction = match movement_action {
            Movement::Up => "Up",
            Movement::Down => "Down",
            Movement::Left => "Left",
            Movement::Right => "Right",
            Movement::UpLeft => "UpLeft",
            Movement::UpRight => "UpRight",
            Movement::DownLeft => "DownLeft",
            Movement::DownRight => "DownRight",
        };

        println!("Player input: {}", input_direction);
    }
}*/

pub fn move_player(
    mut input_receiver: EventReader<Movement>,
    mut query: Query<(&mut Transform, &mut Sprite), With<Player>>,
) {
    for movement_action in input_receiver.iter() {
        let (mut player_transform, mut sprite) = query.single_mut();

        let pixel_distance = 3.0;
        let mut direction = Vec3::ZERO;
        match movement_action {
            Movement::Up => direction += Vec3::new(0.0, pixel_distance, 0.0),
            Movement::Down => direction -= Vec3::new(0.0, pixel_distance, 0.0),
            Movement::Left => {
                direction -= Vec3::new(pixel_distance, 0.0, 0.0);
                sprite.flip_x = false;
            }
            Movement::Right => {
                direction += Vec3::new(pixel_distance, 0.0, 0.0);
                sprite.flip_x = true;
            }
            Movement::UpLeft => {
                direction += Vec3::new(-pixel_distance, pixel_distance, 0.0);
                sprite.flip_x = false;
            }
            Movement::UpRight => {
                direction += Vec3::new(pixel_distance, pixel_distance, 0.0);
                sprite.flip_x = true;
            }
            Movement::DownLeft => {
                direction -= Vec3::new(pixel_distance, pixel_distance, 0.0);
                sprite.flip_x = false;
            }
            Movement::DownRight => {
                direction -= Vec3::new(-pixel_distance, pixel_distance, 0.0);
                sprite.flip_x = true;
            }
        }

        player_transform.translation += direction;
    }
}

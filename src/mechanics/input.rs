use crate::Player;
use bevy::prelude::*;

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

pub fn move_player(
    mut input_receiver: EventReader<Movement>,
    mut query: Query<(&mut Transform, &mut TextureAtlasSprite), With<Player>>,
) {
    for movement_action in input_receiver.iter() {
        let (mut player_transform, mut sprite) = query.single_mut();

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

        player_transform.translation += direction;
    }
}

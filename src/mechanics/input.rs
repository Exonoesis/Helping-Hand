use crate::entities::player::{Player, PlayerMovementActions};
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

pub fn move_player(
    mut input_receiver: EventReader<Movement>,
    mut player_query: Query<(&mut Transform, &mut TextureAtlasSprite), With<Player>>,
    tile_query: Query<&EntityInstance>,
    level_query: Query<&Handle<LdtkLevel>>,
    loaded_levels: Res<Assets<LdtkLevel>>,
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

        let level_info = &loaded_levels
            .get(level_query.single())
            .expect("The level should exist by now.")
            .level;
        let level_height = level_info.px_hei;
        let level_width = level_info.px_wid;

        let tile_side_length = 64.0;
        let tile_mid_point = tile_side_length / 2.0;

        let mut projected_position = player_transform.translation + direction;
        projected_position.x = projected_position
            .x
            .clamp(tile_mid_point, level_width as f32 - tile_mid_point);
            
        projected_position.y = projected_position
            .y
            .clamp(tile_mid_point, level_height as f32 - tile_mid_point);

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
                (level_height - (collision_tile.px.y)) as f32,
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

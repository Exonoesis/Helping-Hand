use crate::entities::player::{Player, PlayerMovementActions};
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_ecs_ldtk::{EntityInstance, LdtkAsset};

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
    world_query: Query<&Handle<LdtkAsset>>,
    loaded_worlds: Res<Assets<LdtkAsset>>,
    mut player_movement_broadcast: EventWriter<PlayerMovementActions>
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

        let world_height = loaded_worlds
            .get(world_query.single())
            .expect("The world should exist by now.")
            .world_height();
        let tile_side_length = 64.0;

        for &collision_tile in collision_tiles.iter() {
            let tile_position = Vec3::new(
                collision_tile.px.x as f32,
                (world_height - collision_tile.px.y) as f32,
                0.0,
            );

            if collide(
                projected_position,
                Vec2::new(tile_side_length, tile_side_length),
                tile_position,
                Vec2::new(tile_side_length, tile_side_length),
            )
            .is_some()
            {
                player_movement_broadcast.send(PlayerMovementActions::Bump);
                return;
            }
        }
        player_transform.translation = projected_position;
        player_movement_broadcast.send(PlayerMovementActions::Walk);
    }
}

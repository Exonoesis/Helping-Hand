use bevy::prelude::*;
use mechanics::input::*;

mod mechanics;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle
{
    _p: Player,

    #[bundle]
    sprite: SpriteBundle,
}

fn spawn_player(mut commands: Commands, asset_spawner: Res<AssetServer>)
{
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(PlayerBundle
        {
            _p: Player,
            sprite: SpriteBundle
            {
                texture: asset_spawner.load("textures/characters/eevee_sky_v2.png"), 
                ..default()
            },
        });
}

fn main()
{
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(spawn_player)
    .add_event::<Movement>()
    .add_system(player_input)
    .add_system(movement_logger)
    .add_system(move_player)
    .run();
}
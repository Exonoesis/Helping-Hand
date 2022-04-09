// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::DefaultPlugins;
use helping_hand::GamePlugin;
use bevy_ecs_tilemap::prelude::*;
use helpers::tiled::*;

//#[path = "../helpers/mod.rs"]
mod helpers;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>)
{
    //Comment out to not spawn map on title screen
    //commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let handle: Handle<TiledMap> = asset_server.load("ortho-map.tmx");

    let map_entity = commands.spawn().id();

    commands.entity(map_entity).insert_bundle(TiledMapBundle {
        tiled_map: handle,
        map: Map::new(0u16, map_entity),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: "Helping Hand".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(TiledMapPlugin)
        .add_startup_system(startup)
        .add_system(helpers::texture::set_texture_filters_to_nearest)
        .add_plugin(GamePlugin)
        .add_system(helpers::camera::movement)
        .run();
}

use bevy::prelude::*;

use cucumber::{given, then, when, World};

use helping_hand::map::*;

#[derive(Debug, Default, World)]
#[world(init = Self::new)]
struct GameWorld {
    pub app: App,
}

impl GameWorld {
    pub fn new() -> Self {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins);

        Self { app }
    }
}

fn spawn_map(mut commands: Commands) {}

#[given("a LDtk map called test_map.ldtk,")]
fn verify_test_map_exists(world: &mut GameWorld) {
    unimplemented!()
}

#[when("the LDtk map is loaded,")]
fn load_test_map(world: &mut GameWorld) {
    world.app.add_systems(Startup, spawn_map);
    world.app.update();
}

#[then("there are 4x4 (16) tiles loaded in a grid.")]
fn verify_loaded_tile_amount(world: &mut GameWorld) {
    unimplemented!()
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(GameWorld::run("tests/feature-files/tilemap.feature"));
}

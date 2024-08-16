use bevy::prelude::*;

use cucumber::{given, then, when, World};

use helping_hand::plugins::levels::LevelsPlugin;

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

// #[given("a Map,")]
// fn load_map(world: &mut GameWorld) {
//     unimplemented!()
// }

// #[given("a Player on a Map on the center of Tile A,")]
// fn verify_player_spawned(world: &mut GameWorld) {
//     unimplemented!()
// }

// #[when("the Player is requested to move to the right,")]
// fn move_player_right(world: &mut GameWorld) {
//     unimplemented!()
// }

// #[then("the Player should be on the center of Tile B.")]
// fn verify_player_moved_to_tile_b(world: &mut GameWorld) {
//     unimplemented!()
// }

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(GameWorld::run(
        "tests/feature-files/grid-based-movement.feature",
    ));
}

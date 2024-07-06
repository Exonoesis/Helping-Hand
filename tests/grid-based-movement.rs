use cucumber::World;

#[derive(Debug, Default, World)]
struct GameWorld;

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(GameWorld::run(
        "tests/feature-files/grid-based-movement.feature",
    ));
}

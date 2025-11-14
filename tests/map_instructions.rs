use cucumber::{given, then, when, World};

#[derive(Debug, World)]
#[world(init = Self::new)]
struct GameWorld {}

impl GameWorld {
    pub fn new() -> Self {
        Self {}
    }
}

fn main() {
    futures::executor::block_on(GameWorld::run(
        "tests/feature-files/in-theory/map_instructions.feature",
    ));
}

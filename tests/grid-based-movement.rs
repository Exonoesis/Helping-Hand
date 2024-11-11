use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
    sprite::SpritePlugin,
};

use cucumber::{given, then, when, World};

use helping_hand::{
    plugins::levels::LevelsPlugin,
    visuals::map::{TileType, XyzCords},
};

#[derive(Debug, Default, World)]
#[world(init = Self::new)]
struct Game {
    app: App,
}

impl Game {
    pub fn new() -> Self {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(RenderPlugin {
            render_creation: WgpuSettings {
                backends: None,
                ..default()
            }
            .into(),
            ..default()
        });
        app.add_plugins(SpritePlugin);
        app.add_plugins(ImagePlugin::default());

        Self { app }
    }

    /// Returns the pixel coordinates for some tile loaded in the game.
    pub fn get_position_from_tile(&mut self, tile_grid_coordinates: &XyzCords) -> Transform {
        let tile_position = *self
            .app
            .world
            .query::<(&Transform, &XyzCords)>()
            .iter(&self.app.world)
            .find(|&tile_positions| {
                let current_tile_grid_coordinates = tile_positions.1;

                current_tile_grid_coordinates == tile_grid_coordinates
            })
            .expect("get_position_from_tile: Tile could not be found in the game.")
            // Specifies from the (Transform, XyzCords), we want the Transform.
            .0;

        tile_position
    }

    /// Returns the pixel coordinates for the player in the game.
    pub fn get_player_position(&mut self) -> Transform {
        let player_position = Transform::default();

        player_position
    }

    /// Returns the grid coordinates for the player in the game.
    pub fn find_coordinates_of_player(&mut self) -> XyzCords {
        let player_tile_coordinate = *self
            .app
            .world
            .query::<(&XyzCords, &TileType)>()
            .iter(&self.app.world)
            .find(|&tile_entry| {
                let current_tile_type = tile_entry.1;

                current_tile_type == &TileType::Player
            })
            .expect("find_coordinates_of_player: No player found in the map.")
            // Specifies from the (XyzCords, TileType), we want the XyzCords.
            .0;

        player_tile_coordinate
    }
}

#[given("the player is at 0,1,1,")]
fn verify_player_spawned_at_tile_pos(game: &mut Game) {
    let expected_player_tile_coordinate = XyzCords::new(0, 1, 1);
    let actual_player_tile_coordinate = game.find_coordinates_of_player();
    assert_eq!(
        expected_player_tile_coordinate,
        actual_player_tile_coordinate
    );
}

#[when("the Player is requested to move to the right,")]
fn move_player_right(game: &mut Game) {
    //TODO: This deceiving method needs to be implemented.
    //game.move_player(Direction::Right);
}

#[then("the Player's pixel coordinates are equivalent to tile 1,1,1.")]
fn verify_player_at_tile_pos(game: &mut Game) {
    let tile_grid_coordinates = XyzCords::new(1, 1, 1);

    let expected_player_position = game.get_position_from_tile(&tile_grid_coordinates);
    let actual_player_position = game.get_player_position();
    assert_eq!(expected_player_position, actual_player_position);
}

// This runs before everything else, so you can setup things here.
fn main() {
    futures::executor::block_on(Game::run("tests/feature-files/grid-based-movement.feature"));
}

use std::time::Duration;

use bevy::{
    input::InputPlugin,
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
    sprite::SpritePlugin,
};

use cucumber::{given, then, when, World};

use helping_hand::{
    entities::player::Player,
    mechanics::input::{ArrivalTime, MovementDirection, Target},
    plugins::{levels::LevelsPlugin, playable_character::PlayableCharacterPlugin},
    visuals::map::{ChangeLevel, TileType, XyzCords},
    AppState,
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

        app.add_plugins(WindowPlugin::default());
        app.add_plugins(InputPlugin::default());

        app.add_plugins(PlayableCharacterPlugin);
        app.insert_resource(ArrivalTime::new(Duration::from_secs_f32(0.0)));

        app.insert_state(AppState::InGame);

        Self { app }
    }

    /// Returns the pixel coordinates for some tile loaded in the game.
    pub fn get_position_from_tile(&mut self, tile_grid_coordinates: &XyzCords) -> Transform {
        let tile_position = self
            .find_containing(tile_grid_coordinates)
            .expect("get_position_from_tile: Could not get Transform from given tile.");

        tile_position
    }

    /// Returns the pixel coordinates for the player in the game.
    pub fn get_player_position(&mut self) -> Transform {
        let player_position = self.get_with::<Transform, Player>();

        player_position
    }

    /// Returns whether a Component exists.
    fn has<C, D>(&mut self) -> bool
    where
        C: Component,
        D: Component,
    {
        let has_component = self
            .app
            .world
            .query_filtered::<&C, With<D>>()
            .iter(&self.app.world)
            .len()
            == 1;

        has_component
    }

    /// Returns a Component that has some other component, or panics otherwise.
    fn get_with<C, D>(&mut self) -> C
    where
        C: Component + Copy,
        D: Component,
    {
        self.find_with::<C, D>()
            .expect("get_with: Cannot find a Component with some other Component.")
    }

    /// Returns a Component that has some other component, or None if not found.
    fn find_with<C, D>(&mut self) -> Option<C>
    where
        C: Component + Copy,
        D: Component,
    {
        let found_component = self
            .app
            .world
            .query_filtered::<&C, With<D>>()
            .iter(&self.app.world)
            .next()
            .map(|entry| *entry);

        found_component
    }

    /// Returns a Component that contains another specific component, or None if not found.
    fn find_containing<C, D>(&mut self, search_component: &D) -> Option<C>
    where
        C: Component + Copy,
        D: Component + PartialEq,
    {
        let found_component = self
            .app
            .world
            .query::<(&C, &D)>()
            .iter(&self.app.world)
            .find(|&entry| {
                let current_component = entry.1;

                current_component == search_component
            })
            .map(|entry| *entry.0);

        found_component
    }

    /// Returns the grid coordinates for the player in the game.
    pub fn find_coordinates_of_player(&mut self) -> XyzCords {
        let player_tile_coordinate = self
            .find_containing(&TileType::Player)
            .expect("find_coordinates_of_player: Could not find XyzCords from player.");

        player_tile_coordinate
    }

    /// Send an event to all systems listening in the Bevy game engine.
    pub fn broadcast_event<C>(&mut self, event_to_send: C)
    where
        C: Event,
    {
        self.app.world.send_event(event_to_send);
        self.app.update();
    }
}

#[given(regex = r"a Tiled map called (.+),")]
fn given_some_tiled_map(game: &mut Game, tiled_map_name: String) {
    game.app.add_plugins(LevelsPlugin);

    let map_path = format!("tests/test-assets/maps/{}", tiled_map_name);
    game.broadcast_event(ChangeLevel::new(&map_path));
}

#[given(regex = r"the player is at ([0-9]+),([0-9]+),([0-9]+),")]
fn verify_player_spawned_at_tile_pos(game: &mut Game, tile_x: u32, tile_y: u32, tile_z: usize) {
    let expected_player_tile_coordinate = XyzCords::new(tile_x, tile_y, tile_z);
    let actual_player_tile_coordinate = game.find_coordinates_of_player();
    assert_eq!(
        expected_player_tile_coordinate,
        actual_player_tile_coordinate
    );
}

#[when("the Player is requested to move to the right,")]
fn request_player_to_move_right(game: &mut Game) {
    game.broadcast_event(MovementDirection::Right);
}

// TODO: Turn "right" into regex ([a-zA-Z]+)
#[when("the Player moves to the right,")]
fn move_player_right(game: &mut Game) {
    request_player_to_move_right(game);

    for _i in 0..255 {
        game.app.update();

        let has_traveled = game
            .app
            .world
            .query::<&Target>()
            .iter(&game.app.world)
            .len()
            == 0;
        if has_traveled {
            break;
        }
    }
}

#[then(
    regex = r"the Player's pixel coordinates are equivalent to tile ([0-9]+),([0-9]+),([0-9]+)."
)]
fn verify_player_at_tile_pos(game: &mut Game, tile_x: u32, tile_y: u32, tile_z: usize) {
    let tile_grid_coordinates = XyzCords::new(tile_x, tile_y, tile_z);

    let expected_player_position = game.get_position_from_tile(&tile_grid_coordinates);
    let actual_player_position = game.get_player_position();
    assert_eq!(expected_player_position, actual_player_position);
}

#[then("the Player should have a Target.")]
fn verify_player_has_target(game: &mut Game) {
    let player_has_target = game.has::<Player, Target>();
    assert!(player_has_target);
}

// This runs before everything else, so you can setup things here.
fn main() {
    futures::executor::block_on(Game::run("tests/feature-files/grid-based-movement.feature"));
}

use std::time::Duration;

use bevy::{
    input::InputPlugin,
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
    sprite::SpritePlugin,
    state::app::StatesPlugin,
    window::WindowResolution,
};
use cucumber::World;

use helping_hand::{
    map::{movement::grid_based_movement::*, player::*, *},
    plugins::playable_character::PlayableCharacterTestingPlugin,
    AppState,
};

#[derive(Debug, Default, World)]
#[world(init = Self::new)]
pub struct Game {
    app: App,
}

impl Game {
    pub fn new() -> Self {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(StatesPlugin);
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

        app.add_plugins(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1280.0, 720.0),
                ..default()
            }),
            ..default()
        });
        app.add_plugins(InputPlugin::default());

        app.add_plugins(PlayableCharacterTestingPlugin);
        app.insert_resource(ArrivalTime::new(Duration::from_secs_f32(0.0)));

        app.insert_state(AppState::InGame);

        Self { app }
    }

    /// Loads a plugin into the game.
    pub fn add_plugin<T>(&mut self, plugin: T)
    where
        T: Plugin,
    {
        self.app.add_plugins(plugin);
    }

    /// Advances the game by one frame.
    pub fn tick(&mut self) {
        self.app.update();
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
        let player_position = self.get_of::<Transform, Player>();

        player_position
    }

    /// Returns the pixel coordinates for the player's center in the game.
    pub fn get_centered_player_position(&mut self) -> Transform {
        let player_position = self.get_player_position();
        let player_tile_dimensions = self.get_of::<PxDimensions, Player>();

        let half_tile_width = player_tile_dimensions.get_width() as f32 / 2.0;
        let half_tile_height = player_tile_dimensions.get_height() as f32 / 2.0;

        let centered_player_position = Transform::from_xyz(
            player_position.translation.x + half_tile_width,
            player_position.translation.y + half_tile_height,
            player_position.translation.z,
        );

        centered_player_position
    }

    /// Returns whether a Component exists.
    pub fn has<C, D>(&mut self) -> bool
    where
        C: Component,
        D: Component,
    {
        let has_component = self
            .app
            .world_mut()
            .query_filtered::<&C, With<D>>()
            .iter(&self.app.world())
            .len()
            == 1;

        has_component
    }

    /// Returns a specified Component
    pub fn get_mut<C>(&mut self) -> Mut<'_, C>
    where
        C: Component,
    {
        self.app
            .world_mut()
            .query::<&mut C>()
            .iter_mut(self.app.world_mut())
            .next()
            .unwrap()
    }

    /// Returns a Component C that has some other Component D associated with it, or panics otherwise.
    pub fn get_of<C, D>(&mut self) -> C
    where
        C: Component + Copy,
        D: Component,
    {
        self.find_with::<C, D>()
            .expect("get_with: Cannot find a Component with some other Component.")
    }

    /// Returns the number of Components found in the game.
    pub fn get_number_of<C>(&mut self) -> usize
    where
        C: Component,
    {
        let num_components_found = self
            .app
            .world_mut()
            .query::<&C>()
            .iter(&self.app.world())
            .len();

        num_components_found
    }

    /// Returns a Component that has some other component, or None if not found.
    pub fn find_with<C, D>(&mut self) -> Option<C>
    where
        C: Component + Copy,
        D: Component,
    {
        let found_component = self
            .app
            .world_mut()
            .query_filtered::<&C, With<D>>()
            .iter(&self.app.world())
            .next()
            .map(|entry| *entry);

        found_component
    }

    /// Returns a Component that contains another specific component, or None if not found.
    pub fn find_containing<C, D>(&mut self, search_component: &D) -> Option<C>
    where
        C: Component + Copy,
        D: Component + PartialEq,
    {
        let found_component = self
            .app
            .world_mut()
            .query::<(&C, &D)>()
            .iter(&self.app.world())
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

    /// Returns the grid dimensions of the currently loaded level.
    pub fn get_map_size(&mut self) -> GridDimensions {
        let map_size = *self.get_mut::<GridDimensions>();

        map_size
    }

    /// Returns the direction the player is currently facing.
    pub fn get_player_facing_direction(&mut self) -> MovementDirection {
        let facing_direction = self
            .find_containing(&TileType::Player)
            .expect("get_player_facing_direction: Could not find direction facing from player.");

        facing_direction
    }

    /// Send an event to all systems listening in the Bevy game engine.
    pub fn broadcast_event<C>(&mut self, event_to_send: C)
    where
        C: Event,
    {
        self.app.world_mut().send_event(event_to_send);
        self.app.update();
    }

    /// Sets the window dimensions of the game to the specified width and height.
    pub fn set_window_resolution(&mut self, window_width: usize, window_height: usize) {
        let mut window = self.get_mut::<Window>();

        window.resolution = WindowResolution::new(window_width as f32, window_height as f32);
    }
}

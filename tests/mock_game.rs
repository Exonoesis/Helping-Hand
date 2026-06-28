use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use bevy::{
    ecs::component::Mutable,
    input::InputPlugin,
    mesh::MeshPlugin,
    prelude::*,
    render::{settings::WgpuSettings, view::screenshot::CapturedScreenshots, RenderPlugin},
    sprite::SpritePlugin,
    state::{app::StatesPlugin, state::FreelyMutableState},
    text::TextPlugin,
    window::WindowResolution,
};
use cucumber::World;

use helping_hand::{
    map::{movement::grid_based_movement::*, npc::NPC, player::*, *},
    narrative::act_loading::{LoadStatus, SpawnDone},
    plugins::{camera::CameraPlugin, playable_character::PlayableCharacterTestingPlugin},
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
        app.add_plugins(InputPlugin::default());
        app.add_plugins(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1280, 720),
                ..default()
            }),
            ..default()
        });
        app.add_plugins(AssetPlugin::default());
        app.add_plugins(RenderPlugin {
            render_creation: WgpuSettings {
                backends: None,
                ..default()
            }
            .into(),
            ..default()
        });
        app.add_plugins(ImagePlugin::default());
        app.add_plugins(SpritePlugin::default());
        app.add_plugins(StatesPlugin);
        app.add_plugins(DefaultPickingPlugins);

        app.add_plugins(PlayableCharacterTestingPlugin);
        app.add_plugins(MeshPlugin);
        app.add_plugins(TextPlugin);
        app.add_plugins(CameraPlugin);
        app.insert_resource(ArrivalTime::new(Duration::from_secs_f32(0.0)));
        app.add_systems(
            Update,
            bypass_waiting_for_assets.run_if(in_state(AppState::Transitioning)),
        );

        // NOTE: How dare you Bevy! We need this to ensure tests do not crash
        // starting in 0.15. Maybe we can remove these two lines in the future.
        let (_, rx) = std::sync::mpsc::channel();
        app.insert_resource(CapturedScreenshots(Arc::new(Mutex::new(rx))));

        app.insert_state(AppState::InScene);

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

    /// Sets the window dimensions of the game to the specified width and height.
    pub fn set_window_resolution(&mut self, window_width: u32, window_height: u32) {
        let mut window = self.get_mut::<Window>();

        window.resolution = WindowResolution::new(window_width, window_height);
    }

    ///Sets the state for the game to be in
    pub fn set_state<S>(&mut self, state: S)
    where
        S: FreelyMutableState,
    {
        self.app.insert_state(state);
    }

    /// Gets the current state the game is in
    pub fn get_state(&mut self) -> &AppState {
        let game_state = self.get_res::<State<AppState>>();

        game_state.get()
    }

    /// Returns the pixel coordinates for some tile found at some grid coordinates loaded in the game.
    pub fn get_position_from_tile(&mut self, tile_grid_coordinates: &GridCords3D) -> Transform {
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

    /// Returns the pixel coordinates for a given npc in the game.
    pub fn get_npc_position(&mut self, npc_name: &String) -> Transform {
        let npc_label = NPC::new(npc_name.clone());

        let npc_position = self
            .find_containing(&npc_label)
            .expect("get_npc_position: Could not find Transform of given NPC.");

        npc_position
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
        C: Component<Mutability = Mutable>,
    {
        self.app
            .world_mut()
            .query::<&mut C>()
            .iter_mut(self.app.world_mut())
            .next()
            .expect("Could not find specified component.")
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
    pub fn find_coordinates_of_player(&mut self) -> GridCords3D {
        let player_tile_coordinate = self
            .find_containing(&TileType::Player)
            .expect("find_coordinates_of_player: Could not find GridCords3D from player.");

        player_tile_coordinate
    }

    /// Returns the grid coordinates for a given NPC in the game.
    pub fn get_npc_coordinate(&mut self, npc_name: &String) -> GridCords3D {
        let npc_label = NPC::new(npc_name.clone());

        self.find_containing(&npc_label)
            .expect("get_npc_coordinate: Could not find coordinates of NPC.")
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
    pub fn write_message<C>(&mut self, event_to_send: C)
    where
        C: Message,
    {
        self.app.world_mut().write_message(event_to_send);
        self.tick();
    }

    /// Returns a specified Resource
    pub fn get_res<R>(&mut self) -> &R
    where
        R: Resource,
    {
        self.app.world().resource::<R>()
    }

    /// Returns a specified mutable Resource
    pub fn get_res_mut<R>(&mut self) -> Mut<'_, R>
    where
        R: Resource,
    {
        self.app.world_mut().resource_mut::<R>()
    }
}

pub fn bypass_waiting_for_assets(
    load_statuses: Query<(Entity, &LoadStatus)>,
    mut commands: Commands,
    mut spawning_done_notification: MessageWriter<SpawnDone>,
) {
    if load_statuses.is_empty() {
        return;
    }

    let total_entities = load_statuses.count();
    let mut loaded_entities = 0;

    for (entity_loading_asset, _) in load_statuses {
        loaded_entities += 1;
        commands.entity(entity_loading_asset).remove::<LoadStatus>();
    }

    if total_entities == loaded_entities {
        spawning_done_notification.write(SpawnDone);
    }
}

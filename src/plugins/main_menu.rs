use bevy::prelude::*;

use crate::{
    visuals::main_menu::*,
    mechanics::main_menu_buttons::*,
    AppState,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
        .add_systems(Update(AppState::MainMenu), (load_background_image,load_button_image,load_text_font))
        .add_systems(Update(AppState::MainMenu), button_system)
        .add_systems(OnExit(AppState::MainMenu), unload_main_menu);
    }
}
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
        .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu))            )
        .add_systems((load_background_image,load_button_image,load_text_font))      
        .add_system(button_system)
        .add_system(unload_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}
use bevy::prelude::*;

use crate::{
    visuals::main_menu::*,
    AppState,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(load_main_menu.in_schedule(OnEnter(AppState::MainMenu)));
    }
}

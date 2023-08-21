use bevy::prelude::*;

use crate::{mechanics::main_menu_buttons::*, visuals::main_menu::*, AppState};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(
                Update,
                (load_background_image, load_button_image, load_text_font)
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(Update, button_system.run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), unload_main_menu);
    }
}

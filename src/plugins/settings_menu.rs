use bevy::prelude::*;

use crate::{mechanics::settings_menu_buttons::*, visuals::settings_menu::*, AppState};

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SettingsMenu), spawn_settings_menu)
            .add_systems(
                Update,
                (
                    load_background_image,
                    load_box_image,
                    load_tab_image,
                    load_button_image,
                    load_increment_button_image,
                    load_decrement_button_image,
                    load_text_font,
                    set_keys,
                    add_widget_components,
                )
                    .run_if(in_state(AppState::SettingsMenu)),
            )
            .add_systems(
                Update,
                (
                    save_button_system,
                    spinner_buttons_system,
                    get_handle_click_position,
                    update_handle_position_on_hold,
                    change_music_volume,
                    change_sfx_volumes,
                )
                    .run_if(in_state(AppState::SettingsMenu)),
            )
            .add_systems(OnExit(AppState::SettingsMenu), unload_settings_menu);
    }
}

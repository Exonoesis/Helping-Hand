use bevy::prelude::*;

pub mod audio;
pub mod diagnostics;
pub mod entities;
pub mod mechanics;
pub mod plugins;
pub mod visuals;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    SettingsMenu,
    InGame,
}

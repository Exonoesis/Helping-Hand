use bevy::prelude::*;

pub mod audio;
pub mod map;
pub mod narrative;
pub mod plugins;
pub mod ui;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    SettingsMenu,
    InGame,
}

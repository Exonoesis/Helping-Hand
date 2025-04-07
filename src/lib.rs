use bevy::prelude::*;

pub mod audio;
pub mod plugins;

pub mod map;
pub mod ui;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    SettingsMenu,
    InGame,
}

use bevy::prelude::*;
use bevy_ecs_ldtk::LevelSelection;
use bevy_kira_audio::{AudioChannel, AudioControl};

#[derive(Default, Component, Resource)]
pub struct MusicChannel;

pub fn play_level_music(
    asset_server: Res<AssetServer>,
    current_level_name: Res<LevelSelection>,
    background_music: Res<AudioChannel<MusicChannel>>,
) {
    let level_has_changed = current_level_name.is_changed() || current_level_name.is_added();

    if !level_has_changed {
        return;
    }

    let level_identifier = match &*current_level_name {
        LevelSelection::Identifier(name) => name.clone(),
        _ => panic!("Expected Level Identifier."),
    };

    let audio_file_path = format!("audio/music/{}_overworld.wav", level_identifier);

    background_music
        .play(asset_server.load(&audio_file_path))
        .looped();
}

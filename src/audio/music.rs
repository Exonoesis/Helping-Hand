use bevy_ecs_ldtk::LevelSelection;
use bevy_kira_audio::Audio;
use bevy::prelude::*;

pub fn play_level_music(asset_server: Res<AssetServer>, current_level_name: Res<LevelSelection>, audio: Res<Audio>) {
    if current_level_name.is_changed() || current_level_name.is_added() {
        let level_identifier = match &*current_level_name {
            LevelSelection::Identifier(name) => name.clone(),
            _ => panic!("Expected Level Identifier."),
        };
        
        let audio_file_path = format!("audio/music/{}_overworld.wav", level_identifier);

        audio.play_looped(asset_server.load(&audio_file_path));
    }
}
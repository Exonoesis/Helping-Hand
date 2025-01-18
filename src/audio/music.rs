use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::visuals::map::ChangeLevel;

#[derive(Default, Component, Resource)]
pub struct MusicChannel;

pub fn play_level_music(
    asset_server: Res<AssetServer>,
    mut level_change_requests: EventReader<ChangeLevel>,
    background_music: Res<AudioChannel<MusicChannel>>,
) {
    let level_has_changed = !level_change_requests.is_empty();
    if !level_has_changed {
        return;
    }

    let level_identifier = level_change_requests
        .read()
        .next()
        .unwrap()
        .get_level_name();
    let audio_file_path = format!("audio/music/{}.wav", level_identifier);

    background_music.stop();
    background_music
        .play(asset_server.load(audio_file_path))
        .looped();
}

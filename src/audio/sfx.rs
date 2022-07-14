use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use crate::entities::player::PlayerMovementActions;

#[derive(Default, Component)]
pub struct SFXChannel;

pub fn load_player_movement_sound(asset_server: Res<AssetServer>, player_movement_sound: Res<AudioChannel<SFXChannel>>) {
    let audio_file_path = "audio/sfx/player_walk.wav";
    player_movement_sound.play_looped(asset_server.load(audio_file_path));
    player_movement_sound.pause();
}

pub fn play_player_movement_sound(mut player_movement_receiver: EventReader<PlayerMovementActions>, player_movement_sound: Res<AudioChannel<SFXChannel>>) {
    if player_movement_receiver.is_empty() {
        player_movement_sound.pause();
        return;
    }

    let has_walked = player_movement_receiver.iter()
        .any(|&movement_action| movement_action == PlayerMovementActions::Walk);

    if !has_walked {
        player_movement_sound.pause();
    } else {
        player_movement_sound.resume();
    }
}
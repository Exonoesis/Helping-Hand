use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use crate::entities::player::{PlayerMovementActions, PlayerBumpChannel, PlayerWalkChannel};

pub fn load_player_movement_sound(asset_server: Res<AssetServer>, player_movement_sound: Res<AudioChannel<PlayerWalkChannel>>) {
    let audio_file_path = "audio/sfx/player_walk.wav";
    player_movement_sound.play_looped(asset_server.load(audio_file_path));
    player_movement_sound.pause();
}

pub fn play_player_movement_sound(mut player_movement_receiver: EventReader<PlayerMovementActions>, player_movement_sound: Res<AudioChannel<PlayerWalkChannel>>) {
    if player_movement_receiver.is_empty() {
        player_movement_sound.pause();
        return;
    }

    let has_walked = player_movement_receiver.iter()
        .all(|&movement_action| movement_action == PlayerMovementActions::Walking);

    if !has_walked {
        player_movement_sound.pause();
    } else {
        player_movement_sound.resume();
    }
}

pub fn load_player_bump_sound(asset_server: Res<AssetServer>, player_bump_sound: Res<AudioChannel<PlayerBumpChannel>>) {
    let audio_file_path = "audio/sfx/player_bump.wav";
    player_bump_sound.play_looped(asset_server.load(audio_file_path));
    player_bump_sound.pause();
}

pub fn play_player_bump_sound(mut player_movement_receiver: EventReader<PlayerMovementActions>, player_bump_sound: Res<AudioChannel<PlayerBumpChannel>>) {
    if player_movement_receiver.is_empty() {
        player_bump_sound.pause();
        return;
    }

    let has_bumped = player_movement_receiver.iter()
        .all(|&movement_action| movement_action == PlayerMovementActions::Bumping);

    if !has_bumped {
        player_bump_sound.pause();
    } else {
        player_bump_sound.resume();
    }
}
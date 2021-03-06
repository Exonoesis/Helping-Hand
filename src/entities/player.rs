use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Component)]
pub struct PlayerBumpChannel;

#[derive(Default, Component)]
pub struct PlayerWalkChannel;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum PlayerMovementActions {
    Walking,
    Bumping,
}

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_sheet_bundle("textures/characters/EeveeSprites.png", 64.0, 64.0, 1, 4, 0.0, 0)]
    #[bundle]
    sprite: SpriteSheetBundle,

    player: Player,
    bump_sound: PlayerBumpChannel,
    walk_sound: PlayerWalkChannel,
}

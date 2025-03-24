use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Event)]
pub struct PlayerInteraction;

#[derive(Default, Component, Resource)]
pub struct PlayerBumpChannel;

#[derive(Default, Component, Resource)]
pub struct PlayerWalkChannel;

#[derive(PartialEq, PartialOrd, Clone, Copy, Event)]
pub enum PlayerMovementActions {
    Walking,
    Bumping,
}

#[derive(Default, Component)]
pub enum DirectionFacing {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default, Component, PartialEq)]
pub enum MovementIntent {
    #[default]
    Idle,
    Moving,
}

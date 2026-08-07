use bevy::prelude::*;

#[derive(Component, PartialEq)]
pub struct Player {
    name: String,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player { name }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[derive(Default, Message)]
pub struct PlayerInteraction;

#[derive(Default, Component, Resource)]
pub struct PlayerBumpChannel;

#[derive(Default, Component, Resource)]
pub struct PlayerWalkChannel;

#[derive(PartialEq, PartialOrd, Clone, Copy, Message)]
pub enum PlayerMovementActions {
    Walking,
    Bumping,
}

use bevy::prelude::*;

#[derive(Component, PartialEq)]
pub struct NPC {
    name: String,
}

impl NPC {
    pub fn new(name: String) -> Self {
        NPC { name }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

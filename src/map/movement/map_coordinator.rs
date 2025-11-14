use crate::{map::XyzCords, narrative::acts::MapInstruction};

pub enum InstructionResult {
    PlaceResult(CharacterCords, MapLocationCords),
}

pub struct CharacterCords {
    current_character_position: XyzCords,
}

impl CharacterCords {
    pub fn new(current_character_position: XyzCords) -> Self {
        Self {
            current_character_position,
        }
    }
}

pub struct MapLocationCords {
    location_of_interest: XyzCords,
}

impl MapLocationCords {
    pub fn new(location_of_interest: XyzCords) -> Self {
        Self {
            location_of_interest,
        }
    }
}

// TODO
fn get_map_instruction_info_from_tiled(map_instruction: MapInstruction) -> InstructionResult {
    // Will need the level data passed in

    // Determine the instruction type
    // Match on enum => call helper function to extract the needed info for that type from tiled
    // Return the corresponding result type
}

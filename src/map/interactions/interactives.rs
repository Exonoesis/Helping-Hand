use std::{collections::HashMap, path::PathBuf};

use bevy::prelude::*;
use tiled::{Map, ObjectShape, PropertyValue};

use crate::map::{flip_y_axis, player::PlayerInteraction, GridDimensions, PxDimensions, XyzCords};

#[derive(Component, Debug, Clone, Default)]
pub struct InteractiveCollection {
    interactive_markers: Vec<InteractiveMarker>,
}

impl InteractiveCollection {
    pub fn new() -> Self {
        let interactive_markers = Vec::new();

        Self {
            interactive_markers,
        }
    }

    pub fn from_markers(mut interactive_markers: Vec<InteractiveMarker>) -> Self {
        interactive_markers.sort();

        Self {
            interactive_markers,
        }
    }

    pub fn len(&self) -> usize {
        self.interactive_markers.len()
    }

    pub fn get_marker_at_index(&self, index: usize) -> &InteractiveMarker {
        &self.interactive_markers[index]
    }

    pub fn get_marker_from_position(&self, position: &XyzCords) -> Option<&InteractiveMarker> {
        if self.len() == 0 {
            return None;
        }

        let mut left = 0;
        let mut right = self.len() - 1;

        while left <= right {
            let mid = (left + right) / 2;
            let marker = self.get_marker_at_index(mid);
            let comparison = marker.containing(position);

            if comparison == Proximity::Higher {
                left = mid + 1;
            } else if comparison == Proximity::Lower {
                if mid == 0 {
                    break;
                }
                right = mid - 1;
            } else {
                return Some(marker);
            }
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct InteractiveMarker {
    position: XyzCords,
    dimensions: PxDimensions,
    interaction_type: InteractiveType,
}

impl InteractiveMarker {
    pub fn new(
        position: XyzCords,
        dimensions: PxDimensions,
        interaction_type: InteractiveType,
    ) -> Self {
        InteractiveMarker {
            position,
            dimensions,
            interaction_type,
        }
    }

    pub fn containing(&self, position: &XyzCords) -> Proximity {
        let marker_min_x = self.position.get_x();
        let marker_max_x = marker_min_x + (self.dimensions.get_width());
        let marker_x_range = marker_min_x..marker_max_x;
        let position_x = position.get_x();

        let marker_min_y = self.position.get_y();
        let marker_max_y = marker_min_y + (self.dimensions.get_height());
        let marker_y_range = marker_min_y..marker_max_y;
        let position_y = position.get_y();

        if marker_x_range.contains(&position_x) && marker_y_range.contains(&position_y) {
            return Proximity::Match;
        }

        if position_x < marker_min_x {
            Proximity::Lower
        } else if position_x >= marker_max_x {
            Proximity::Higher
        } else if position_y < marker_min_y {
            Proximity::Lower
        } else {
            Proximity::Higher
        }
    }

    pub fn get_position(&self) -> XyzCords {
        self.position
    }

    pub fn get_dimensions(&self) -> PxDimensions {
        self.dimensions
    }

    pub fn get_interactive_type(&self) -> InteractiveType {
        self.interaction_type.clone()
    }

    pub fn get_type_name(&self) -> String {
        self.interaction_type.type_name()
    }

    pub fn get_path(&self) -> PathBuf {
        self.interaction_type.type_value()
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum Proximity {
    Lower,
    Higher,
    Match,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum InteractiveType {
    Transition(PathBuf),
}

impl InteractiveType {
    fn type_name(&self) -> String {
        match self {
            InteractiveType::Transition(_) => "Transition".to_string(),
        }
    }

    fn type_value(&self) -> PathBuf {
        match self {
            InteractiveType::Transition(value) => value.clone(),
        }
    }
}

pub fn get_interactives_from(tiled_map: &Map) -> Vec<InteractiveMarker> {
    let mut interactive_markers = Vec::new();

    for z in 0..tiled_map.layers().len() {
        let found_object_layer = tiled_map.get_layer(z).unwrap().as_object_layer();

        if found_object_layer.is_none() {
            continue;
        }

        let object_layer = found_object_layer.unwrap();
        let objects = object_layer.objects();

        for object in objects {
            let position = XyzCords::new(object.x as usize, object.y as usize, z);

            // Get properties and create interactive type from it
            let properties = &object.properties;
            let interactive_type = create_interactive_type(properties);

            // Get shape, check it's a Rect, get width and height
            if let ObjectShape::Rect { width, height } = object.shape {
                let object_width = width as u32;
                let object_height = height as u32;

                let dimensions = PxDimensions::new(object_width, object_height);

                let interactive_marker =
                    InteractiveMarker::new(position, dimensions, interactive_type);
                interactive_markers.push(interactive_marker);
            }
        }
    }

    interactive_markers
}

pub fn create_interactive_type(properties: &HashMap<String, PropertyValue>) -> InteractiveType {
    let property_value = properties.get("Transition");
    // We assume that there is only one property on a marker
    if let Some(PropertyValue::StringValue(property_string)) = property_value {
        InteractiveType::Transition(PathBuf::from(property_string))
    } else {
        panic!("create_interactive_type: Marker Type is invalid")
    }
}

pub fn flip_interactives_on_y_axis(
    markers: Vec<InteractiveMarker>,
    map_size_in_px: PxDimensions,
    map_grid_dimensions: GridDimensions,
) -> Vec<InteractiveMarker> {
    let mut y_flipped_markers = Vec::new();
    let map_height = map_size_in_px.get_height();
    let tile_height = map_height / map_grid_dimensions.get_rows() as usize;

    for marker in markers {
        let marker_xyzcords = marker.get_position();

        let flipped_ycord = XyzCords::new(
            marker_xyzcords.get_x(),
            flip_y_axis(map_height, marker_xyzcords.get_y() as f32, tile_height) as usize,
            marker_xyzcords.get_z(),
        );

        let flipped_marker = InteractiveMarker::new(
            flipped_ycord,
            marker.get_dimensions(),
            marker.get_interactive_type(),
        );

        y_flipped_markers.push(flipped_marker);
    }

    y_flipped_markers
}

pub fn interact_entity(
    input: Res<ButtonInput<KeyCode>>,
    mut interactive_event_writer: EventWriter<PlayerInteraction>,
) {
    if !input.just_pressed(KeyCode::KeyE) {
        return;
    }

    interactive_event_writer.write(PlayerInteraction);
}

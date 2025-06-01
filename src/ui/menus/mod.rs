pub mod main_menu;
pub mod settings_menu;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ImageNodeBundle {
    node: Node,
    image: ImageNode,
}

impl Default for ImageNodeBundle {
    fn default() -> Self {
        Self {
            node: Node::default(),
            image: ImageNode::default(),
        }
    }
}

impl ImageNodeBundle {
    pub fn from_nodes(node: Node, image: ImageNode) -> Self {
        Self { node, image }
    }
}

#[derive(Bundle)]
pub struct ButtonNodeBundle {
    node: Node,
    image: ImageNode,
    button: Button,
}

impl Default for ButtonNodeBundle {
    fn default() -> Self {
        Self {
            node: Node::default(),
            image: ImageNode::default(),
            button: Button::default(),
        }
    }
}

#[derive(Bundle)]
pub struct TextNodeBundle {
    text: Text,
    font: TextFont,
    color: TextColor,
}

impl Default for TextNodeBundle {
    fn default() -> Self {
        Self {
            text: Text::default(),
            font: TextFont::default(),
            color: TextColor::default(),
        }
    }
}

#[derive(Bundle)]
pub struct ColoredNodeBundle {
    node: Node,
    background_color: BackgroundColor,
}

impl Default for ColoredNodeBundle {
    fn default() -> Self {
        Self {
            node: Node::default(),
            background_color: BackgroundColor::default(),
        }
    }
}

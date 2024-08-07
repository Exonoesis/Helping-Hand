use crate::visuals::settings_menu::*;
use bevy::prelude::*;

pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
pub const DBROWN: Color = Color::rgb(0.49, 0.29, 0.14);
//pub const LBROWN: Color = Color::rgb(0.72, 0.53, 0.36);

#[derive(Component, Copy, Clone)]
pub enum CountingSliderKeys {
    Handle = 0,
    Fill = 1,
    Value = 2,
    Decrement = 3,
    Increment = 4,
    Back = 5,
}

pub struct Slider {
    pub back: (NodeBundle, CountingSliderKeys),
    pub handle: (
        ButtonBundle,
        ButtonTypes,
        SettingsMenuElements,
        CountingSliderKeys,
    ),
    pub fill: (NodeBundle, CountingSliderKeys),
}

pub struct Spinner {
    pub value_container: NodeBundle,
    pub buttons_container: NodeBundle,
    pub value: (TextBundle, SettingsMenuElements, CountingSliderKeys),
    pub increment: (
        ButtonBundle,
        ButtonTypes,
        SettingsMenuElements,
        CountingSliderKeys,
    ),
    pub decrement: (
        ButtonBundle,
        ButtonTypes,
        SettingsMenuElements,
        CountingSliderKeys,
    ),
}

pub fn create_widget_label(text: String) -> (TextBundle, SettingsMenuElements) {
    (
        TextBundle::from_section(
            text,
            TextStyle {
                font_size: 20.0,
                color: WHITE,
                ..default()
            },
        ),
        SettingsMenuElements::Text,
    )
}

pub fn create_widget_slider() -> Slider {
    Slider {
        back: (
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(20.0),
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(DBROWN),
                ..default()
            },
            CountingSliderKeys::Back,
        ),
        handle: (
            ButtonBundle {
                style: Style {
                    width: Val::Percent(5.00),
                    //Handle does not clip outside widget container because it is
                    //parented to fill and subsequently back which is 20% of the
                    //widget containers height. [200% of 20% = 40%]
                    height: Val::Percent(200.00),
                    ..default()
                },
                ..default()
            },
            ButtonTypes::Slider,
            SettingsMenuElements::Button,
            CountingSliderKeys::Handle,
        ),
        fill: (
            NodeBundle {
                style: Style {
                    width: Val::Percent(0.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: BackgroundColor(WHITE),
                ..default()
            },
            CountingSliderKeys::Fill,
        ),
    }
}

pub fn create_widget_spinner() -> Spinner {
    Spinner {
        value_container: (NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        }),
        buttons_container: (NodeBundle {
            style: Style {
                width: Val::Percent(30.0),
                height: Val::Percent(80.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        }),
        value: (
            TextBundle::from_section(
                "0",
                TextStyle {
                    font_size: 25.0,
                    color: WHITE,
                    ..default()
                },
            ),
            SettingsMenuElements::Text,
            CountingSliderKeys::Value,
        ),
        increment: (
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.00),
                    height: Val::Percent(45.00),
                    ..default()
                },
                ..default()
            },
            ButtonTypes::Increment,
            SettingsMenuElements::IncrementButton,
            CountingSliderKeys::Increment,
        ),
        decrement: (
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.00),
                    height: Val::Percent(45.00),
                    ..default()
                },
                ..default()
            },
            ButtonTypes::Decrement,
            SettingsMenuElements::DecrementButton,
            CountingSliderKeys::Decrement,
        ),
    }
}

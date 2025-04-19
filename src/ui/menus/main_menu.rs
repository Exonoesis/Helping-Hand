use crate::map::interactions::map_changing::CameraBundle;
use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;

use super::{ButtonNodeBundle, ImageNodeBundle, TextNodeBundle};

#[derive(Component)]
pub enum ButtonTypes {
    Play,
    Settings,
    Quit,
}

const WHITE: Color = Color::srgb(1.0, 1.0, 1.0);

pub fn button_system(
    mut exit_event: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &ButtonTypes),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, button_type) in &mut interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match button_type {
            ButtonTypes::Play => next_state.set(AppState::InGame),
            ButtonTypes::Settings => next_state.set(AppState::SettingsMenu),
            ButtonTypes::Quit => {
                exit_event.send(AppExit::Success);
            }
        }
    }
}

#[derive(Component)]
pub struct MainMenuUI;

pub fn spawn_main_menu(mut commands: Commands) {
    let ui_container = (
        ImageNodeBundle {
            node: Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        MainMenuUI,
        MainMenuElements::BackgroundImage,
    );

    let top_half = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(50.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::FlexEnd,
        padding: UiRect {
            left: Val::Percent(0.0),
            right: Val::Percent(0.0),
            top: Val::Percent(0.0),
            bottom: Val::Percent(3.0),
        },
        ..default()
    };

    let title_text = (
        TextNodeBundle {
            text: Text::new("Helping Hand"),
            font: TextFont {
                font_size: 130.0,
                ..default()
            },
            color: TextColor(WHITE),
        },
        MainMenuElements::Text,
    );

    let bottom_half = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(50.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        row_gap: Val::Percent(4.2),
        ..default()
    };

    let play_button = create_button(ButtonTypes::Play);
    let settings_button = create_button(ButtonTypes::Settings);
    let quit_button = create_button(ButtonTypes::Quit);

    let play_text = create_button_text(String::from("Play"));
    let settings_text = create_button_text(String::from("Settings"));
    let quit_text = create_button_text(String::from("Quit"));

    //Spawn UI Camera
    commands.spawn((CameraBundle::default(), MainMenuUI));

    //UI Construction
    commands.spawn(ui_container).with_children(|ui_container| {
        ui_container.spawn(top_half).with_children(|top_half| {
            top_half.spawn(title_text);
        });
        ui_container
            .spawn(bottom_half)
            .with_children(|bottom_half| {
                bottom_half.spawn(play_button).with_children(|play_button| {
                    play_button.spawn(play_text);
                });
                bottom_half
                    .spawn(settings_button)
                    .with_children(|settings_button| {
                        settings_button.spawn(settings_text);
                    });
                bottom_half.spawn(quit_button).with_children(|quit_button| {
                    quit_button.spawn(quit_text);
                });
            });
    });
}

#[derive(Component)]
pub enum MainMenuElements {
    BackgroundImage,
    Button,
    Text,
}

fn create_button(b_type: ButtonTypes) -> (ButtonNodeBundle, ButtonTypes, MainMenuElements) {
    (
        ButtonNodeBundle {
            node: Node {
                width: Val::Percent(23.0),
                height: Val::Percent(23.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        b_type,
        MainMenuElements::Button,
    )
}

fn create_button_text(text: String) -> (TextNodeBundle, MainMenuElements) {
    (
        TextNodeBundle {
            text: Text::new(text),
            font: TextFont {
                font_size: 40.0,
                ..default()
            },
            color: TextColor(WHITE),
        },
        MainMenuElements::Text,
    )
}

pub fn load_background_image(
    asset_server: Res<AssetServer>,
    mut element_query: Query<(&MainMenuElements, &mut ImageNode), Added<MainMenuElements>>,
) {
    for (element, mut image_node) in &mut element_query {
        if let MainMenuElements::BackgroundImage = element {
            image_node.image = asset_server
                .load("textures/main_menu/HH-background.png")
                .into()
        }
    }
}

pub fn load_button_image(
    asset_server: Res<AssetServer>,
    mut element_query: Query<(&MainMenuElements, &mut ImageNode), Added<MainMenuElements>>,
) {
    for (element, mut image_node) in &mut element_query {
        if let MainMenuElements::Button = element {
            image_node.image = asset_server.load("textures/main_menu/button.png").into()
        }
    }
}

pub fn load_text_font(
    asset_server: Res<AssetServer>,
    mut element_query: Query<(&MainMenuElements, &mut TextFont), Added<MainMenuElements>>,
) {
    for (element, mut text) in &mut element_query {
        if let MainMenuElements::Text = element {
            text.font = asset_server.load("fonts/Untitled.ttf")
        }
    }
}

pub fn unload_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

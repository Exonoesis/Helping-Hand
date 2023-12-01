use bevy::prelude::*;

const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);

#[derive(Component)]
pub enum SettingsMenuElements {
    BackgroundImage,
    OptionsBox,
    TabBox,
    Button,
    Text,
}

#[derive(Component)]
pub enum ButtonTypes {
    Apply,
    Cancel,
}

#[derive(Component)]
pub struct SettingsMenuUI;

pub fn spawn_settings_menu(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), SettingsMenuUI));
    commands
        //Node that spans entire screen, acts as container for other UI elements
        .spawn((
            ImageBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            SettingsMenuUI,
            SettingsMenuElements::BackgroundImage,
        ))
        .with_children(|parent| {
            //Node for the top third of the screen
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(9.375),
                        align_items: AlignItems::FlexStart,
                        padding: UiRect {
                            left: Val::Percent(2.0),
                            right: Val::Percent(0.0),
                            top: Val::Percent(1.0),
                            bottom: Val::Percent(0.0),
                        },
                        ..default()
                    },
                    ..default()
                })
                //Node for the title text
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Settings",
                            TextStyle {
                                font_size: 37.0,
                                color: WHITE,
                                ..default()
                            },
                        ),
                        SettingsMenuElements::Text,
                    ));
                });
            //Node for the middle third of the screen
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(66.0),
                        height: Val::Percent(74.995),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        justify_content: JustifyContent::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                //Node for the tabs
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(18.82),
                                    height: Val::Percent(9.434),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    padding: UiRect {
                                        left: Val::Percent(0.0),
                                        right: Val::Percent(0.0),
                                        top: Val::Percent(0.6),
                                        bottom: Val::Percent(0.0),
                                    },
                                    ..default()
                                },
                                ..default()
                            },
                            SettingsMenuElements::TabBox,
                        ))
                        //Node for tab text
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "Audio",
                                    TextStyle {
                                        font_size: 29.0,
                                        color: WHITE,
                                        ..default()
                                    },
                                ),
                                SettingsMenuElements::Text,
                            ));
                        });
                })
                //Node for the options container
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    justify_content: JustifyContent::FlexStart,
                                    align_items: AlignItems::FlexStart,
                                    padding: UiRect {
                                        left: Val::Percent(7.0),
                                        right: Val::Percent(0.0),
                                        top: Val::Percent(3.0),
                                        bottom: Val::Percent(0.0),
                                    },
                                    ..default()
                                },
                                ..default()
                            },
                            SettingsMenuElements::OptionsBox,
                        ))
                        //Node for individual settings options
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "Music",
                                    TextStyle {
                                        font_size: 25.0,
                                        color: WHITE,
                                        ..default()
                                    },
                                ),
                                SettingsMenuElements::Text,
                            ));
                        });
                });
            //Node for the bottom third of the screen
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(66.0),
                        height: Val::Percent(15.63),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::FlexStart,
                        justify_content: JustifyContent::FlexEnd,
                        column_gap: Val::Percent(2.0),
                        padding: UiRect {
                            left: Val::Percent(0.0),
                            right: Val::Percent(0.0),
                            top: Val::Percent(0.8),
                            bottom: Val::Percent(0.0),
                        },
                        ..default()
                    },
                    ..default()
                })
                //Node for Apply button
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(145.0),
                                    height: Val::Px(45.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            ButtonTypes::Apply,
                            SettingsMenuElements::Button,
                        ))
                        //Node for Apply text
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "Apply",
                                    TextStyle {
                                        font_size: 25.0,
                                        color: WHITE,
                                        ..default()
                                    },
                                ),
                                SettingsMenuElements::Text,
                            ));
                        });
                })
                //Node for Cancel button
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(145.0),
                                    height: Val::Px(45.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            ButtonTypes::Cancel,
                            SettingsMenuElements::Button,
                        ))
                        //Node for Cancel text
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "Cancel",
                                    TextStyle {
                                        font_size: 25.0,
                                        color: WHITE,
                                        ..default()
                                    },
                                ),
                                SettingsMenuElements::Text,
                            ));
                        });
                });
        });
}

pub fn load_background_image(
    asset_server: Res<AssetServer>,
    mut element_query: Query<(&SettingsMenuElements, &mut UiImage), Added<SettingsMenuElements>>,
) {
    for (element, mut image) in &mut element_query {
        if let SettingsMenuElements::BackgroundImage = element {
            *image = asset_server
                .load("textures/main_menu/HH-background.png")
                .into()
        }
    }
}

pub fn load_box_image(
    asset_server: Res<AssetServer>,
    mut element_query: Query<(&SettingsMenuElements, &mut UiImage), Added<SettingsMenuElements>>,
) {
    for (element, mut image) in &mut element_query {
        if let SettingsMenuElements::OptionsBox = element {
            *image = asset_server
                .load("textures/settings_menu/window-box.png")
                .into()
        }
    }
}

pub fn load_tab_image(
    asset_server: Res<AssetServer>,
    mut element_query: Query<(&SettingsMenuElements, &mut UiImage), Added<SettingsMenuElements>>,
) {
    for (element, mut image) in &mut element_query {
        if let SettingsMenuElements::TabBox = element {
            *image = asset_server
                .load("textures/settings_menu/tab-box.png")
                .into()
        }
    }
}

pub fn load_button_image(
    asset_server: Res<AssetServer>,
    mut element_query: Query<(&SettingsMenuElements, &mut UiImage), Added<SettingsMenuElements>>,
) {
    for (element, mut image) in &mut element_query {
        if let SettingsMenuElements::Button = element {
            *image = asset_server.load("textures/main_menu/button.png").into()
        }
    }
}

pub fn load_text_font(
    asset_server: Res<AssetServer>,
    mut element_query: Query<(&SettingsMenuElements, &mut Text), Added<SettingsMenuElements>>,
) {
    for (element, mut text) in &mut element_query {
        if let SettingsMenuElements::Text = element {
            text.sections[0].style.font = asset_server.load("fonts/Huglove.ttf")
        }
    }
}

pub fn unload_settings_menu(mut commands: Commands, query: Query<Entity, With<SettingsMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_settings_menu_build_and_cleanup_checking() -> App {
        let mut app = App::new();

        //We test this as a startup system because we cannot test states directly
        app.add_systems(Startup, spawn_settings_menu);

        app
    }

    #[test]
    fn settings_menu_build_and_cleanup_checking() {
        //No entities should exist at this point
        let mut app = setup_settings_menu_build_and_cleanup_checking();
        let mut item_num = app.world.entities().len();
        assert_eq!(0, item_num);

        //Settings Menu entities should now be loaded
        app.update();
        item_num = app.world.entities().len();
        assert!(item_num > 0);

        //Now we call our unload Settings Menu function...
        app.add_systems(Update, unload_settings_menu);
        app.update();

        //and ensure that no entities remain
        item_num = app.world.entities().len();
        assert_eq!(0, item_num);
    }
}

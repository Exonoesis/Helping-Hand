use crate::mechanics::custom_widgets::*;
use bevy::prelude::*;

#[derive(Component)]
pub enum SettingsMenuElements {
    BackgroundImage,
    OptionsBox,
    TabBox,
    Button,
    IncrementButton,
    DecrementButton,
    Text,
}

#[derive(Component, PartialEq)]
pub enum ButtonTypes {
    Apply,
    Cancel,
    Slider,
    Increment,
    Decrement,
}

#[derive(Component)]
pub struct SliderKeyComponents {
    pub array: [Option<Entity>; 6],
}

#[derive(Component, Clone, Copy)]
pub struct FillReference(pub Entity);

#[derive(Component, Clone, Copy)]
pub struct ValueReference(pub Entity);

#[derive(Component, Clone, Copy)]
pub struct HandleReference(pub Entity);

#[derive(Component, Clone, Copy)]
pub struct BackReference(pub Entity);

#[derive(Component)]
pub struct SettingsMenuUI;

pub fn spawn_settings_menu(mut commands: Commands) {
    let ui_container = (
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
    );

    let top_third = NodeBundle {
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
    };

    let title_text = (
        TextBundle::from_section(
            "Settings",
            TextStyle {
                font_size: 30.0,
                color: WHITE,
                ..default()
            },
        ),
        SettingsMenuElements::Text,
    );

    let middle_third = NodeBundle {
        style: Style {
            width: Val::Percent(66.0),
            height: Val::Percent(74.995),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexStart,
            ..default()
        },
        ..default()
    };

    let tabs_container = (
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
    );

    let tab_text = (
        TextBundle::from_section(
            "Audio",
            TextStyle {
                font_size: 25.0,
                color: WHITE,
                ..default()
            },
        ),
        SettingsMenuElements::Text,
    );

    let options_container = (
        ImageBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                padding: UiRect {
                    left: Val::Percent(0.0),
                    right: Val::Percent(0.0),
                    top: Val::Percent(2.0),
                    bottom: Val::Percent(0.0),
                },
                row_gap: Val::Percent(2.00),
                ..default()
            },
            ..default()
        },
        SettingsMenuElements::OptionsBox,
    );

    let music_widget_label = NodeBundle {
        style: Style {
            width: Val::Percent(25.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };

    let music_slider_container = NodeBundle {
        style: Style {
            width: Val::Percent(60.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };

    let music_spinner_container = NodeBundle {
        style: Style {
            width: Val::Percent(15.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    let music_widget_text = create_widget_label(String::from("Music"));
    let music_slider = create_widget_slider();
    let music_spinner = create_widget_spinner();

    let mut music_widget_keys = SliderKeyComponents { array: [None; 6] };
    let mut sfx_widget_keys = SliderKeyComponents { array: [None; 6] };

    let bottom_third = NodeBundle {
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
    };

    let apply_button = create_button(ButtonTypes::Apply);
    let apply_text = create_button_text(String::from("Apply"));

    let cancel_button = create_button(ButtonTypes::Cancel);
    let cancel_text = create_button_text(String::from("Cancel"));

    //Spawn UI Camera
    commands.spawn((Camera2dBundle::default(), SettingsMenuUI));

    //UI Construction
    commands.spawn(ui_container).with_children(|ui_container| {
        ui_container.spawn(top_third).with_children(|top_third| {
            top_third.spawn(title_text);
        });
        ui_container
            .spawn(middle_third)
            .with_children(|middle_third| {
                middle_third
                    .spawn(tabs_container)
                    .with_children(|tabs_container| {
                        tabs_container.spawn(tab_text);
                    });
                middle_third
                    .spawn(options_container)
                    .with_children(|options_container| {
                        options_container
                            .spawn(create_widget_container(music_widget_keys))
                            .with_children(|widget_container| {
                                widget_container.spawn(music_widget_label).with_children(
                                    |widget_label| {
                                        widget_label.spawn(music_widget_text);
                                    },
                                );
                            })
                            .with_children(|widget_container| {
                                widget_container
                                    .spawn(music_slider_container)
                                    .with_children(|music_slider_container| {
                                        music_slider_container
                                            .spawn(music_slider.back)
                                            .with_children(|music_slider_back| {
                                                music_slider_back.spawn(music_slider.fill);
                                                music_slider_back.spawn(music_slider.handle);
                                            });
                                    });
                            })
                            .with_children(|widget_container| {
                                widget_container
                                    .spawn(music_spinner_container)
                                    .with_children(|music_spinner_container| {
                                        music_spinner_container
                                            .spawn(music_spinner.value_container)
                                            .with_children(|music_spinner_value_container| {
                                                music_spinner_value_container
                                                    .spawn(music_spinner.value);
                                            });
                                    })
                                    .with_children(|music_spinner_container| {
                                        music_spinner_container
                                            .spawn(music_spinner.buttons_container)
                                            .with_children(|music_spinner_buttons_container| {
                                                music_spinner_buttons_container
                                                    .spawn(music_spinner.increment);
                                            })
                                            .with_children(|music_spinner_buttons_container| {
                                                music_spinner_buttons_container
                                                    .spawn(music_spinner.decrement);
                                            });
                                    });
                            });
                    })
                    .with_children(|options_container| {
                        options_container.spawn(create_widget_container(sfx_widget_keys));
                    });
            });
        ui_container
            .spawn(bottom_third)
            .with_children(|bottom_third| {
                bottom_third
                    .spawn(apply_button)
                    .with_children(|apply_button| {
                        apply_button.spawn(apply_text);
                    });
                bottom_third
                    .spawn(cancel_button)
                    .with_children(|cancel_button| {
                        cancel_button.spawn(cancel_text);
                    });
            });
    });
}

fn create_button(b_type: ButtonTypes) -> (ButtonBundle, ButtonTypes, SettingsMenuElements) {
    (
        ButtonBundle {
            style: Style {
                width: Val::Percent(11.33),
                height: Val::Percent(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        b_type,
        SettingsMenuElements::Button,
    )
}

fn create_button_text(text: String) -> (TextBundle, SettingsMenuElements) {
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

fn create_widget_container(keys: SliderKeyComponents) -> (NodeBundle, SliderKeyComponents) {
    (
        NodeBundle {
            style: Style {
                width: Val::Percent(96.0),
                height: Val::Percent(12.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        },
        keys,
    )
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

pub fn load_increment_button_image(
    asset_server: Res<AssetServer>,
    mut element_query: Query<(&SettingsMenuElements, &mut UiImage), Added<SettingsMenuElements>>,
) {
    for (element, mut image) in &mut element_query {
        if let SettingsMenuElements::IncrementButton = element {
            *image = asset_server
                .load("textures/settings_menu/increment-button.png")
                .into()
        }
    }
}

pub fn load_decrement_button_image(
    asset_server: Res<AssetServer>,
    mut element_query: Query<(&SettingsMenuElements, &mut UiImage), Added<SettingsMenuElements>>,
) {
    for (element, mut image) in &mut element_query {
        if let SettingsMenuElements::DecrementButton = element {
            *image = asset_server
                .load("textures/settings_menu/decrement-button.png")
                .into()
        }
    }
}

pub fn load_text_font(
    asset_server: Res<AssetServer>,
    mut element_query: Query<(&SettingsMenuElements, &mut Text), Added<SettingsMenuElements>>,
) {
    for (element, mut text) in &mut element_query {
        if let SettingsMenuElements::Text = element {
            text.sections[0].style.font = asset_server.load("fonts/Untitled.ttf")
        }
    }
}

pub fn unload_settings_menu(mut commands: Commands, query: Query<Entity, With<SettingsMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn set_keys(
    entity_query: Query<(Entity, &CountingSliderKeys), Added<CountingSliderKeys>>,
    parent_query: Query<&Parent>,
    mut widget_containers_query: Query<&mut SliderKeyComponents>,
) {
    for (entity, key) in &entity_query {
        for parent in parent_query.iter_ancestors(entity) {
            if let Ok(mut counting_slider) = widget_containers_query.get_mut(parent) {
                counting_slider.array[*key as usize] = Some(entity);
                break;
            }
        }
    }
}

pub fn add_widget_components(
    key_components_query: Query<&SliderKeyComponents, Changed<SliderKeyComponents>>,
    mut commands: Commands,
) {
    for key_components in key_components_query.iter() {
        let is_not_full = key_components
            .array
            .iter()
            .any(|key_component| key_component.is_none());

        if is_not_full {
            continue;
        }

        let fill = FillReference(
            key_components.array[1].expect("add_widget_components: Fill does not exist"),
        );
        let value = ValueReference(
            key_components.array[2].expect("add_widget_components: Value does not exist"),
        );
        let handle_ref = HandleReference(
            key_components.array[0].expect("add_widget_componenets: Handle does not exist"),
        );
        let back = BackReference(
            key_components.array[5].expect("add_widget_components: Back does not exist"),
        );

        let handle = key_components.array[0];
        let increment = key_components.array[4];
        let decrement = key_components.array[3];

        commands
            .entity(handle.expect("add_widget_components: Handle does not exist"))
            .insert(fill)
            .insert(value)
            .insert(back);

        commands
            .entity(increment.expect("add_widget_components: Increment does not exist"))
            .insert(fill)
            .insert(value)
            .insert(handle_ref);

        commands
            .entity(decrement.expect("add_widget_components: Decrement does not exist"))
            .insert(fill)
            .insert(value)
            .insert(handle_ref);
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

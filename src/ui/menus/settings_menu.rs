use bevy::{prelude::*, window::PrimaryWindow};
use bevy_kira_audio::{AudioChannel, AudioControl};

use crate::{audio::music::*, map::player::*, AppState};

// Setup
pub const WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
pub const DBROWN: Color = Color::srgb(0.49, 0.29, 0.14);
//pub const LBROWN: Color = Color::rgb(0.72, 0.53, 0.36);

#[derive(Component, PartialEq)]
pub enum ButtonTypes {
    Apply,
    Cancel,
    Slider,
    Increment,
    Decrement,
}

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

#[derive(Component, Copy, Clone)]
pub enum CountingSliderKeys {
    Handle = 0,
    Fill = 1,
    Value = 2,
    Decrement = 3,
    Increment = 4,
    Back = 5,
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

#[derive(Component, PartialEq)]
pub enum AudioType {
    Music,
    SFX,
}

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
                        spawn_volume_slider(options_container, AudioType::Music);
                        spawn_volume_slider(options_container, AudioType::SFX);
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

pub struct CountingSlider {
    barrier_keys: SliderKeyComponents,

    label: (TextBundle, SettingsMenuElements),
    slider: Slider,
    spinner: Spinner,
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

/// Creates a Counting Slider with a Label.
fn create_counting_slider(label: String) -> CountingSlider {
    let slider_widget_keys = SliderKeyComponents { array: [None; 6] };

    let slider_widget_text = create_widget_label(label);
    let slider = create_widget_slider();
    let spinner = create_widget_spinner();

    CountingSlider {
        barrier_keys: slider_widget_keys,

        label: slider_widget_text,
        slider,
        spinner,
    }
}

/// Spawns a Volume Slider at a given point in the UI.
fn spawn_volume_slider(ui_container: &mut ChildBuilder, audio_type: AudioType) {
    let volume_slider = match audio_type {
        AudioType::Music => create_counting_slider(String::from("Music")),
        AudioType::SFX => create_counting_slider(String::from("SFX")),
    };

    let slider_widget_label = NodeBundle {
        style: Style {
            width: Val::Percent(25.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };

    let slider_container = NodeBundle {
        style: Style {
            width: Val::Percent(60.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };

    let spinner_container = NodeBundle {
        style: Style {
            width: Val::Percent(15.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    ui_container
        .spawn(create_widget_container(volume_slider.barrier_keys))
        .with_children(|widget_container| {
            widget_container
                .spawn(slider_widget_label)
                .with_children(|widget_label| {
                    widget_label.spawn(volume_slider.label);
                });
        })
        .with_children(|widget_container| {
            widget_container
                .spawn(slider_container)
                .with_children(|slider_container| {
                    slider_container
                        .spawn(volume_slider.slider.back)
                        .with_children(|slider_back| {
                            slider_back.spawn(volume_slider.slider.fill);
                            slider_back.spawn(volume_slider.slider.handle);
                        });
                });
        })
        .with_children(|widget_container| {
            widget_container
                .spawn(spinner_container)
                .with_children(|spinner_container| {
                    spinner_container
                        .spawn(volume_slider.spinner.value_container)
                        .with_children(|spinner_value_container| {
                            spinner_value_container
                                .spawn((volume_slider.spinner.value, audio_type));
                        });
                })
                .with_children(|spinner_container| {
                    spinner_container
                        .spawn(volume_slider.spinner.buttons_container)
                        .with_children(|spinner_buttons_container| {
                            spinner_buttons_container.spawn(volume_slider.spinner.increment);
                            spinner_buttons_container.spawn(volume_slider.spinner.decrement);
                        });
                });
        });
}

pub fn change_music_volume(
    spinner_query: Query<(&Text, &AudioType), Changed<Text>>,
    background_music: Res<AudioChannel<MusicChannel>>,
) {
    for (text, audio_type) in &spinner_query {
        if *audio_type != AudioType::Music {
            continue;
        }

        let slider_percentage = get_percentage_from(text.clone());

        background_music.set_volume(slider_percentage);
    }
}

pub fn change_sfx_volumes(
    spinner_query: Query<(&Text, &AudioType), Changed<Text>>,
    player_movement_sound: Res<AudioChannel<PlayerWalkChannel>>,
    player_bump_sound: Res<AudioChannel<PlayerBumpChannel>>,
) {
    for (text, slider_type) in &spinner_query {
        if *slider_type != AudioType::SFX {
            continue;
        }

        let slider_percentage = get_percentage_from(text.clone());

        player_movement_sound.set_volume(slider_percentage);
        player_bump_sound.set_volume(slider_percentage);
    }
}

pub fn get_percentage_from(spinner_value: Text) -> f64 {
    let value = spinner_value.sections[0].value.parse::<f64>().unwrap();

    //Audio is 0-1 normalized so we convert to a decimal percentage
    let percentage_value = value * 0.01;

    return percentage_value;
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

// Behavior

#[derive(Component, Clone, Copy)]
pub struct BeingClicked {
    original_x: f32,
    original_val: f32,
    handle_width: Val,
}

pub fn spinner_buttons_system(
    mut spinner_button_query: Query<
        (
            &Interaction,
            &ButtonTypes,
            &ValueReference,
            &FillReference,
            &HandleReference,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut spinner_value_query: Query<
        &mut Text,
        (With<SettingsMenuElements>, With<CountingSliderKeys>),
    >,
    mut spinner_fill_query: Query<&mut Style, (With<CountingSliderKeys>, Without<Button>)>,
    spinner_handle_query: Query<&Style, (With<CountingSliderKeys>, With<Button>)>,
) {
    for (interaction, button_type, value_reference, fill_reference, handle_reference) in
        &mut spinner_button_query
    {
        let is_spinner_button =
            *button_type == ButtonTypes::Increment || *button_type == ButtonTypes::Decrement;

        if *interaction != Interaction::Pressed || !is_spinner_button {
            continue;
        }

        let mut spinner_value = spinner_value_query
            .get_mut(value_reference.0)
            .expect("spinner_buttons_system: Spinner value should exist.");

        let old_value = spinner_value.sections[0].value.parse::<u32>().unwrap();

        let trying_to_overflow = *button_type == ButtonTypes::Increment && old_value == 100;
        let trying_to_underflow = *button_type == ButtonTypes::Decrement && old_value == 0;

        if trying_to_overflow || trying_to_underflow {
            continue;
        }

        let new_value = if *button_type == ButtonTypes::Increment {
            old_value + 1
        } else if *button_type == ButtonTypes::Decrement {
            old_value - 1
        } else {
            panic!("spinner_buttons_system: ButtonTypes was not Increment or Drecement, which is impossible.")
        };

        spinner_value.sections[0].value = new_value.to_string();

        let mut fill_value = spinner_fill_query
            .get_mut(fill_reference.0)
            .expect("spinner_buttons_system: Spinner fill value should exist.");

        /*
        We subtract half the handles width from the fill bar so that the values
        position is visually accurate to the center of the handle, as is expected,
        rather than the right edge of the fill bar
        */
        let handle_style = spinner_handle_query
            .get(handle_reference.0)
            .expect("spinner_buttons_system: Spinner handle should exist.");

        let handle_width_percentage = if let Val::Percent(width_percentage) = handle_style.width {
            width_percentage
        } else {
            panic!("spinner_buttons_system: Handle width should be a percentage.")
        };

        let new_fill_amount = spinner_value.sections[0].value.parse::<f32>().unwrap()
            - (handle_width_percentage / 2.0);

        fill_value.width = Val::Percent(new_fill_amount);
    }
}

pub fn get_handle_click_position(
    mut slider_handle_query: Query<
        (
            &Interaction,
            &ButtonTypes,
            &ValueReference,
            Entity,
            &mut Style,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut slider_value_query: Query<
        &mut Text,
        (With<SettingsMenuElements>, With<CountingSliderKeys>),
    >,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    //Capture handle click
    for (interaction, button_type, value_reference, handle, style) in &mut slider_handle_query {
        if *interaction != Interaction::Pressed || *button_type != ButtonTypes::Slider {
            commands.entity(handle).remove::<BeingClicked>();
            continue;
        }

        //Capture mouse.x position at time of click
        let original_x_position = if let Some(position) = window_query.single().cursor_position() {
            position.x
        } else {
            panic!("slider_handle_system: Handle was clicked while mouse was outside game window")
        };

        //Capture original spinner value at time of click
        let spinner_value = slider_value_query
            .get_mut(value_reference.0)
            .expect("get_handle_click_position: Spinner value should exist.");

        let original_spinner_value = spinner_value.sections[0].value.parse::<f32>().unwrap();

        //Attach original_mouse_x to Handle as component
        let original_mouse_x_reference = BeingClicked {
            original_x: original_x_position,
            original_val: original_spinner_value,
            handle_width: style.width,
        };

        commands.entity(handle).insert(original_mouse_x_reference);
    }
}

pub fn update_handle_position_on_hold(
    slider_handle_query: Query<
        (
            &BeingClicked,
            &FillReference,
            &ValueReference,
            &BackReference,
        ),
        (With<BeingClicked>, With<Button>),
    >,
    mut slider_value_query: Query<
        &mut Text,
        (With<SettingsMenuElements>, With<CountingSliderKeys>),
    >,
    mut width_query: Query<&mut Style>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<&Camera>,
    parent_query: Query<&Parent>,
    mouse_button_state: Res<ButtonInput<MouseButton>>,
) {
    //Detect if left mouse button is being held down
    if !mouse_button_state.pressed(MouseButton::Left) {
        return;
    }

    //Get current mouse.x
    let current_x_position = if let Some(position) = window_query.single().cursor_position() {
        position.x
    } else {
        //Doesn't matter if mouse goes outside game window (values beyond slider extremes will be capped)
        return;
    };

    //Get original mouse.x + spinner and fill references
    if slider_handle_query.is_empty() {
        return;
    }

    let handle_data_references = if let Ok(data) = slider_handle_query.get_single() {
        data
    } else {
        panic!("update_handle_position_on_hold: Expected single cursor")
    };

    let original_values = handle_data_references.0;
    let fill_reference = handle_data_references.1;
    let spinner_value_reference = handle_data_references.2;
    let back_reference = handle_data_references.3;

    let read_only_width_query = width_query.to_readonly();

    //Calculate change in mouse x movement as a percent
    let back_width_in_px = get_node_width(
        back_reference.0,
        read_only_width_query,
        camera_query,
        parent_query,
    );

    let change_as_percent =
        (((current_x_position - original_values.original_x) / back_width_in_px) * 100.00).trunc();

    //Change spinner value
    let mut spinner_value = slider_value_query
        .get_mut(spinner_value_reference.0)
        .expect("update_handle_position_on_hold: Spinner value should exist.");

    let mut new_spinner_value = original_values.original_val + change_as_percent;

    new_spinner_value = new_spinner_value.clamp(0.00, 100.00);

    spinner_value.sections[0].value = new_spinner_value.to_string();

    //Change fill bar width
    let mut fill_value = width_query
        .get_mut(fill_reference.0)
        .expect("update_handle_position_on_hold: Slider fill value should exist.");

    /*
    We subtract half the handles width from the fill bar so that the values
    position is visually accurate to the center of the handle, as is expected,
    rather than the right edge of the fill bar
    */
    let handle_width_percentage =
        if let Val::Percent(width_percentage) = original_values.handle_width {
            width_percentage
        } else {
            panic!("spinner_buttons_system: Handle width should be a percentage.")
        };

    let new_fill_amount =
        spinner_value.sections[0].value.parse::<f32>().unwrap() - (handle_width_percentage / 2.0);

    fill_value.width = Val::Percent(new_fill_amount);
}

fn get_node_width(
    node: Entity,
    width_query: Query<&Style>,
    camera_query: Query<&Camera>,
    parent_query: Query<&Parent>,
) -> f32 {
    let mut value_stack = get_all_ancestors(node, parent_query, width_query);

    let camera = camera_query.single();

    let viewport_size = camera.logical_viewport_size().unwrap();
    let mut node_width = viewport_size.x;

    while let Some(val) = value_stack.pop() {
        node_width = val
            .resolve(node_width, viewport_size)
            .expect("get_node_width: Node width could not be resolved");
    }

    return node_width;
}

fn get_all_ancestors(
    node: Entity,
    parent_query: Query<&Parent>,
    width_query: Query<&Style>,
) -> Vec<bevy::ui::Val> {
    let mut to_be_visited_nodes = Vec::new();
    let mut seen_styles = Vec::new();

    //Build stack
    to_be_visited_nodes.push(node);

    while let Some(current_node) = to_be_visited_nodes.pop() {
        let current_node_value = width_query
            .get(current_node)
            .expect("get_all_ancestors: Could not get Style for node")
            .width;

        seen_styles.push(current_node_value);

        if let Ok(parent) = parent_query.get(current_node) {
            to_be_visited_nodes.push(parent.get());
        }
    }

    return seen_styles;
}

pub fn save_button_system(
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
            ButtonTypes::Apply => next_state.set(AppState::MainMenu), //TO-DO Save player preference changes
            ButtonTypes::Cancel => next_state.set(AppState::MainMenu),
            ButtonTypes::Slider => (),
            _ => continue,
        }
    }
}

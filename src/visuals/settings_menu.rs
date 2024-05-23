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

#[derive(Component, PartialEq)]
pub enum SliderType {
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
                        spawn_counting_slider(options_container, String::from("Music"));
                        spawn_counting_slider(options_container, String::from("SFX"));
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

/// Inserts a Counting Slider with a Label at the given point in the UI.
fn spawn_counting_slider(ui_container: &mut ChildBuilder, label: String) {
    let slider_widget_keys = SliderKeyComponents { array: [None; 6] };

    let slider_type = match label.as_str() {
        "Music" => SliderType::Music,
        "SFX" => SliderType::SFX,
        _ => panic!("spawn_counting_slider: Label found does not match any current slider type"),
    };

    let slider_widget_text = create_widget_label(label);
    let slider_widget_label = NodeBundle {
        style: Style {
            width: Val::Percent(25.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };

    let slider = create_widget_slider();
    let slider_container = NodeBundle {
        style: Style {
            width: Val::Percent(60.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    };

    let spinner = create_widget_spinner();
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
        .spawn((create_widget_container(slider_widget_keys), slider_type))
        .with_children(|widget_container| {
            widget_container
                .spawn(slider_widget_label)
                .with_children(|widget_label| {
                    widget_label.spawn(slider_widget_text);
                });
        })
        .with_children(|widget_container| {
            widget_container
                .spawn(slider_container)
                .with_children(|slider_container| {
                    slider_container
                        .spawn(slider.back)
                        .with_children(|slider_back| {
                            slider_back.spawn(slider.fill);
                            slider_back.spawn(slider.handle);
                        });
                });
        })
        .with_children(|widget_container| {
            widget_container
                .spawn(spinner_container)
                .with_children(|spinner_container| {
                    spinner_container
                        .spawn(spinner.value_container)
                        .with_children(|spinner_value_container| {
                            spinner_value_container.spawn(spinner.value);
                        });
                })
                .with_children(|spinner_container| {
                    spinner_container
                        .spawn(spinner.buttons_container)
                        .with_children(|spinner_buttons_container| {
                            spinner_buttons_container.spawn(spinner.increment);
                            spinner_buttons_container.spawn(spinner.decrement);
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

fn get_percentage_from(spinner_value: Text) -> f32 {
    let value = spinner_value.sections[0].value.parse::<f32>().unwrap();

    let percentage_value = value * 0.01;

    return percentage_value;
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

    struct HelpingHand {
        app: App,
    }

    enum SoundSource {
        Music,
    }

    impl HelpingHand {
        pub fn new() -> HelpingHand {
            let mut app = App::new();

            //We test this as a startup system because we cannot test states directly
            app.add_systems(Startup, spawn_settings_menu);

            HelpingHand { app }
        }

        pub fn find_slider(&mut self, slider_type: SliderType) -> Entity {
            self.app.update();

            let slider_entity = self
                .app
                .world
                .query::<(Entity, &SliderType)>()
                .iter(&self.app.world)
                .find(|sliders| *sliders.1 == slider_type)
                .expect("find_slider: Could not find Slider with given type")
                .0;

            slider_entity
        }

        pub fn move_slider_to(&mut self, slider: Entity, percentage: usize) {
            todo!("HelpingHand move_slider_to: Need to implement this.")
        }

        pub fn get_slider_percentage_of(&self, slider: Entity) -> usize {
            todo!("HelpingHand get_slider_percentage_of: Need to implement this.")
        }

        pub fn get_volume_of(&self, sound_source: SoundSource) -> usize {
            todo!("HelpingHand get_volume_of: Need to implement this.")
        }
    }

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

    #[test]
    fn get_0_percent_from_text_field() {
        let spinner_text_bundle = TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 25.0,
                color: WHITE,
                ..default()
            },
        );

        let spinner_text_field = spinner_text_bundle.text;

        let expected_percentage = 0.0;
        let actual_percentage = get_percentage_from(spinner_text_field);

        assert_eq!(expected_percentage, actual_percentage);
    }

    #[test]
    fn get_50_percent_from_text_field() {
        let spinner_text_bundle = TextBundle::from_section(
            "50",
            TextStyle {
                font_size: 25.0,
                color: WHITE,
                ..default()
            },
        );

        let spinner_text_field = spinner_text_bundle.text;

        let expected_percentage = 0.5;
        let actual_percentage = get_percentage_from(spinner_text_field);

        assert_eq!(expected_percentage, actual_percentage);
    }

    #[test]
    fn get_100_percent_from_text_field() {
        let spinner_text_bundle = TextBundle::from_section(
            "100",
            TextStyle {
                font_size: 25.0,
                color: WHITE,
                ..default()
            },
        );

        let spinner_text_field = spinner_text_bundle.text;

        let expected_percentage = 1.0;
        let actual_percentage = get_percentage_from(spinner_text_field);

        assert_eq!(expected_percentage, actual_percentage);
    }

    // TODO: Implement the rest of these methods with TDD
    // for the 'Introduce Tests for Existing Features'
    // Feature Request/GitHub Issue types.
    //
    //#[test]
    //fn music_slider_changes_volume() {
    //    let mut game = HelpingHand::new();

    //    let slider = game.find_slider(SliderType::Music);
    //    let percentage = 50;

    //    game.move_slider_to(slider, percentage);

    //    let slider_percentage = game.get_slider_percentage_of(slider);
    //    let volume_percentage = game.get_volume_of(SoundSource::Music);

    //    assert_eq!(percentage, volume_percentage);
    //    assert_eq!(slider_percentage, volume_percentage);
    //}
}

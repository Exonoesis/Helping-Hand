use bevy::prelude::*;

const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);

#[derive(Component)]
pub enum MainMenuElements {
    BackgroundImage,
    Button,
    Text,
}

#[derive(Component)]
pub enum ButtonTypes {
    Play,
    Settings,
    Quit,
}

#[derive(Component)]
pub struct MainMenuUI;

pub fn spawn_main_menu(mut commands: Commands) {
    let ui_container = (ImageBundle {
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
    MainMenuUI,
    MainMenuElements::BackgroundImage);

    let top_half = NodeBundle {
        style: Style {
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
        },
        ..default()
    };

    let title_text = (
        TextBundle::from_section(
            "Helping Hand",
            TextStyle {
                font_size: 180.0,
                color: WHITE,
                ..default()
            },
        ),
        MainMenuElements::Text
    );

    let bottom_half = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(50.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Percent(4.2),
            ..default()
        },
        ..default()
    };

    let play_button = create_button(ButtonTypes::Play);
    let settings_button = create_button(ButtonTypes::Settings);
    let quit_button = create_button(ButtonTypes::Quit);

    let play_text = create_button_text(String::from("Play"));
    let settings_text = create_button_text(String::from("Settings"));
    let quit_text = create_button_text(String::from("Quit"));

    //Spawn UI Camera
    commands.spawn((Camera2dBundle::default(), MainMenuUI));

    //UI Construction
    commands
        .spawn(ui_container)
        .with_children(|ui_container| {
            ui_container
                .spawn(top_half)
                .with_children(|top_half| {
                    top_half.spawn(title_text);
                });
            ui_container
                .spawn(bottom_half)
                .with_children(|bottom_half| {
                    bottom_half
                        .spawn(play_button)
                        .with_children(|play_button| {
                            play_button.spawn(play_text);
                        });
                })
                .with_children(|bottom_half| {
                    bottom_half
                        .spawn(settings_button)
                        .with_children(|settings_button| {
                            settings_button.spawn(settings_text);
                        });
                })
                .with_children(|bottom_half| {
                    bottom_half
                        .spawn(quit_button)
                        .with_children(|quit_button| {
                            quit_button.spawn(quit_text);
                        });
                });
        });
}

fn create_button (b_type: ButtonTypes) -> (ButtonBundle, ButtonTypes, MainMenuElements)
{
    (
        ButtonBundle {
            style: Style {
                width: Val::Percent(23.0),
                height: Val::Percent(23.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        b_type,
        MainMenuElements::Button
    )
}

fn create_button_text (text: String) -> (TextBundle, MainMenuElements)
{
    (
        TextBundle::from_section(
            text,
            TextStyle {
                font_size: 80.0,
                color: WHITE,
                ..default()
            },
        ),
        MainMenuElements::Text
    )
}

pub fn load_background_image(
    asset_server: Res<AssetServer>,
    mut element_query: Query<(&MainMenuElements, &mut UiImage), Added<MainMenuElements>>,
) {
    for (element, mut image) in &mut element_query {
        if let MainMenuElements::BackgroundImage = element {
            *image = asset_server
                .load("textures/main_menu/HH-background.png")
                .into()
        }
    }
}

pub fn load_button_image(
    asset_server: Res<AssetServer>,
    mut element_query: Query<(&MainMenuElements, &mut UiImage), Added<MainMenuElements>>,
) {
    for (element, mut image) in &mut element_query {
        if let MainMenuElements::Button = element {
            *image = asset_server.load("textures/main_menu/button.png").into()
        }
    }
}

pub fn load_text_font(
    asset_server: Res<AssetServer>,
    mut element_query: Query<(&MainMenuElements, &mut Text), Added<MainMenuElements>>,
) {
    for (element, mut text) in &mut element_query {
        if let MainMenuElements::Text = element {
            text.sections[0].style.font = asset_server.load("fonts/Huglove.ttf")
        }
    }
}

pub fn unload_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_main_menu_build_and_cleanup_checking() -> App {
        let mut app = App::new();

        //We test this as a startup system because we cannot test states directly
        app.add_systems(Startup, spawn_main_menu);

        app
    }

    #[test]
    fn main_menu_build_and_cleanup_checking() {
        //No entities should exist at this point
        let mut app = setup_main_menu_build_and_cleanup_checking();
        let mut item_num = app.world.entities().len();
        assert_eq!(0, item_num);

        //Main Menu entities should now be loaded
        app.update();
        item_num = app.world.entities().len();
        assert!(item_num > 0);

        //Now we call our unload Main Menu function...
        app.add_systems(Update, unload_main_menu);
        app.update();

        //and ensure that no entities remain
        item_num = app.world.entities().len();
        assert_eq!(0, item_num);
    }
}

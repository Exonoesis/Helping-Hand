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
    Quit,
}

#[derive(Component)]
pub struct MainMenuUI;

pub fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainMenuUI));
    commands
        //Node that spans entire screen, acts as container for other UI elements
        .spawn((ImageBundle {
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
        MainMenuElements::BackgroundImage,
        ))
            .with_children(|parent| {
                //Node for the top half of the screen
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        padding: UiRect {
                            left: Val::Percent(0.0),
                            right: Val::Percent(0.0),
                            top: Val::Percent(0.0),
                            bottom: Val::Percent(3.0)
                        },
                        ..default()
                    },
                    ..default()
                })
                    //Node for the title text
                    .with_children(|parent|{
                        parent.spawn((TextBundle::from_section (
                            "Helping Hand",
                            TextStyle {
                                font_size: 180.0,
                                color: WHITE,
                                ..default()
                            }
                        ),
                        MainMenuElements::Text
                    ));
                    }
                );
                //Node for the bottom half of the screen
                parent.spawn(NodeBundle {
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
                })
                    //Node for the play button
                    .with_children(|parent|{
                        parent.spawn((ButtonBundle {
                            style: Style {
                                width: Val::Px(370.0),
                                height: Val::Px(95.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        },
                            ButtonTypes::Play,
                            MainMenuElements::Button,
                        ))
                        //Node for play text
                        .with_children(|parent|{
                            parent.spawn((TextBundle::from_section (
                                "Play",
                                TextStyle {
                                    font_size: 80.0,
                                    color: WHITE,
                                    ..default()
                                }
                            ),
                            MainMenuElements::Text
                            ));
                        });
                    }) 
                    //Node for the quit button
                    .with_children(|parent|{
                        parent.spawn((ButtonBundle {
                            style: Style {
                                width: Val::Px(370.0),
                                height: Val::Px(95.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        },
                            ButtonTypes::Quit,
                            MainMenuElements::Button,
                        ))
                            //Node for quit text
                        .with_children(|parent|{
                            parent.spawn((TextBundle::from_section (
                                "Quit",
                                TextStyle {
                                    font_size: 80.0,
                                    color: WHITE,
                                    ..default()
                                }
                            ),
                            MainMenuElements::Text
                            ));
                        });
                    });
            });
}

pub fn load_background_image(
    asset_server: Res<AssetServer>, 
    mut element_query: Query<(&MainMenuElements, &mut UiImage), Added<MainMenuElements>>)
{
    for (element, mut image) in &mut element_query {
        
        if let MainMenuElements::BackgroundImage = element {                
                *image = asset_server.load("textures/main_menu/HH-background.png").into()
            }
    }
}

pub fn load_button_image(
    asset_server: Res<AssetServer>, 
    mut element_query: Query<(&MainMenuElements, &mut UiImage), Added<MainMenuElements>>)
{
    for (element, mut image) in &mut element_query {
        
        if let MainMenuElements::Button = element {
            *image = asset_server.load("textures/main_menu/button.png").into()
        }
    }
}

pub fn load_text_font(
    asset_server: Res<AssetServer>, 
    mut element_query: Query<(&MainMenuElements, &mut Text), Added<MainMenuElements>>)
{
    for (element, mut text) in &mut element_query {
        
        if let MainMenuElements::Text = element {
            text.sections[0].style.font = asset_server.load("fonts/Huglove.ttf")
        }
    }
}

pub fn unload_main_menu (mut commands: Commands, query: Query<Entity, With<MainMenuUI>>)
{
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
        app.add_startup_system(spawn_main_menu);

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
        app.add_system(unload_main_menu);
        app.update();

        //and ensure that no entities remain
        item_num = app.world.entities().len();
        assert_eq!(0, item_num);
    }
}
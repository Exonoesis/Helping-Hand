use bevy::prelude::*;

const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);

#[derive(Component)]
pub enum MainMenuButtonTypes {
    Play,
    Quit,
}

#[derive(Component)]
pub struct MainMenuUI;

pub fn load_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainMenuUI));
    commands
        //Node that spans entire screen, acts as container for other UI elements
        .spawn((ImageBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0),Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            image: asset_server.load("textures/main_menu/HH-background.png").into(),
            ..default()
        }, 
        MainMenuUI,
        ))
            .with_children(|parent| {
                //Node for the top half of the screen
                parent.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
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
                        parent.spawn(TextBundle::from_section (
                            "Helping Hand",
                            TextStyle {
                                font: asset_server.load("fonts/Huglove.ttf"),
                                font_size: 180.0,
                                color: WHITE,
                            }
                        ));
                    });
                //Node for the bottom half of the screen
                parent.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        gap: Size::height(Val::Percent(4.2)),
                        ..default()
                    },
                    ..default()
                })
                    //Node for the play button
                    .with_children(|parent|{
                        parent.spawn((ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(370.0), Val::Px(95.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            image: asset_server.load("textures/main_menu/button.png").into(),
                            ..default()
                        },
                            MainMenuButtonTypes::Play
                        ))
                            //Node for play text
                            .with_children(|parent|{
                                parent.spawn(TextBundle::from_section (
                                    "Play",
                                    TextStyle {
                                        font: asset_server.load("fonts/Huglove.ttf"),
                                        font_size: 80.0,
                                        color: WHITE,
                                    }
                                ));
                            });
                    }) 
                    //Node for the quit button
                    .with_children(|parent|{
                        parent.spawn((ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(370.0), Val::Px(95.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            image: asset_server.load("textures/main_menu/button.png").into(),
                            ..default()
                        },
                            MainMenuButtonTypes::Quit
                        ))
                            //Node for quit text
                            .with_children(|parent|{
                                parent.spawn(TextBundle::from_section (
                                    "Quit",
                                    TextStyle {
                                        font: asset_server.load("fonts/Huglove.ttf"),
                                        font_size: 80.0,
                                        color: WHITE,
                                    }
                                ));
                            });
                    });
            });
}

pub fn unload_main_menu (mut commands: Commands, query: Query<Entity, With<MainMenuUI>>)
{
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
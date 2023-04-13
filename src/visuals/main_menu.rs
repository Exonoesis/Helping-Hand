use bevy::prelude::*;

const RED: Color = Color::rgb(0.255, 0.0, 0.0);
const BLUE: Color = Color::rgb(0.0, 0.0, 0.255);
const GREEN: Color = Color::rgb(0.0, 0.255, 0.0);
const PINK: Color = Color::rgb(0.255, 0.0, 0.255);
const CYAN: Color = Color::rgb(0.0, 0.255, 0.255);
const BLACK: Color = Color::rgb(0.255, 0.255, 0.255);
const WHITE: Color = Color::rgb(0.0, 0.0, 0.0);

pub fn load_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        //Node that spans entire screen, acts as container for other UI elements
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0),Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
            .with_children(|parent| {
                //Node for the top half of the screen
                parent.spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                        justify_content: JustifyContent::Center,
                        padding: UiRect {
                            left: Val::Px(0.0),
                            right: Val::Px(0.0),
                            top: Val::Px(0.0),
                            bottom: Val::Px(35.0)
                        },
                        ..default()
                    },
                    background_color: RED.into(),
                    ..default()
                })
                    //Node for the title text
                    .with_children(|parent|{
                        parent.spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(945.0), Val::Px(175.0)),
                                align_self: AlignSelf::FlexEnd,
                                ..default()
                            },
                            background_color: BLUE.into(),
                            ..default()
                        });
                    });
                //Node for the bottom half of the screen
                parent.spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        gap: Size::height(Val::Px(30.0)),
                        ..default()
                    },
                    background_color: GREEN.into(),
                    ..default()
                })
                    //Node for the play button
                    .with_children(|parent|{
                        parent.spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(370.0), Val::Px(95.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: PINK.into(),
                            ..default()
                        })
                            //Node for play text
                            .with_children(|parent|{
                                parent.spawn(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                                        ..default()
                                    },
                                    background_color: BLACK.into(),
                                    ..default()
                                });
                            });
                    }) 
                    //Node for the quit button
                    .with_children(|parent|{
                        parent.spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(370.0), Val::Px(95.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: CYAN.into(),
                            ..default()
                        })
                            //Node for quit text
                            .with_children(|parent|{
                                parent.spawn(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(80.0), Val::Percent(80.0)),
                                        ..default()
                                    },
                                    background_color: WHITE.into(),
                                    ..default()
                                });
                            });
                    });
            });
}
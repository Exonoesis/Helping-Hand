use bevy::prelude::*;
use bevy::app::AppExit;

use crate::visuals::main_menu::MainMenuButtonTypes;

pub fn button_system(
    mut exit_event: EventWriter<AppExit>,
    mut interaction_query: Query<(&Interaction, &MainMenuButtonTypes), (Changed<Interaction>, With<Button>), 
    >
) {
    for (interaction, button_type) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                match button_type {
                    MainMenuButtonTypes::Play => println!("Play clicked!"),
                    MainMenuButtonTypes::Quit => exit_event.send(AppExit)
                }
            }
            Interaction::Hovered => {
                println! ("Hover");
            }
            Interaction::None => {
            }
        }
    }
}
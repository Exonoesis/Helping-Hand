use bevy::prelude::*;
use bevy::app::AppExit;

use crate::AppState;
use crate::visuals::main_menu::ButtonTypes;

pub fn button_system(
    mut exit_event: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<(&Interaction, &ButtonTypes), (Changed<Interaction>, With<Button>)>
) {
    for (interaction, button_type) in &mut interaction_query {
        if *interaction != Interaction::Clicked {
            continue;
        }
        
        match button_type {
            ButtonTypes::Play => next_state.set(AppState::InGame),
            ButtonTypes::Quit => exit_event.send(AppExit),
        }
    }
}
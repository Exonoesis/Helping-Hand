use bevy::app::AppExit;
use bevy::prelude::*;

use crate::visuals::main_menu::ButtonTypes;
use crate::AppState;

pub fn button_system(
    mut exit_event: EventWriter<AppExit>,
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
            ButtonTypes::Play => next_state.set(AppState::InGame),
            ButtonTypes::Settings => next_state.set(AppState::SettingsMenu),
            ButtonTypes::Quit => {
                exit_event.send(AppExit);
            }
        }
    }
}

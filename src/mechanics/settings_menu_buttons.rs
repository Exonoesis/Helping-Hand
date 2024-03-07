use bevy::prelude::*;

use crate::visuals::settings_menu::{
    ButtonTypes, FillReference, SettingsMenuElements, ValueReference,
};
use crate::AppState;

use super::custom_widgets::CountingSliderKeys;

pub fn spinner_buttons_system(
    mut spinner_button_query: Query<
        (&Interaction, &ButtonTypes, &ValueReference, &FillReference),
        (Changed<Interaction>, With<Button>),
    >,
    mut spinner_value_query: Query<
        &mut Text,
        (With<SettingsMenuElements>, With<CountingSliderKeys>),
    >,
    mut spinner_fill_query: Query<&mut Style, With<CountingSliderKeys>>,
) {
    for (interaction, button_type, value_reference, fill_reference) in &mut spinner_button_query {
        if *interaction != Interaction::Pressed
            || *button_type != ButtonTypes::Increment && *button_type != ButtonTypes::Decrement
        {
            continue;
        }

        let mut spinner_value = spinner_value_query
            .get_mut(value_reference.0)
            .expect("spinner_buttons_system: Spinner value should exist.");

        let old_value = spinner_value.sections[0].value.parse::<u32>().unwrap();

        if *button_type == ButtonTypes::Increment && old_value == 100
            || *button_type == ButtonTypes::Decrement && old_value == 0
        {
            continue;
        }

        let mut new_value = old_value;

        if *button_type == ButtonTypes::Increment {
            new_value = old_value + 1;
        } else if *button_type == ButtonTypes::Decrement {
            new_value = old_value - 1;
        }

        spinner_value.sections[0].value = new_value.to_string();

        let mut fill_value = spinner_fill_query
            .get_mut(fill_reference.0)
            .expect("spinner_buttons_system: Spinner fill value should exist.");

        let new_fill_amount = spinner_value.sections[0].value.parse::<f32>().unwrap() - 1.5;

        fill_value.width = Val::Percent(new_fill_amount);
    }
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

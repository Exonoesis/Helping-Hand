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

        //We subtract half the handle width from the fill amount so that the value is accurate
        //to the center of the handle rather than the edge of the fill bar
        /*
        ui_container = 100% = 1920 (default)
        middle_third = 66% = 126.72
        options_container = 100% = 126.72
        widget_container = 96% = 121.632
        music_slider_container = 60% = 72.9792
        music_slider_back = 100% = 72.9792
        music_slider_handle = 5% = 3.6485
        1/2 of 3.6485 = 1.82448 (rounded to 1.8)
        */
        let new_fill_amount = spinner_value.sections[0].value.parse::<f32>().unwrap() - 1.8;

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

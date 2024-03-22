use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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

        /*
        We calculate the pixel value of half the handle's width to obtain the percentage
        of the fill bar that needs to be subtracted such that the value is visually accurate
        to the center of the handle, as is expected, rather than the edge of the fill bar

        ui_container = 100% = 1280 (default)
        middle_third = 66% = 844.8
        options_container = 100% = 844.8
        widget_container = 96% = 811.008
        music_slider_container = 60% = 486.6048
        music_slider_back = 100% = 486.6048
        music_slider_handle = 5% = 24.33024
        1/2 of slider_handle = 50% = 12.16512

        12.16512 is 2.5% of 486.605
        */
        let new_fill_amount = spinner_value.sections[0].value.parse::<f32>().unwrap() - 2.5;

        fill_value.width = Val::Percent(new_fill_amount);
    }
}

pub fn slider_handle_system(
    mut slider_handle_query: Query<
        (&Interaction, &ButtonTypes, &ValueReference, &FillReference),
        (Changed<Interaction>, With<Button>),
    >,
    mut slider_value_query: Query<
        &mut Text,
        (With<SettingsMenuElements>, With<CountingSliderKeys>),
    >,
    mut slider_fill_query: Query<&mut Style, With<CountingSliderKeys>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_button_state: Res<Input<MouseButton>>,
) {
    //Capture initial click of handle
    for (interaction, button_type, value_reference, fill_reference) in &mut slider_handle_query {
        if *interaction != Interaction::Pressed || *button_type != ButtonTypes::Slider {
            continue;
        }

        //Capture mouse.x position at time of click
        if let Some(position) = window_query.single().cursor_position() {
            //Capture position.x as mouse original_x_position
        } else {
            //It shouldn't be possible to click the handle while mouse is outside game window
        }
    }

    //Detect if left mouse button is being held down
    for button in mouse_button_state.get_pressed() {
        if button != &MouseButton::Left {
            continue;
        }

        //Get current mouse.x for percent of change calculation
        if let Some(position) = window_query.single().cursor_position() {
            //Get current mouse position.x
            //Access original mouse.x???
        } else {
            //Doesn't matter if mouse goes outside game window (values beyond slider will be floored)
        }
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

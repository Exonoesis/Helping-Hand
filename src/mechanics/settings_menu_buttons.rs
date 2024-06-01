use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::visuals::settings_menu::{
    BackReference, ButtonTypes, FillReference, HandleReference, SettingsMenuElements,
    ValueReference,
};
use crate::AppState;

use super::custom_widgets::CountingSliderKeys;

#[derive(Component, Clone, Copy)]
pub struct BeingClicked {
    original_x: f32,
    original_val: f32,
    handle_width: Val,
}

pub fn spinner_buttons_system(
    mut spinner_button_query: Query<
        (
            &Interaction,
            &ButtonTypes,
            &ValueReference,
            &FillReference,
            &HandleReference,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut spinner_value_query: Query<
        &mut Text,
        (With<SettingsMenuElements>, With<CountingSliderKeys>),
    >,
    mut spinner_fill_query: Query<&mut Style, (With<CountingSliderKeys>, Without<Button>)>,
    spinner_handle_query: Query<&Style, (With<CountingSliderKeys>, With<Button>)>,
) {
    for (interaction, button_type, value_reference, fill_reference, handle_reference) in
        &mut spinner_button_query
    {
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
        We subtract half the handles width from the fill bar so that the values
        position is visually accurate to the center of the handle, as is expected,
        rather than the right edge of the fill bar
        */
        let handle_style = spinner_handle_query
            .get(handle_reference.0)
            .expect("spinner_buttons_system: Spinner handle should exist.");

        let handle_width_percentage = if let Val::Percent(width_percentage) = handle_style.width {
            width_percentage
        } else {
            panic!("spinner_buttons_system: Handle width should be a percentage.")
        };

        let new_fill_amount = spinner_value.sections[0].value.parse::<f32>().unwrap()
            - (handle_width_percentage / 2.0);

        fill_value.width = Val::Percent(new_fill_amount);
    }
}

pub fn get_handle_click_position(
    mut slider_handle_query: Query<
        (
            &Interaction,
            &ButtonTypes,
            &ValueReference,
            Entity,
            &mut Style,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut slider_value_query: Query<
        &mut Text,
        (With<SettingsMenuElements>, With<CountingSliderKeys>),
    >,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    //Capture handle click
    for (interaction, button_type, value_reference, handle, style) in &mut slider_handle_query {
        if *interaction != Interaction::Pressed || *button_type != ButtonTypes::Slider {
            commands.entity(handle).remove::<BeingClicked>();
            continue;
        }

        //Capture mouse.x position at time of click
        let original_x_position = if let Some(position) = window_query.single().cursor_position() {
            position.x
        } else {
            panic!("slider_handle_system: Handle was clicked while mouse was outside game window")
        };

        //Capture original spinner value at time of click
        let spinner_value = slider_value_query
            .get_mut(value_reference.0)
            .expect("get_handle_click_position: Spinner value should exist.");

        let original_spinner_value = spinner_value.sections[0].value.parse::<f32>().unwrap();

        //Attach original_mouse_x to Handle as component
        let original_mouse_x_reference = BeingClicked {
            original_x: original_x_position,
            original_val: original_spinner_value,
            handle_width: style.width,
        };

        commands.entity(handle).insert(original_mouse_x_reference);
    }
}

pub fn update_handle_position_on_hold(
    slider_handle_query: Query<
        (
            &BeingClicked,
            &FillReference,
            &ValueReference,
            &BackReference,
        ),
        (With<BeingClicked>, With<Button>),
    >,
    mut slider_value_query: Query<
        &mut Text,
        (With<SettingsMenuElements>, With<CountingSliderKeys>),
    >,
    mut width_query: Query<&mut Style>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<&Camera>,
    parent_query: Query<&Parent>,
    mouse_button_state: Res<Input<MouseButton>>,
) {
    //Detect if left mouse button is being held down
    if !mouse_button_state.pressed(MouseButton::Left) {
        return;
    }

    //Get current mouse.x
    let current_x_position = if let Some(position) = window_query.single().cursor_position() {
        position.x
    } else {
        //Doesn't matter if mouse goes outside game window (values beyond slider extremes will be capped)
        return;
    };

    //Get original mouse.x + spinner and fill references
    if slider_handle_query.is_empty() {
        return;
    }

    let handle_data_references = if let Ok(data) = slider_handle_query.get_single() {
        data
    } else {
        panic!("update_handle_position_on_hold: Expected single cursor")
    };

    let original_values = handle_data_references.0;
    let fill_reference = handle_data_references.1;
    let spinner_value_reference = handle_data_references.2;
    let back_reference = handle_data_references.3;

    let read_only_width_query = width_query.to_readonly();

    //Calculate change in mouse x movement as a percent
    let back_width_in_px = get_node_width(
        back_reference.0,
        read_only_width_query,
        camera_query,
        parent_query,
    );

    let change_as_percent =
        (((current_x_position - original_values.original_x) / back_width_in_px) * 100.00).trunc();

    //Change spinner value
    let mut spinner_value = slider_value_query
        .get_mut(spinner_value_reference.0)
        .expect("update_handle_position_on_hold: Spinner value should exist.");

    let mut new_spinner_value = original_values.original_val + change_as_percent;

    if new_spinner_value > 100.00 {
        new_spinner_value = 100.00
    } else if new_spinner_value < 0.00 {
        new_spinner_value = 0.00
    }

    spinner_value.sections[0].value = new_spinner_value.to_string();

    //Change fill bar width
    let mut fill_value = width_query
        .get_mut(fill_reference.0)
        .expect("update_handle_position_on_hold: Slider fill value should exist.");

    /*
    We subtract half the handles width from the fill bar so that the values
    position is visually accurate to the center of the handle, as is expected,
    rather than the right edge of the fill bar
    */
    let handle_width_percentage =
        if let Val::Percent(width_percentage) = original_values.handle_width {
            width_percentage
        } else {
            panic!("spinner_buttons_system: Handle width should be a percentage.")
        };

    let new_fill_amount =
        spinner_value.sections[0].value.parse::<f32>().unwrap() - (handle_width_percentage / 2.0);

    fill_value.width = Val::Percent(new_fill_amount);
}

fn get_node_width(
    node: Entity,
    width_query: Query<&Style>,
    camera_query: Query<&Camera>,
    parent_query: Query<&Parent>,
) -> f32 {
    let mut value_stack = get_all_ancestors(node, parent_query, width_query);

    let camera = camera_query.single();

    let viewport_size = camera.logical_viewport_size().unwrap();
    let mut node_width = viewport_size.x;

    while let Some(val) = value_stack.pop() {
        //node_width = val.resolve(node_width, viewport_size);
        node_width = val
            .evaluate(node_width)
            .expect("get_node_width: Node width could not be resolved");
    }

    return node_width;
}

fn get_all_ancestors(
    node: Entity,
    parent_query: Query<&Parent>,
    width_query: Query<&Style>,
) -> Vec<bevy::ui::Val> {
    let mut to_be_visited_nodes = Vec::new();
    let mut seen_styles = Vec::new();

    //Build stack
    to_be_visited_nodes.push(node);

    while let Some(current_node) = to_be_visited_nodes.pop() {
        let current_node_value = width_query
            .get(current_node)
            .expect("get_all_ancestors: Could not get Style for node")
            .width;

        seen_styles.push(current_node_value);

        if let Ok(parent) = parent_query.get(current_node) {
            to_be_visited_nodes.push(parent.get());
        }
    }

    return seen_styles;
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

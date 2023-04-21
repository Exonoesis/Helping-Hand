use bevy::prelude::*;

pub fn button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>), 
    >
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                println! ("Click!");
            }
            Interaction::Hovered => {
                println! ("Hover");
            }
            Interaction::None => {
            }
        }
    }
}
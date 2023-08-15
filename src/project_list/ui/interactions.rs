use bevy::prelude::*;
use bevy::app::AppExit;

use crate::project_list::components::*;
use crate::project_list::styles::*;

pub fn interact_with_project_1_button(
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
        ),
        (Changed<Interaction>, With<Project1Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Project 1 soon to be added".to_string();
                *color = HOVERED_PRESSED_BUTTON_COLOR.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Click to view project 1".to_string();
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                text.sections[0].value = "Project 1".to_string();
                *color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_project_2_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor,Option<&SelectedOption>),(Changed<Interaction>, With<Project2Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut button_query {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON_COLOR.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON_COLOR.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON_COLOR.into(),
            (Interaction::None, None) => NORMAL_BUTTON_COLOR.into(),
        }
    }
}


pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<(&Interaction, &mut BackgroundColor),(Changed<Interaction>, With<QuitButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
        }
    }
}
}

use bevy::prelude::*;
use bevy::app::AppExit;

use crate::AppState;

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
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color, children) in &mut button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *color = HOVERED_PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(AppState::TaskScreen);
            }
            Interaction::Hovered => {
                text.sections[0].value = "Click to view Project 1".to_string();
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
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
        ),
        (Changed<Interaction>, With<Project2Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Project 2 soon to be added".to_string();
                *color = HOVERED_PRESSED_BUTTON_COLOR.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Click to view Project 2".to_string();
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                text.sections[0].value = "Project 2".to_string();
                *color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_project_3_button(
    mut button_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &Children,
        ),
        (Changed<Interaction>, With<Project3Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut button_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Project 3 soon to be added".to_string();
                *color = HOVERED_PRESSED_BUTTON_COLOR.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Click to complete".to_string();
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                text.sections[0].value = "Project 3".to_string();
                *color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn button_interaction(
    mut revealed_query: Query<&mut Revealed>,
    mut button_query: Query<(&mut Style, &Project3Button, &Interaction)>,
) {
    for (mut style, _, interaction) in button_query.iter_mut() {
        if interaction == &Interaction::Clicked {
            if let Some(mut visible) = revealed_query.iter_mut().next() {
                if !visible.0 {
                    visible.0 = true;

                    style.display = Display::Flex;
                } else {
                    style.display = Display::None;
                }
            }
        }
    }
}

pub fn interact_with_graph_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<GraphButton>)
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(AppState::Graph);
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


pub fn interact_with_main_menu_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>)
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(AppState::MainMenu);
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

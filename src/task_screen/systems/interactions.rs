use bevy::prelude::*;
use bevy::app::AppExit;

use crate::AppState;
use crate::task_screen::components::*;
use crate::task_screen::styles::*;

pub fn interact_with_task_1_completed_button(mut button_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &Children,
    ),
    (Changed<Interaction>, With<Task1CompletedButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    ) {
    for (interaction, mut color, children) in &mut button_query {
    let mut text = text_query.get_mut(children[0]).unwrap();
    match *interaction {
        Interaction::Clicked => {
            *color = HOVERED_PRESSED_BUTTON_COLOR.into();
            app_state_next_state.set(AppState::ProjectList);
        }
        Interaction::Hovered => {
            text.sections[0].value = "Click here if task is Completed".to_string();
            *color = HOVERED_BUTTON_COLOR.into();
        }
        Interaction::None => {
            text.sections[0].value = "Task Completed".to_string();
            *color = NORMAL_BUTTON_COLOR.into();
        }
    }
    }
    }

pub fn interact_with_project_list_button(mut button_query: Query<
    (
        &Interaction,
        &mut BackgroundColor,
        &Children,
    ),
    (Changed<Interaction>, With<ProjectListButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    ) {
    for (interaction, mut color, children) in &mut button_query {
    let mut text = text_query.get_mut(children[0]).unwrap();
    match *interaction {
        Interaction::Clicked => {
            *color = HOVERED_PRESSED_BUTTON_COLOR.into();
            app_state_next_state.set(AppState::ProjectList);
        }
        Interaction::Hovered => {
            text.sections[0].value = "Back to Project List".to_string();
            *color = HOVERED_BUTTON_COLOR.into();
        }
        Interaction::None => {
            text.sections[0].value = "Project List".to_string();
            *color = NORMAL_BUTTON_COLOR.into();
        }
    }
    }
    }
    


pub fn interact_with_main_menu_button (mut button_query: Query<
(
    &Interaction,
    &mut BackgroundColor,
    &Children,
),
(Changed<Interaction>, With<MainMenuButton>),
>,
mut text_query: Query<&mut Text>,
mut app_state_next_state: ResMut<NextState<AppState>>,
) {
for (interaction, mut color, children) in &mut button_query {
let mut text = text_query.get_mut(children[0]).unwrap();
match *interaction {
    Interaction::Clicked => {
        *color = HOVERED_PRESSED_BUTTON_COLOR.into();
        app_state_next_state.set(AppState::MainMenu);
    }
    Interaction::Hovered => {
        text.sections[0].value = "Back to Main Menu".to_string();
        *color = HOVERED_BUTTON_COLOR.into();
    }
    Interaction::None => {
        text.sections[0].value = "Main Menu".to_string();
        *color = NORMAL_BUTTON_COLOR.into();
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

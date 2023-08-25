mod components;
mod styles;
mod ui;

use bevy::prelude::*;

use crate::AppState;
use crate::project_list::ui::layout::*;
use ui::interactions::*;

pub struct ProjectListPlugin; 

impl Plugin for ProjectListPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_project_list.in_schedule(OnEnter(AppState::ProjectList)))
          .add_systems(
             (
               interact_with_project_1_button,
               interact_with_project_2_button,
               interact_with_project_3_button,
               interact_with_project_4_button,
               button_interaction,
               interact_with_main_menu_button,
               interact_with_quit_button,
             )
             .in_set(OnUpdate(AppState::ProjectList))
          )
          .add_system(despawn_project_list.in_schedule(OnExit(AppState::ProjectList)));
    } 
}
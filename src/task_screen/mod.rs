mod components;
mod styles;
mod systems;

use bevy::prelude::*;
use crate::AppState;
use crate::task_screen::systems::layout::*;
use systems::interactions::*;

pub struct TaskScreenPlugin; 
impl Plugin for TaskScreenPlugin {
   fn build(&self, app: &mut App) {
       app.add_system(spawn_task_screen.in_schedule(OnEnter(AppState::TaskScreen)))
       .add_systems(
        (
            interact_with_project_list_button,
            interact_with_task_1_completed_button,
            interact_with_task_1_button,
            button_interaction,
            interact_with_main_menu_button,
            interact_with_quit_button,
        )
        .in_set(OnUpdate(AppState::TaskScreen))
     )
         .add_system(despawn_task_screen.in_schedule(OnExit(AppState::TaskScreen)));
   } 
}
mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::AppState;
use crate::main_menu::systems::layout::*;
use systems::interactions::*;

pub struct MainMenuPlugin; 

impl Plugin for MainMenuPlugin {
   fn build(&self, app: &mut App) {
       app.add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
         .add_systems(
            (
               interact_with_project_list_button,
               interact_with_visibility_test_button,
               interact_with_quit_button
            )
            .in_set(OnUpdate(AppState::MainMenu))
         )
         .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
   } 
}
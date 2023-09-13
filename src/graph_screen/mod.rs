mod components;
mod interactions;
mod styles;
mod systems;

use bevy::prelude::*;
use crate::AppState;
use crate::graph_screen::systems::*;
use interactions::*;

pub struct GraphPlugin;

impl Plugin for GraphPlugin {
    fn build(&self, app: &mut App) {
    app.add_system(spawn_graph.in_schedule(OnEnter(AppState::Graph)))
    .add_systems(
      (
        interact_with_main_menu_button,
        interact_with_project_list_button,
        interact_with_quit_button,
      )
      .in_set(OnUpdate(AppState::Graph))
   )
      .add_system(despawn_graph.in_schedule(OnExit(AppState::Graph)));
} 
}
mod systems;

use bevy::prelude::*;
use crate::graph_screen::graph_setup::systems::*;

use crate::AppState;

pub struct GraphMainPlugin;

impl Plugin for GraphMainPlugin {
    fn build(&self, app: &mut App) {
    app.add_system(spawn_graph_main.in_schedule(OnEnter(AppState::Graph)))
      .add_system(despawn_graph_main.in_schedule(OnExit(AppState::Graph)));
} 
}
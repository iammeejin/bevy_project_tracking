mod ui;
mod graph_setup;

use bevy::prelude::*;

use ui::GraphUIPlugin;
use graph_setup::GraphMainPlugin; 


pub struct GraphPlugin;

impl Plugin for GraphPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(GraphUIPlugin)
        .add_plugin(GraphMainPlugin);
    }
}
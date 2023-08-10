use bevy::prelude::*;

use systems::*;

pub mod main_menu;
use main_menu::MainMenuPlugin;
mod systems;

fn main() {
    App::new()
    //Bevy Plugins
    .add_plugins(DefaultPlugins)
    .add_state::<AppState>()
    //My Plugins
    .add_plugin(MainMenuPlugin)
    //Startup System
    .add_startup_system(spawn_camera)
    //Systems
    .add_system(transition_to_main_menu_state)
    .add_system(exit_menu)
    .run();
  }

  #[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
}
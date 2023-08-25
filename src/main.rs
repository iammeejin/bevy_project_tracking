use bevy::prelude::*;


use systems::*;

mod main_menu;
mod project_list;
mod task_screen;

use main_menu::MainMenuPlugin;
use project_list::ProjectListPlugin;
use task_screen::TaskScreenPlugin;
mod systems;

fn main() {
    App::new()
    //Bevy Plugins
    .add_plugins(DefaultPlugins)
    .add_state::<AppState>()
    //My Plugins
    .add_plugin(MainMenuPlugin)
    .add_plugin(ProjectListPlugin)
    .add_plugin(TaskScreenPlugin)
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
    ProjectList,
    TaskScreen,
}
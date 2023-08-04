use bevy::prelude::*;

mod main_menu;
use main_menu::MainMenuPlugin;


fn main() {
    App::new()
    //Bevy Plugins
    .add_plugins(DefaultPlugins)
    .add_state::<AppState>()
    //My Plugins
    .add_plugin(MainMenuPlugin)
    .run();
  }

  #[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
}
use bevy::prelude::Component;
use bevy::prelude::Resource;

#[derive(Component)]
pub struct GraphScreen{}


#[derive(Component)]
pub struct MainMenuButton{}

#[derive(Component)]
pub struct ProjectListButton{}

#[derive(Component)]
pub struct QuitButton{}

#[derive(Component)]
pub struct Node {}

#[derive(Component)]
pub struct Edge {}

#[derive(Component)]
pub struct Selected {}

#[derive(Resource)]
struct SelectedNode{}
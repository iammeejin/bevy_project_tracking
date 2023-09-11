use bevy::prelude::Component;

#[derive(Component)]
pub struct TaskScreen{}

#[derive(Component)]
pub struct MainMenuButton{}

#[derive(Component)]
pub struct ProjectListButton{}

#[derive(Component)]
pub struct QuitButton{}

#[derive(Component)]
pub struct Task1Button{}

#[derive(Component)]
pub struct Task2Button{}


#[derive(Component)]
pub struct Task1CompletedButton{}

#[derive(Component)]
pub struct DescriptionScreen{}

#[derive(Component)]
pub struct DescriptionButton{}

#[derive(Component)]
pub struct Visible(
    pub bool,
);
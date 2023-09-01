use bevy::prelude::*;
use crate::task_screen::components::*;
use crate::task_screen::styles::*;


pub fn spawn_task_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    build_task_screen(&mut commands, &asset_server);
}


pub fn despawn_task_screen(
    mut commands: Commands,
    task_screen_query: Query<Entity, With<TaskScreen>>
) {
    if let Ok(task_screen_entity) = task_screen_query.get_single() {
        commands.entity(task_screen_entity).despawn_recursive();
    }
}

pub fn build_task_screen(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
)
    -> Entity {
        let task_screen_entity = commands.spawn(
            (
                NodeBundle {
                style: TASK_SCREEN_STYLE,
                background_color: Color::GRAY.into(),
                ..default()
            },
            TaskScreen {},
        ))

        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn(TextBundle::from_section(
                                    "Task List",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(5.)),
                                    ..default()
                                }
                            ));
                        });
                });
            // right vertical fill
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle::from_section(
                            "Task Description Here",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 25.,
                                color: Color::WHITE,
                            },
                        )
                    );
                });
           
                
        })

        .id();
        task_screen_entity
    }
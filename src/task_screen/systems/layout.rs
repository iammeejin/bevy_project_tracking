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

        .with_children(|parent|{
            // Header
            parent
                .spawn(NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(6.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle {
                            text: Text {
                                sections: vec!(
                                    TextSection::new(
                                        "Task Screen",
                                        get_button_text_style(&asset_server),
                    )),
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        }
                    );
                });
                    
            
            // Mark as Completed Button
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    Task1CompletedButton {},
                )
            )
            .with_children(|parent|{
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec!(
                                TextSection::new(
                                    "Task Completed",
                                    get_button_text_style(&asset_server),
                )),
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            });


            // Project List Button
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    ProjectListButton {},
                )
            )
            .with_children(|parent|{
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec!(
                                TextSection::new(
                                    "Project List",
                                    get_button_text_style(&asset_server),
                )),
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            });

             //Main Menu Button
             parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    MainMenuButton {}
                ))
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Main Menu",
                                    get_button_text_style(&asset_server)
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        }
                    );
                });

            // Quit Button
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    QuitButton {},
                )
            )
            .with_children(|parent|{
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec!(
                                TextSection::new(
                                    "Quit",
                                    get_button_text_style(&asset_server),
                                )
                            ),
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            });

        })

        .id();
        task_screen_entity
    }
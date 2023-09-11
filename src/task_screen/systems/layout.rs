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
                    style: LEFT_VERTICAL_BORDER_STYLE,
                    background_color: Color::BLACK.into(),
                    ..default()
                })
                .with_children(|parent| {

                     // Task 1 Button
                    parent.spawn(
                        (
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            Task1Button {},
                            )
                            )
                                .with_children(|parent|{
                                parent.spawn(
                            TextBundle {
                                text: Text {
                                    sections: vec!(
                                        TextSection::new(
                                            "Task 1",
                                            get_button_text_style(&asset_server),
                                )),
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                                }
                                );
                                });

                    // Task 2 Button
                    parent.spawn(
                        (
                            ButtonBundle {
                                style: BUTTON_STYLE,
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            Task2Button {},
                            )
                            )
                                .with_children(|parent|{
                                parent.spawn(
                            TextBundle {
                                text: Text {
                                    sections: vec!(
                                        TextSection::new(
                                            "Task 2",
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
                (ButtonBundle {
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
                });
            
            // right vertical fill
            parent
                .spawn((NodeBundle {
                    style: RIGHT_VERTICAL_FILL_STYLE,
                    background_color: Color::BLACK.into(),
                    ..default()
                },
                DescriptionScreen {},
                )
                )
                .with_children(|parent| {
                    // Title
                    parent.spawn(
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Task Description",
                                    get_button_text_style(&asset_server)
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },

                            ..default()
                        }
                    );

                    // Description Button
                    parent.spawn(
                        (ButtonBundle {
                            style: BUTTON_STYLE1,
                            background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                                },
                                DescriptionButton {},
                                )
                                )
                                .with_children(|parent|{
                                parent.spawn(
                            TextBundle {
                                text: Text {
                                    sections: vec!(
                                        TextSection::new(
                                            "Description Here",
                                            get_button_text_style(&asset_server),
                                        )
                                    ),
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                                }
                                );

                     // Task1 Completed Button
                     parent.spawn(
                        (ButtonBundle {
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
                                            "Complete Task",
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
                            });
                });


        })
        .id();
        task_screen_entity
    }
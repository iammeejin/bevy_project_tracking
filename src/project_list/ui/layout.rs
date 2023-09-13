use bevy::prelude::*;
use crate::project_list::components::*;
use crate::project_list::styles::*;


pub fn spawn_project_list(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    build_project_list(&mut commands, &asset_server);
}


pub fn despawn_project_list(
    mut commands: Commands,
    project_list_query: Query<Entity, With<ProjectList>>
) {
    if let Ok(project_list_entity) = project_list_query.get_single() {
        commands.entity(project_list_entity).despawn_recursive();
    }
}

pub fn build_project_list(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
)
    -> Entity {
        let project_list_entity = commands.spawn(
            (
                NodeBundle {
                style: PROJECT_LIST_STYLE,
                background_color: Color::CRIMSON.into(),
                ..default()
            },
            ProjectList {},
        ))

        .with_children(|parent|{
            // Project 1 Button
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    Project1Button {},
                )
            )
            .with_children(|parent|{
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec!(
                                TextSection::new(
                                    "Project 1",
                                    get_button_text_style(&asset_server),
                )),
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            });

            // Project 2 Button
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    Project2Button {},
                    Revealed(true),
                )
            )
            .with_children(|parent|{
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec!(
                                TextSection::new(
                                    "Project 2",
                                    get_button_text_style(&asset_server),
                )),
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            });

            // Project 3 Button
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    Project3Button {},
                    Revealed(true),
                )
            )
            .with_children(|parent|{
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec!(
                                TextSection::new(
                                    "Project 3",
                                    get_button_text_style(&asset_server),
                )),
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            });


            // Graph Button
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    GraphButton {},
                )
            )
            .with_children(|parent|{
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec!(
                                TextSection::new(
                                    "Graph",
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
        project_list_entity
    }



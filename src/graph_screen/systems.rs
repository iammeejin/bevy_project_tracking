use bevy::prelude::*;
use crate::graph_screen::components::*;
use crate::graph_screen::styles::*;


pub fn spawn_graph(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    build_graph_screen(&mut commands, &asset_server);
}


pub fn despawn_graph(
    mut commands: Commands,
    graph_screen_query: Query<Entity, With<GraphScreen>>
) {
    if let Ok(graph_screen_entity) = graph_screen_query.get_single() {
        commands.entity(graph_screen_entity).despawn_recursive();
    }
}

pub fn build_graph_screen(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
)
    -> Entity {
        let graph_screen_entity = commands.spawn(
            (
                NodeBundle {
                style: GRAPH_SCREEN_STYLE,
                background_color: Color::GRAY.into(),
                ..default()
            },
            GraphScreen {},
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
                .spawn(NodeBundle {
                    style: RIGHT_VERTICAL_FILL_STYLE,
                    background_color: Color::BLACK.into(),
                    ..default()
                }
                )
                .with_children(|parent| {
                    // Title
                    parent.spawn(
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Graph",
                                    get_button_text_style(&asset_server)
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },

                            ..default()
                        }
                    );
                });


        })
        .id();
    graph_screen_entity
    }
use bevy::prelude::*;
use crate::main_menu::components::*;
use crate::main_menu::styles::*;


pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    build_main_menu(&mut commands, &asset_server);
}


pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
)
    -> Entity {
        let main_menu_entity = commands.spawn(
            (
                NodeBundle {
                style: MAIN_MENU_STYLE,
                background_color: Color::WHITE.into(),
                ..default()
            },
            MainMenu {},
        ))

        .with_children(|parent|{
            // Title
            parent.spawn(
                NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                }
            )
            .with_children(|parent|{
                // Text
                parent.spawn(
                    TextBundle {
                        text: Text {
                            sections: vec!(
                                TextSection::new(
                                    "Project Tracking",
                                    get_title_text_style(&asset_server),
                                )
                            ),
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    }
                );
            });

            // Main Menu Button
            parent.spawn(
                (
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    MainMenuButton {},
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
        main_menu_entity
    }
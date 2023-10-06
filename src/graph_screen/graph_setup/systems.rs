use bevy::prelude::*;


#[derive(Component)]
pub struct GraphMain{}

#[derive(Component)]
pub struct Node {}

#[derive(Component)]
pub struct Edge {}



pub const RIGHT_VERTICAL_FILL_STYLE: Style = Style {
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(70.0), Val::Percent(100.0)),
    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
    ..Style::DEFAULT
};

pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
}}


pub fn spawn_graph_main(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    build_graph(&mut commands, &asset_server);
}


pub fn despawn_graph_main(
    mut commands: Commands,
    graph_main_query: Query<Entity, With<GraphMain>>
) {
    if let Ok(graph_main_entity) = graph_main_query.get_single() {
        commands.entity(graph_main_entity).despawn_recursive();
    }
}

pub fn build_graph(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>
)
    -> Entity {
        let graph_main_entity = commands.spawn(
            (
                NodeBundle {
                style: RIGHT_VERTICAL_FILL_STYLE,
                background_color: Color::GRAY.into(),
                ..default()
            },
            GraphMain {},
        ))
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
        })
        .id();
    graph_main_entity
    }

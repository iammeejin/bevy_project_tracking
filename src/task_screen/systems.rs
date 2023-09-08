use bevy::prelude::*;

use crate::task_screen::styles::*;


#[derive(Component)]

pub struct TaskScreen{}

#[derive(Component)]
pub struct Target<T> {
    id: Entity,
    phantom: std::marker::PhantomData<T>,
}

impl<T> Target<T> {
    pub fn new(id: Entity) -> Self {
        Self {
            id,
            phantom: std::marker::PhantomData,
        }
    }
}

pub trait TargetUpdate {
    type TargetComponent: Component;
    const NAME: &'static str;
    fn update_target(&self, target: &mut Self::TargetComponent) -> String;
}

impl TargetUpdate for Target<Display> {
    type TargetComponent = Style;
    const NAME: &'static str = "Display";
    fn update_target(&self, style: &mut Self::TargetComponent) -> String {
        style.display = match style.display {
            Display::Flex => Display::None,
            Display::None => Display::Flex,
        };
        format!("{}::{:?} ", Self::NAME, style.display)
    }
}

impl TargetUpdate for Target<Visibility> {
    type TargetComponent = Visibility;
    const NAME: &'static str = "Visibility";
    fn update_target(&self, visibility: &mut Self::TargetComponent) -> String {
        *visibility = match *visibility {
            Visibility::Inherited => Visibility::Visible,
            Visibility::Visible => Visibility::Hidden,
            Visibility::Hidden => Visibility::Inherited,
        };
        format!("{}::{visibility:?}", Self::NAME)
    }
}


pub fn spawn_task_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let palette = PALETTE.map(|hex| Color::hex(hex).unwrap());
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 24.0,
        color: Color::WHITE,
    };

    commands.spawn(NodeBundle {
        style: TASK_SCREEN_STYLE,
        background_color: BackgroundColor(Color::BLACK),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(
            TextBundle {
                text: Text {
                    sections: vec!(
                        TextSection::new(
                            "Name of Project 1",
                            get_button_text_style(&asset_server),
                )),
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
                }
                );

        parent
            .spawn(NodeBundle {
                style: TASK_SCREEN_STYLE1,
                ..Default::default()
            })
            .with_children(|parent| {
                let mut target_ids = vec![];
                parent.spawn(NodeBundle {
                    style: LEFT_PANEL_STYLE,
                    ..Default::default()
                }).with_children(|parent| {
                    target_ids = spawn_left_panel(parent, &palette);
                });

                parent.spawn(NodeBundle {
                    style: RIGHT_PANEL_STYLE,
                    ..Default::default()
                }).with_children(|parent| {
                    spawn_right_panel(parent, text_style, &palette, target_ids);
                });
            });
    });
}

pub fn spawn_left_panel(builder: &mut ChildBuilder, palette: &[Color; 4]) -> Vec<Entity> {
    let mut target_ids = vec![];
    builder
        .spawn(NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(10.)),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    ..Default::default()
                })
                .with_children(|parent| {
                    let id = parent
                        .spawn((NodeBundle {
                            style: Style {
                                align_items: AlignItems::FlexEnd,
                                justify_content: JustifyContent::FlexEnd,
                                ..Default::default()
                            },
                            background_color: BackgroundColor(palette[0]),
                            ..Default::default()
                        },))
                        .with_children(|parent| {
                            parent.spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(500.0)),
                                    ..Default::default()
                                },
                                ..Default::default()
                            });

                            let id = parent
                                .spawn((NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(100.0), Val::Px(400.0)),
                                        align_items: AlignItems::FlexEnd,
                                        justify_content: JustifyContent::FlexEnd,
                                        ..Default::default()
                                    },
                                    background_color: BackgroundColor(palette[1]),
                                    ..Default::default()
                                },))
                                .with_children(|parent| {
                                    parent.spawn(NodeBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(100.0), Val::Px(400.0)),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    });

                                    let id = parent
                                        .spawn((NodeBundle {
                                            style: Style {
                                                size: Size::new(Val::Px(100.0), Val::Px(300.0)),
                                                align_items: AlignItems::FlexEnd,
                                                justify_content: JustifyContent::FlexEnd,
                                                ..Default::default()
                                            },
                                            background_color: BackgroundColor(palette[2]),
                                            ..Default::default()
                                        },))
                                        .with_children(|parent| {
                                            parent.spawn(NodeBundle {
                                                style: Style {
                                                    size: Size::new(Val::Px(100.0), Val::Px(300.0)),
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            });

                                            let id = parent
                                                .spawn((NodeBundle {
                                                    style: Style {
                                                        size: Size::new(Val::Px(200.0), Val::Px(200.0)),
                                                        ..Default::default()
                                                    },
                                                    background_color: BackgroundColor(palette[3]),
                                                    ..Default::default()
                                                },))
                                                .id();
                                            target_ids.push(id);
                                        })
                                        .id();
                                    target_ids.push(id);
                                })
                                .id();
                            target_ids.push(id);
                        })
                        .id();
                    target_ids.push(id);
                });
        });
    target_ids
}

pub fn spawn_right_panel(
    parent: &mut ChildBuilder,
    text_style: TextStyle,
    palette: &[Color; 4],
    mut target_ids: Vec<Entity>,
) {
    let spawn_buttons = |parent: &mut ChildBuilder, target_id| {
        spawn_button::<Display>(parent, text_style.clone(), target_id);
        spawn_button::<Visibility>(parent, text_style.clone(), target_id);
    };
    parent
        .spawn(NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(10.)),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(500.0), Val::Px(500.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect {
                            left: Val::Px(5.),
                            top: Val::Px(5.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    background_color: BackgroundColor(palette[0]),
                    ..Default::default()
                })
                .with_children(|parent| {
                    spawn_buttons(parent, target_ids.pop().unwrap());

                    parent
                        .spawn((NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(400.0), Val::Px(400.0)),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::FlexEnd,
                                justify_content: JustifyContent::SpaceBetween,
                                padding: UiRect {
                                    left: Val::Px(5.),
                                    top: Val::Px(5.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            background_color: BackgroundColor(palette[1]),
                            ..Default::default()
                        },))
                        .with_children(|parent| {
                            spawn_buttons(parent, target_ids.pop().unwrap());

                            parent
                                .spawn((NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(300.0), Val::Px(300.0)),
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::FlexEnd,
                                        justify_content: JustifyContent::SpaceBetween,
                                        padding: UiRect {
                                            left: Val::Px(5.),
                                            top: Val::Px(5.),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    background_color: BackgroundColor(palette[2]),
                                    ..Default::default()
                                },))
                                .with_children(|parent| {
                                    spawn_buttons(parent, target_ids.pop().unwrap());

                                    parent
                                        .spawn((NodeBundle {
                                            style: Style {
                                                size: Size::new(Val::Px(200.0), Val::Px(200.0)),
                                                align_items: AlignItems::FlexStart,
                                                justify_content: JustifyContent::SpaceBetween,
                                                flex_direction: FlexDirection::Column,
                                                padding: UiRect {
                                                    left: Val::Px(5.),
                                                    top: Val::Px(5.),
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            },
                                            background_color: BackgroundColor(palette[3]),
                                            ..Default::default()
                                        },))
                                        .with_children(|parent| {
                                            spawn_buttons(parent, target_ids.pop().unwrap());

                                            parent.spawn(NodeBundle {
                                                style: Style {
                                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            });
                                        });
                                });
                        });
                });
        });
}

pub fn spawn_button<T>(parent: &mut ChildBuilder, text_style: TextStyle, target: Entity)
where
    T: Default + std::fmt::Debug + Send + Sync + 'static,
    Target<T>: TargetUpdate,
{
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    //height: Val::Px(24.),
                    align_self: AlignSelf::FlexStart,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                ..Default::default()
            },
            Target::<T>::new(target),
        ))
        .with_children(|builder| {
            builder.spawn(
                TextBundle::from_section(
                    format!("{}::{:?}", Target::<T>::NAME, T::default()),
                    text_style,
                )
                .with_text_alignment(TextAlignment::Center),
            );
        });
}

pub fn buttons_handler<T>(
    mut left_panel_query: Query<&mut <Target<T> as TargetUpdate>::TargetComponent>,
    mut visibility_button_query: Query<(&Target<T>, &Interaction, &Children), Changed<Interaction>>,
    mut text_query: Query<&mut Text>,
) where
    T: Send + Sync,
    Target<T>: TargetUpdate + Component,
{
    for (target, interaction, children) in visibility_button_query.iter_mut() {
        if matches!(interaction, Interaction::Clicked) {
            let mut target_value = left_panel_query.get_mut(target.id).unwrap();
            for &child in children {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.sections[0].value = target.update_target(target_value.as_mut());
                    text.sections[0].style.color = if text.sections[0].value.contains("None")
                        || text.sections[0].value.contains("Hidden")
                    {
                        Color::rgb(1.0, 0.7, 0.7)
                    } else {
                        Color::WHITE
                    };
                }
            }
        }
    }
}
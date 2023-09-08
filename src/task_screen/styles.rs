use bevy::prelude::*;

pub const PALETTE: [&str; 4] = ["27496D", "466B7A", "669DB3", "ADCBE3"];



pub const TASK_SCREEN_STYLE: Style = Style {
    flex_direction: FlexDirection::Column,
    flex_basis: Val::Percent(100.),
    align_items: AlignItems::Center,
    justify_content: JustifyContent::SpaceEvenly,
    ..Style::DEFAULT
};

pub const TASK_SCREEN_STYLE1: Style = Style {
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    ..Style::DEFAULT
};

pub const LEFT_PANEL_STYLE: Style = Style {
    size: Size::new(Val::Percent(50.0), Val::Px(520.0)),
    justify_content: JustifyContent::Center,
    ..Style::DEFAULT
};

pub const RIGHT_PANEL_STYLE: Style = Style {
    size: Size::new(Val::Percent(50.0), Val::Px(520.0)),
    justify_content: JustifyContent::Center,
    ..Style::DEFAULT
};


pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
}}



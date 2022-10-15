use bevy::{prelude::*, window::PresentMode};
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use iyes_loopless::prelude::*;

use crate::pong::components::*;
use crate::pong::resources::*;
use crate::pong::player::Player;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        let mut app = App::new();
        app.add_plugin(EguiPlugin)
            .insert_resource(
                WindowDescriptor {
                title: "4D Pong".to_string(),
                width: 500.,
                height: 500.,
                present_mode: PresentMode::Fifo,
                ..default()
                }
            ).add_loopless_state(PongState::LoadingAssets)
            .add_enter_system(PongState::InGame, ui_load_system);
    }
}


// Systems

fn ui_load_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Roboto-Regular.ttf");
    commands.spawn_bundle(
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        }
    ).with_children(
        |parent| {
            parent.spawn_bundle(
                get_text_bundle(
                    "0",
                    get_text_style(font.clone(), Color::BLUE),
                    JustifyContent::SpaceBetween,
                )
            ).insert(ScoreComponent(Player::Blue, 0));
            parent.spawn_bundle(
                get_text_bundle(
                    "4D Pong",
                    get_text_style(font.clone(), Color::WHITE),
                    JustifyContent::SpaceBetween,
                )
            );
            parent.spawn_bundle(
                get_text_bundle(
                    "0",
                    get_text_style(font, Color::RED),
                    JustifyContent::SpaceBetween,
                )
            ).insert(ScoreComponent(Player::Red, 0));
        }
    );
}

// End Systems

// Helper functions


fn get_text_bundle(
    text: &str,
    text_style: TextStyle,
    justify_content: JustifyContent,
) -> TextBundle {
    TextBundle::from_section(
        text.to_string(),
        text_style
    ).with_text_alignment(TextAlignment::TOP_CENTER)
    .with_style(
        Style {
            align_self: AlignSelf::FlexEnd,
            justify_content: justify_content,
            margin: UiRect::all(Val::Px(25.0)),
            ..Default::default()
        }
    )
}

fn get_text_style(font: Handle<Font>, color: Color) -> TextStyle {
    TextStyle {
        font: font,
        font_size: 50.0,
        color: color,
    }
}

// End Helper functions
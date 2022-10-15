use bevy::{prelude::*, window::PresentMode};
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};
use iyes_loopless::prelude::*;

use crate::pong::components::*;
use crate::pong::resources::*;
use crate::pong::player::Player;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_enter_system(PongState::SettingUpUI, configure_visuals)
            .add_enter_system(PongState::SettingUpUI, ui_load_system)
            .add_system(ui_system.run_in_state(PongState::InGame));
    }
}

// Systems

fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}


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

    commands.insert_resource(NextState(PongState::LoadingAssets));

}

fn ui_system(
    mut egui_ctx: ResMut<EguiContext>,
) {
    
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
        });
    });
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
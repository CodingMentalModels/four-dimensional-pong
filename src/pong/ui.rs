use bevy::{prelude::*};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use iyes_loopless::prelude::*;

use crate::pong::components::*;
use crate::pong::resources::*;
use crate::pong::constants::*;
use crate::pong::player::Player;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_enter_system(PongState::LoadingUI, configure_visuals)
            .add_enter_system(PongState::LoadingUI, ui_load_system)
            .add_system(ui_system.run_in_state(PongState::InGame))
            .add_system(paused_ui_system.run_in_state(PongState::Paused))
            .add_system(paused_input_system.run_in_state(PongState::Paused));
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

    commands.insert_resource(NextState(PongState::InGame));

}

fn ui_system(
    mut egui_ctx: ResMut<EguiContext>,
    projection_images: Res<ProjectionImages>,
) {
    
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
            egui::menu::menu_button(ui, "About", |ui| {
                ui.label("https://www.twitch.tv/codingmentalmodels");
            });
        });
    });

    let (xw_image, yw_image, zw_image) = projection_images.unpack();

    instantiate_projection_panel(&mut egui_ctx, xw_image, "xw-projection", "X-W Projection", egui::Align2::LEFT_BOTTOM);
    instantiate_projection_panel(&mut egui_ctx, yw_image, "yw-projection", "Y-W Projection", egui::Align2::CENTER_BOTTOM);
    instantiate_projection_panel(&mut egui_ctx, zw_image, "zw-projection", "Z-W Projection", egui::Align2::RIGHT_BOTTOM);
}

fn paused_ui_system(
    mut egui_ctx: ResMut<EguiContext>,
    mut ai_query: Query<&mut AIComponent>,
) {
    egui::Area::new("pause-menu")
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(
            egui_ctx.ctx_mut(), 
            |ui| {
                ui.with_layout(
                    egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new("Paused")
                            .size(20.)
                            .text_style(egui::TextStyle::Heading)
                            .underline()
                            .color(egui::Color32::BLACK)
                        );
                        ui.add_space(50.0);
                        let mut ai = ai_query.single_mut();
                        let mut new_speed: Option<Speed> = None;
                        new_speed = ai_speed_button(ui, "AI Speed Easy", AI_PADDLE_SPEED_EASY, ai.0).map_or(new_speed, |s| Some(s));
                        new_speed = ai_speed_button(ui, "AI Speed Medium", AI_PADDLE_SPEED_MEDIUM, ai.0).map_or(new_speed, |s| Some(s));
                        new_speed = ai_speed_button(ui, "AI Speed Hard", AI_PADDLE_SPEED_HARD, ai.0).map_or(new_speed, |s| Some(s));
                        match new_speed {
                            Some(speed) => {
                                ai.0 = speed;
                            },
                            None => (),
                        };
                    }
                );
            }
        );
}

fn paused_input_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(PongState::InGame));
    }
}

// End Systems

// Helper functions

fn ai_speed_button(
    ui: &mut egui::Ui,
    text: &str,
    speed: Speed,
    previous_speed: Speed,
) -> Option<Speed> {
    let color = if speed == previous_speed {
        egui::Color32::GREEN
    } else {
        egui::Color32::WHITE
    };

    if ui.button(egui::RichText::new(text).color(color)).clicked() {
        Some(speed)
    } else {
        None
    }
}

fn instantiate_projection_panel(egui_ctx: &mut EguiContext, image: Handle<Image>, id: &str, label: &str, align: egui::Align2) {
    let texture = egui_ctx.add_image(image);
    egui::Area::new(id)
        .anchor(align, egui::Vec2::ZERO)
        .show(
            egui_ctx.ctx_mut(), |ui| {
                ui.set_width(PROJECTION_AREA_WIDTH);
                ui.set_height(PROJECTION_AREA_HEIGHT);
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(label).color(egui::Color32::BLACK).underline());
                    ui.image(texture, egui::vec2(PROJECTION_AREA_WIDTH, PROJECTION_AREA_HEIGHT));
                });
            }
        );
}

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
use std::vec;

use bevy::{prelude::*, window::{PresentMode}};




pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(
            WindowDescriptor {
            title: "Pong".to_string(),
            width: 500.,
            height: 500.,
            present_mode: PresentMode::Fifo,
            ..default()
            }
        ).insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 2.0,
        }).add_startup_system(stage_load_system)
        .add_startup_system(ui_load_system);
    }
}

// Run Conditions


// End Run Conditions


// Resources



// End Resources


// Events



// End Events


// Components



// End Components


// Systems

fn stage_load_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
        let scene = asset_server.load("four-dimensional-pong.glb#Scene0");

        // to position our 3d model, simply use the Transform
        // in the SceneBundle
        commands.spawn_bundle(SceneBundle {
            scene: scene,
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        });

        let x_from_blender = 0.019767;
        let y_from_blender = -8.21107;
        let z_from_blender = 4.66824;
        let scalar = 0.5;
        commands.spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(x_from_blender*scalar, y_from_blender*scalar, z_from_blender*scalar).looking_at(Vec3::new(0.0, 0., 0.0), Vec3::Y),
            ..default()
        });
}

fn ui_load_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Roboto-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: Color::WHITE,
    };
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
                    text_style.clone(),
                    JustifyContent::SpaceBetween,
                )
            );
            parent.spawn_bundle(
                get_text_bundle(
                    "4D Pong",
                    text_style.clone(),
                    JustifyContent::SpaceBetween,
                )
            );
            parent.spawn_bundle(
                get_text_bundle(
                    "0",
                    text_style.clone(),
                    JustifyContent::SpaceBetween,
                )
            );
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

// End Systems



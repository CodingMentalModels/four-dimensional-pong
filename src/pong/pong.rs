use std::vec;
use rand::{seq::SliceRandom, distributions::Standard};

use bevy::{prelude::*, window::{PresentMode}};

const ARENA_LENGTH: f32 = 2.0;


pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(
            WindowDescriptor {
            title: "4D Pong".to_string(),
            width: 500.,
            height: 500.,
            present_mode: PresentMode::Fifo,
            ..default()
            }
        ).insert_resource(
            AmbientLight {
                color: Color::WHITE,
                brightness: 1.0 / 2.0,
            }
        ).insert_resource(Time::default())
        .add_startup_system(stage_load_system)
        .add_startup_system(ui_load_system)
        .add_startup_system(ball_initial_velocity_system)
        .add_system(ball_movement_system);
    }
}

// Run Conditions


// End Run Conditions


// Resources



// End Resources


// Events



// End Events


// Components

#[derive(Component)]
struct BallComponent;

#[derive(Component)]
struct PaddleComponent;

#[derive(Component)]
struct GoalComponent;


#[derive(Component)]
struct WallComponent;

#[derive(Component)]
struct PositionComponent(Vec3);

#[derive(Component)]
struct VelocityComponent(Vec3);


#[derive(Component)]
struct NeedsRenderingComponent;

// End Components


// Systems

fn stage_load_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
        let scene = asset_server.load("four-dimensional-pong.glb#Scene0");
        let ball = asset_server.load("four-dimensional-pong.glb#Mesh6");
        let ball_material = asset_server.load("four-dimensional-pong.glb#Material2");
        let player_paddle = asset_server.load("four-dimensional-pong.glb#Mesh1");
        let opponent_paddle = asset_server.load("four-dimensional-pong.glb#Mesh4");

        commands.spawn_bundle(
            SceneBundle {
                scene: scene,
                transform: Transform::from_xyz(0., 0., 0.),
                ..Default::default()
            }
        );

        commands.spawn_bundle(
            PbrBundle {
                mesh: ball,
                material: ball_material,
                ..Default::default()
            }
        ).insert(BallComponent)
        .insert(PositionComponent(Vec3::new(0., 0., 0.)))
        .insert(VelocityComponent(Vec3::new(0., 0., 0.)))
        .insert(NeedsRenderingComponent);

        commands.spawn_bundle(
            PbrBundle {
                mesh: player_paddle,
                ..Default::default()
            }
        ).insert(PaddleComponent)
        .insert(PositionComponent(Vec3::new(0., 0., -ARENA_LENGTH)))
        .insert(NeedsRenderingComponent);

        
        commands.spawn_bundle(
            PbrBundle {
                mesh: opponent_paddle,
                ..Default::default()
            }
        ).insert(PaddleComponent)
        .insert(PositionComponent(Vec3::new(0., 0., ARENA_LENGTH)))
        .insert(NeedsRenderingComponent);

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

fn ball_initial_velocity_system(
    mut ball_query: Query<(&mut VelocityComponent, &BallComponent)>,
) {
    let directions = vec![-1., 1.];
    let w_velocity = directions.choose(&mut rand::thread_rng()).expect("Directions is never empty.");
    for (mut velocity, ball) in ball_query.iter_mut() {
        velocity.0 = Vec4::new(0.0, 0.0, 0.0, *w_velocity);
    }
}

fn ball_movement_system(
    time: Res<Time>,
    mut ball_query: Query<(&mut PositionComponent, &VelocityComponent, &BallComponent)>,
) {
    for (mut position, velocity, ball) in ball_query.iter_mut() {
        position.0 += velocity.0 * time.delta_seconds();
    }
}

fn render_system(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut StandardMaterial, &PositionComponent), With<NeedsRenderingComponent>>,
) {
    for (mut transform, mut material, position) in query.iter_mut() {
        transform = transform.with_translation(position.0.truncate());
        material = material.with_color(get_color_from_w(position.0.w, ARENA_LENGTH));
    }
}

fn get_color_from_w(w: f32, arena_length: f32) -> Color {
    let blue = Color::BLUE.as_hsla();
    let red = Color::RED.as_hsla();
    let saturation = blue.1;
    let lightness = blue.2;
    let alpha = blue.3;
    let factor = (w + arena_length) / (2.0 * arena_length);
    Color::Hsla {
        hue: lerp(blue.0, red.0, factor),
        saturation: saturation,
        lightness: lightness,
        alpha: alpha
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

// End Systems


#[cfg(test)]
mod test_pong_plugin {
    use super::*;

    #[test]
    fn test_pong_plugin_initializes() {
        let app = App::new()
            .add_plugin(PongPlugin)
            .run();

        app.world.contains_resource::<Time>();
        app.world.contains_resource::<WindowDescriptor>();
        app.world.contains_resource::<AmbientLight>();
        app.query::<&PositionComponent>().iter().count() == 3;
        app.query::<&VelocityComponent>().iter().count() == 1;
        for (velocity) in app.query::<&VelocityComponent>() {
            assert!(velocity.0.truncate().distance(Vec3::new(0.0, 0.0, 0.0)) < 0.0001);
            assert!(velocity.0.w == 1.0 || velocity.0.w == -1.0);
        }
    }
}
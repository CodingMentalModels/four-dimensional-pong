use std::vec;
use rand::{seq::SliceRandom, distributions::Standard};

use bevy::{prelude::*, window::{PresentMode}, gltf::{Gltf, GltfMesh}, asset::LoadState};
use iyes_loopless::prelude::*;

const GLTF_PATH: &str = "pong.glb";
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
        .insert_resource(PongState::LoadingAssets)
        .add_loopless_state(PongState::LoadingAssets)
        .add_startup_system(load_gltf)
        .add_system(stage_load_system.run_in_state(PongState::LoadingAssets))
        .add_enter_system(PongState::InGame, ui_load_system)
        .add_enter_system(PongState::InGame, ball_initial_velocity_system)
        .add_system(ball_movement_system.run_in_state(PongState::InGame))
        .add_system(render_system.run_in_state(PongState::InGame));
    }
}

// Run Conditions

fn in_loading_assets_state(pong_state: Res<PongState>) -> bool {
    *pong_state == PongState::LoadingAssets
}

fn in_game_state(pong_state: Res<PongState>) -> bool {
    *pong_state == PongState::InGame
}

// End Run Conditions


// Resources

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PongState {
    LoadingAssets,
    InGame,
    Paused,
}

struct GltfModel(Handle<Gltf>);

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
struct PositionComponent(Vec4);

#[derive(Component)]
struct VelocityComponent(Vec4);

#[derive(Component)]
struct MaterialHandleComponent(Handle<StandardMaterial>);

#[derive(Component)]
struct NeedsRenderingComponent;

// End Components


// Systems

fn load_gltf(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let gltf = asset_server.load(GLTF_PATH);
    if asset_server.get_load_state(gltf.clone()) == LoadState::Failed {
        println!("Immediately failed to load gltf.");
    }
    commands.insert_resource(GltfModel(gltf));
}

fn stage_load_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    model: Res<GltfModel>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltf_meshes: Res<Assets<GltfMesh>>,
    mut pong_state: ResMut<PongState>,
) {
    if asset_server.get_load_state(&model.0) == LoadState::Failed {
        println!("Failed to load gltf.");
    }

    if let Some(model_root) = assets_gltf.get(&model.0) {

        let scene = model_root.scenes[0].clone();
        
        let ball = model_root.meshes[2].clone();
        let player_paddle = model_root.meshes[3].clone();
        let opponent_paddle = model_root.meshes[4].clone();

        let ball_material = model_root.named_materials["Ball Material"].clone();
        let player_paddle_material = model_root.named_materials["Blue Paddle Material"].clone();
        let opponent_paddle_material = model_root.named_materials["Red Paddle Material"].clone();
        
        commands.spawn_bundle(
            SceneBundle {
                scene: scene,
                transform: Transform::from_xyz(0., 0., 0.),
                ..Default::default()
            }
        );

        
        commands.spawn_bundle(
            PbrBundle {
                mesh: get_mesh_from_gltf_or_panic(&assets_gltf_meshes, &ball),
                material: ball_material.clone(),
                ..Default::default()
            }
        ).insert(BallComponent)
        .insert(PositionComponent(Vec4::ZERO))
        .insert(VelocityComponent(Vec4::ZERO))
        .insert(MaterialHandleComponent(ball_material))
        .insert(NeedsRenderingComponent);

        commands.spawn_bundle(
            PbrBundle {
                mesh: get_mesh_from_gltf_or_panic(&assets_gltf_meshes, &player_paddle),
                material: player_paddle_material.clone(),
                ..Default::default()
            }
        ).insert(PaddleComponent)
        .insert(PositionComponent(Vec4::new(0., 0., 0., -ARENA_LENGTH)))
        .insert(MaterialHandleComponent(player_paddle_material))
        .insert(NeedsRenderingComponent);

        
        commands.spawn_bundle(
            PbrBundle {
                mesh: get_mesh_from_gltf_or_panic(&assets_gltf_meshes, &opponent_paddle),
                material: opponent_paddle_material.clone(),
                ..Default::default()
            }
        ).insert(PaddleComponent)
        .insert(PositionComponent(Vec4::new(0., 0., 0., ARENA_LENGTH)))
        .insert(MaterialHandleComponent(opponent_paddle_material))
        .insert(NeedsRenderingComponent);

        let x_from_blender = 0.019767;
        let y_from_blender = -8.21107;
        let z_from_blender = 4.66824;
        let scalar = 0.5;
        commands.spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(x_from_blender*scalar, y_from_blender*scalar, z_from_blender*scalar).looking_at(Vec3::new(0.0, 0., 0.0), Vec3::Y),
            ..default()
        });

        *pong_state = PongState::InGame;
    }
}

fn get_mesh_from_gltf_or_panic(gltf_mesh_assets: &Res<Assets<GltfMesh>>, gltf_mesh_handle: &Handle<GltfMesh>) -> Handle<Mesh> {
    let gltf_mesh = gltf_mesh_assets.get(&gltf_mesh_handle).expect("The GLTFMesh should exist.");
    gltf_mesh.primitives[0].mesh.clone()
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
    mut ball_query: Query<(&mut VelocityComponent), With<BallComponent>>,
) {
    let directions = vec![-1., 1.];
    let w_velocity = directions.choose(&mut rand::thread_rng()).expect("Directions is never empty.");
    for (mut velocity) in ball_query.iter_mut() {
        velocity.0 = Vec4::new(0.0, 0.0, 0.0, *w_velocity);
    }
    panic!("ball_initial_velocity_system");
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
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&mut Transform, &mut MaterialHandleComponent, &PositionComponent), With<NeedsRenderingComponent>>,
) {
    for (mut transform, material, position) in query.iter_mut() {
        *transform = transform.with_translation(position.0.truncate());
        match materials.get_mut(&material.0) {
            Some(material) => {
                material.base_color = get_color_from_w(position.0.w, ARENA_LENGTH);
            },
            None => {
                panic!("Material not found.");
            }
        }
    }
}

fn get_color_from_w(w: f32, arena_length: f32) -> Color {
    let blue = Color::BLUE.as_hsla_f32();
    let red = Color::RED.as_hsla_f32();
    let saturation = blue[1];
    let lightness = blue[2];
    let alpha = blue[3];
    let factor = (w + arena_length) / (2.0 * arena_length);
    Color::Hsla {
        hue: lerp(blue[0], red[0], factor),
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
    use bevy::{asset::AssetPlugin, gltf::GltfPlugin};

    use super::*;

    #[test]
    fn test_assets_load() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugin(AssetPlugin)
            .add_plugin(GltfPlugin)
            .insert_resource(PongState::LoadingAssets)
            .add_asset::<bevy::pbr::prelude::StandardMaterial>()
            .add_asset::<bevy::render::prelude::Mesh>()
            .add_asset::<bevy::scene::Scene>()
            .add_startup_system(load_gltf.run_if(in_loading_assets_state))
            .add_system(stage_load_system.run_if(in_loading_assets_state));

        app.update();
        assert!(app.world.contains_resource::<GltfModel>());
        std::thread::sleep(std::time::Duration::from_millis(100));
        app.update();

        assert!(app.world.contains_resource::<AssetServer>());
        let asset_server = app.world.get_resource::<AssetServer>().expect("AssetServer should exist.");

        let model = app.world.get_resource::<GltfModel>();
        assert!(model.is_some());
        let model = model.unwrap();
        assert!(asset_server.get_load_state(model.0.clone()) != LoadState::Failed);
        assert!(asset_server.get_load_state(model.0.clone()) == LoadState::Loaded);
        
    }

    #[test]
    fn test_pong_plugin_initializes() {
        let mut app = App::new();
        app
            .add_plugins(MinimalPlugins)
            .add_plugin(AssetPlugin)
            .add_plugin(GltfPlugin)
            .add_plugin(PongPlugin)
            .add_asset::<bevy::pbr::prelude::StandardMaterial>()
            .add_asset::<bevy::render::prelude::Mesh>()
            .add_asset::<bevy::scene::Scene>();

        assert!(app.world.contains_resource::<Time>());
        assert!(app.world.contains_resource::<WindowDescriptor>());
        assert!(app.world.contains_resource::<AmbientLight>());
        assert!(app.world.contains_resource::<PongState>());
        
        app.update();
        assert!(app.world.contains_resource::<GltfModel>());
        std::thread::sleep(std::time::Duration::from_millis(100)); // Allow time for assets to load.
        app.update(); // PongState::LoadingAssets -> PongState::InGame
        
        let state = app.world.get_resource::<PongState>().expect("PongState should exist.");
        assert_eq!(*state, PongState::InGame);

        assert_eq!(app.world.query::<&PositionComponent>().iter(&app.world).count(), 3);
        assert_eq!(app.world.query::<&MaterialHandleComponent>().iter(&app.world).count(), 3);
        assert_eq!(app.world.query::<&VelocityComponent>().iter(&app.world).count(), 1);
        assert_eq!(app.world.query::<(&mut VelocityComponent, &BallComponent)>().iter(&app.world).count(), 1);
        app.update(); // Should run startup systems for PongState::InGame
        app.update();


        for (velocity) in app.world.query::<&VelocityComponent>().iter(&app.world) {
            assert!(velocity.0.truncate().distance(Vec3::ZERO) < 0.0001);
        }
        for (velocity, _) in app.world.query::<(&VelocityComponent, &BallComponent)>().iter(&app.world) {
            assert!(velocity.0.w == 1.0 || velocity.0.w == -1.0, "Velocity w component should be +1 or -1 but is: {}", velocity.0.w);;
            assert_eq!(velocity.0.length(), 1.0);
        }
    }
}
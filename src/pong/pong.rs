use std::vec;
use rand::{seq::SliceRandom, distributions::Standard};

use bevy::{prelude::*, window::{PresentMode}, gltf::{Gltf, GltfMesh}, asset::LoadState};
use iyes_loopless::prelude::*;

use crate::pong::player::Player;

const GLTF_PATH: &str = "pong.glb";
const ARENA_LENGTH: f32 = 5.0;
const ARENA_WIDTH: f32 = 2.0;
const PADDLE_STARTING_OFFSET: f32 = 0.5;
const GOAL_OFFSET_FROM_ARENA: f32 = 0.1;
const PADDLE_SPEED: f32 = 2.0;
const PADDLE_WIDTH: f32 = 0.1;
const BALL_RADIUS: f32 = 0.03;


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
        .insert_resource(Input::<KeyCode>::default())
        .add_event::<ScoreEvent>()
        .add_loopless_state(PongState::LoadingAssets)
        .add_startup_system(load_gltf)
        .add_system(stage_load_system.run_in_state(PongState::LoadingAssets))
        .add_enter_system(PongState::InGame, ui_load_system)
        .add_enter_system(PongState::InGame, ball_initial_velocity_system)
        .add_system(input_system.run_in_state(PongState::InGame))
        .add_system(movement_system.run_in_state(PongState::InGame))
        .add_system(collision_system.run_in_state(PongState::InGame))
        .add_system(render_system.run_in_state(PongState::InGame))
        .add_system(score_system.run_in_state(PongState::InGame));
    }
}

// Run Conditions

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScoreEvent(Player);

// End Events


// Components

#[derive(Component)]
struct PlayerInputComponent;

#[derive(Component)]
struct BallComponent;

#[derive(Component)]
struct PaddleComponent(Player);

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

#[derive(Component)]
struct ScoreComponent(Player, usize);

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
) {
    if asset_server.get_load_state(&model.0) == LoadState::Failed {
        println!("Failed to load gltf.");
    }

    if let Some(model_root) = assets_gltf.get(&model.0) {
        
        let arena = model_root.meshes[1].clone();
        let ball = model_root.meshes[2].clone();
        let player_paddle = model_root.meshes[3].clone();
        let opponent_paddle = model_root.meshes[4].clone();

        let arena_material = model_root.named_materials["Material.001"].clone();
        let ball_material = model_root.named_materials["Ball Material"].clone();
        let player_paddle_material = model_root.named_materials["Blue Paddle Material"].clone();
        let opponent_paddle_material = model_root.named_materials["Red Paddle Material"].clone();
        
        commands.spawn_bundle(
            PbrBundle {
                mesh: get_mesh_from_gltf_or_panic(&assets_gltf_meshes, &arena),
                material: arena_material.clone(),
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

        
        let player_starting_position = Vec4::new(0., 0., -PADDLE_STARTING_OFFSET, -(ARENA_LENGTH / 2.));
        let opponent_starting_position = Vec4::new(0., 0., PADDLE_STARTING_OFFSET, (ARENA_LENGTH / 2.0));

        commands.spawn_bundle(
            PbrBundle {
                mesh: get_mesh_from_gltf_or_panic(&assets_gltf_meshes, &player_paddle),
                material: player_paddle_material.clone(),
                transform: Transform::from_translation(player_starting_position.truncate()),
                ..Default::default()
            }
        ).insert(PaddleComponent(Player::Blue))
        .insert(PositionComponent(player_starting_position))
        .insert(VelocityComponent(Vec4::ZERO))
        .insert(MaterialHandleComponent(player_paddle_material))
        .insert(NeedsRenderingComponent)
        .insert(PlayerInputComponent);

        
        commands.spawn_bundle(
            PbrBundle {
                mesh: get_mesh_from_gltf_or_panic(&assets_gltf_meshes, &opponent_paddle),
                material: opponent_paddle_material.clone(),
                transform: Transform::from_translation(opponent_starting_position.truncate()),
                ..Default::default()
            }
        ).insert(PaddleComponent(Player::Red))
        .insert(PositionComponent(opponent_starting_position))
        .insert(VelocityComponent(Vec4::ZERO))
        .insert(MaterialHandleComponent(opponent_paddle_material))
        .insert(NeedsRenderingComponent);

        let x_from_blender = 0.019767;
        let y_from_blender = -8.21107;
        let z_from_blender = 4.66824;
        let scalar = 0.5;
        commands.spawn_bundle(
            Camera3dBundle {
                transform: Transform::from_xyz(x_from_blender*scalar, y_from_blender*scalar, z_from_blender*scalar).looking_at(Vec3::new(0.0, 0., 0.0), Vec3::Y),
                ..default()
            }
        );

        commands.insert_resource(NextState(PongState::InGame));
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

fn ball_initial_velocity_system(
    mut ball_query: Query<(&mut VelocityComponent), With<BallComponent>>,
) {
    for (mut velocity) in ball_query.iter_mut() {
        velocity.0 = roll_initial_velocity();
    }
}

fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut paddle_query: Query<&mut VelocityComponent, With<PlayerInputComponent>>,
) {
    for (mut velocity) in paddle_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::W) {
            velocity.0 += PADDLE_SPEED*Vec4::new(0., 0., 1., 0.);
        }
        if keyboard_input.just_pressed(KeyCode::S) {
            velocity.0 += PADDLE_SPEED*Vec4::new(0., 0., -1., 0.);
        }
        if keyboard_input.just_pressed(KeyCode::A) {
            velocity.0 += PADDLE_SPEED*Vec4::new(-1., 0., 0., 0.);
        }
        if keyboard_input.just_pressed(KeyCode::D) {
            velocity.0 += PADDLE_SPEED*Vec4::new(1., 0., 0., 0.);
        }
        if keyboard_input.just_pressed(KeyCode::Up) {
            velocity.0 += PADDLE_SPEED*Vec4::new(0., 1., 0., 0.);
        }
        if keyboard_input.just_pressed(KeyCode::Down) {
            velocity.0 += PADDLE_SPEED*Vec4::new(0., -1., 0., 0.);
        }

        
        if keyboard_input.just_released(KeyCode::W) {
            velocity.0 -= PADDLE_SPEED*Vec4::new(0., 0., 1., 0.);
        }
        if keyboard_input.just_released(KeyCode::S) {
            velocity.0 -= PADDLE_SPEED*Vec4::new(0., 0., -1., 0.);
        }
        if keyboard_input.just_released(KeyCode::A) {
            velocity.0 -= PADDLE_SPEED*Vec4::new(-1., 0., 0., 0.);
        }
        if keyboard_input.just_released(KeyCode::D) {
            velocity.0 -= PADDLE_SPEED*Vec4::new(1., 0., 0., 0.);
        }
        if keyboard_input.just_released(KeyCode::Up) {
            velocity.0 -= PADDLE_SPEED*Vec4::new(0., 1., 0., 0.);
        }
        if keyboard_input.just_released(KeyCode::Down) {
            velocity.0 -= PADDLE_SPEED*Vec4::new(0., -1., 0., 0.);
        }
    }
}

fn movement_system(
    time: Res<Time>,
    mut ball_query: Query<(&mut PositionComponent, &VelocityComponent)>,
) {
    for (mut position, velocity) in ball_query.iter_mut() {
        position.0 += velocity.0 * time.delta_seconds();
    }
}

fn clamp_3d(
    position: Vec4,
    min: Vec3,
    max: Vec3,
) -> Vec4 {
    let mut to_return = position;
    to_return.x = to_return.x.clamp(min.x, max.x);
    to_return.y = to_return.y.clamp(min.y, max.y);
    to_return.z = to_return.z.clamp(min.z, max.z);
    return to_return;
}

fn collision_system(
    mut ball_query: Query<(&mut PositionComponent, &mut VelocityComponent), With<BallComponent>>,
    mut paddle_query: Query<(&mut PositionComponent, &PaddleComponent), Without<BallComponent>>,
    mut score_event_writer: EventWriter<ScoreEvent>,
) {
    for (mut ball_position, mut ball_velocity) in ball_query.iter_mut() {
        match is_goal_collision(ball_position.0) {
            Some(player) => {
                ball_position.0 = Vec4::new(0., 0., 0., 0.);
                ball_velocity.0 = roll_initial_velocity();

                score_event_writer.send(ScoreEvent(player));
            },
            None => {
                // Do nothing
            }
        }
        for (mut paddle_position, paddle) in paddle_query.iter_mut() {
            let paddle_player = paddle.0;
            let paddle_front_w = paddle_position.0.w + PADDLE_WIDTH/2.0;
            
            let is_at_or_beyond_paddle = match paddle_player {
                Player::Blue => (ball_position.0.w - BALL_RADIUS) <= -paddle_front_w,
                Player::Red => (ball_position.0.w + BALL_RADIUS) >= paddle_front_w,
            };

            if is_at_or_beyond_paddle {
                match get_ball_paddle_collision(ball_position.0, paddle_position.0) {
                    Some(normal) => {
                        ball_velocity.0 = reflect(ball_velocity.0, normal);
                    },
                    None => {
                        // Do nothing
                    }
                }
            }
        }
    }
    for (mut paddle_position, paddle) in paddle_query.iter_mut() {
        let clamp_distance = ARENA_WIDTH/2. - PADDLE_WIDTH/2.;
        paddle_position.0 = clamp_3d(
            paddle_position.0, 
            -clamp_distance*Vec3::ONE,
            clamp_distance*Vec3::ONE,
        );
    }
}

fn reflect(vector: Vec4, normal: Vec4) -> Vec4 {
    let dot = vector.dot(normal);
    let twice_dot = 2. * dot;
    let twice_dot_times_normal = twice_dot * normal;
    let reflected = vector - twice_dot_times_normal;
    return reflected;
}

fn get_ball_paddle_collision(ball_position: Vec4, paddle_position: Vec4) -> Option<Vec4> {
    let ball_radius = BALL_RADIUS;
    let paddle_radius = PADDLE_WIDTH/2.;

    let ball_paddle_distance_x = (ball_position.x - paddle_position.x).abs();
    let ball_paddle_distance_y = (ball_position.y - paddle_position.y).abs();
    let ball_paddle_distance_z = (ball_position.z - paddle_position.z).abs();

    let is_x_collision = ball_paddle_distance_x <= ball_radius + paddle_radius;
    let is_y_collision = ball_paddle_distance_y <= ball_radius + paddle_radius;
    let is_z_collision = ball_paddle_distance_z <= ball_radius + paddle_radius;

    if is_x_collision && is_y_collision && is_z_collision {
        let normal = get_ball_paddle_collision_normal(ball_position, paddle_position);
        return Some(normal);
    } else {
        return None;
    }
}

fn get_ball_paddle_collision_normal(ball_position: Vec4, paddle_position: Vec4) -> Vec4 {
    let ball_paddle_distance = ball_position - paddle_position;
    let potential_normals = vec![
        Vec4::new(1., 0., 0., 0.),
        Vec4::new(-1., 0., 0., 0.),
        Vec4::new(0., 1., 0., 0.),
        Vec4::new(0., -1., 0., 0.),
        Vec4::new(0., 0., 1., 0.),
        Vec4::new(0., 0., -1., 0.),
    ];
    let mut to_return = Vec4::ZERO;
    let mut max_similarity = -999.0;
    for normal in potential_normals {
        let similarity = ball_paddle_distance.dot(normal);
        if similarity > max_similarity {
            max_similarity = ball_paddle_distance.dot(normal);
            to_return = normal;
        }
    }
    return to_return;
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

fn score_system(
    mut score_event_reader: EventReader<ScoreEvent>,
    mut score_query: Query<(&mut Text, &mut ScoreComponent)>,
) {
    for score_event in score_event_reader.iter() {
        for (mut text, mut score_component) in score_query.iter_mut() {
            if score_event.0 == score_component.0 {
                score_component.1 += 1;
                text.sections[0].value = (text.sections[0].value.parse::<u32>().unwrap() + 1).to_string();
            }
        }
    }
}


// End Systems

// Helper Functions

fn roll_initial_velocity() -> Vec4 {
    let directions = vec![-1., 1.];
    let w_velocity = directions.choose(&mut rand::thread_rng()).expect("Directions is never empty.");
    Vec4::new(0.0, 0.0, 0.0, *w_velocity)
}

fn is_goal_collision(position: Vec4) -> Option<Player> {
    let goal_distance = (ARENA_LENGTH + GOAL_OFFSET_FROM_ARENA) / 2.;
    if position.w >= goal_distance {
        Some(Player::Blue)
    } else if position.w <= -goal_distance {
        Some(Player::Red)
    } else {
        None
    }
}


fn get_color_from_w(w: f32, arena_length: f32) -> Color {
    let blue = Color::BLUE.as_hsla_f32();
    let red = Color::RED.as_hsla_f32();
    let saturation = blue[1];
    let lightness = blue[2];
    let alpha = blue[3];
    let factor = (w + arena_length / 2.) / arena_length;
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

// End Helper Functions

#[cfg(test)]
mod test_pong_plugin {
    use bevy::{asset::AssetPlugin, gltf::GltfPlugin};

    use super::*;

    fn initialize_pong_plugin_and_load_assets() -> App {
        let mut app = App::new();
        app
            .add_plugins(MinimalPlugins)
            .add_plugin(AssetPlugin)
            .add_plugin(GltfPlugin)
            .add_plugin(PongPlugin)
            .add_asset::<bevy::pbr::prelude::StandardMaterial>()
            .add_asset::<bevy::render::prelude::Mesh>()
            .add_asset::<bevy::scene::Scene>();


        app.update();
        assert!(app.world.contains_resource::<GltfModel>());
        std::thread::sleep(std::time::Duration::from_millis(100)); // Allow time for assets to load.
        app.update(); // PongState::LoadingAssets -> PongState::InGame

        return app;
    }

    #[test]
    fn test_assets_load() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugin(AssetPlugin)
            .add_plugin(GltfPlugin)
            .add_asset::<bevy::pbr::prelude::StandardMaterial>()
            .add_asset::<bevy::render::prelude::Mesh>()
            .add_asset::<bevy::scene::Scene>()
            .add_startup_system(load_gltf)
            .add_system(stage_load_system);

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
        assert_eq!(app.world.entities().len(), 5);
        
    }

    #[test]
    fn test_pong_plugin_initializes() {
        let mut app = initialize_pong_plugin_and_load_assets();
        
        assert!(app.world.contains_resource::<Time>());
        assert!(app.world.contains_resource::<WindowDescriptor>());
        assert!(app.world.contains_resource::<AmbientLight>());
        
        assert_eq!(app.world.query::<&PositionComponent>().iter(&app.world).count(), 3);
        assert_eq!(app.world.query::<&MaterialHandleComponent>().iter(&app.world).count(), 3);
        assert_eq!(app.world.query::<&VelocityComponent>().iter(&app.world).count(), 3);
        assert_eq!(app.world.query::<(&mut VelocityComponent, &BallComponent)>().iter(&app.world).count(), 1);
        app.update(); // Should run on_entry systems for PongState::InGame

        for (velocity) in app.world.query::<&VelocityComponent>().iter(&app.world) {
            assert!(velocity.0.truncate().distance(Vec3::ZERO) < 0.0001);
        }
        for (velocity, _) in app.world.query::<(&VelocityComponent, &BallComponent)>().iter(&app.world) {
            assert!(velocity.0.w == 1.0 || velocity.0.w == -1.0, "Velocity w component should be +1 or -1 but is: {}", velocity.0.w);;
            assert_eq!(velocity.0.length(), 1.0);
        }
    }

    #[test]
    fn test_can_score_goal() {
        let mut app = initialize_pong_plugin_and_load_assets();

        let new_ball_position = Vec4::new(0.0, 0.0, 0.0, (ARENA_LENGTH + GOAL_OFFSET_FROM_ARENA - 0.0001)/2.);

        let mut ball_query = app.world.query::<(&mut PositionComponent, &mut VelocityComponent, &BallComponent)>();
        for (mut position, mut velocity, _) in ball_query.iter_mut(&mut app.world) {
            position.0 = new_ball_position;
            velocity.0 = Vec4::new(0.0, 0.0, 0.0, 1.0);
        }
        let mut ball_query = app.world.query::<(&PositionComponent, &VelocityComponent, &BallComponent)>();
        for (position, velocity, _) in ball_query.iter(&mut app.world) {
            assert_eq!(position.0, new_ball_position);
            assert_eq!(velocity.0, Vec4::new(0.0, 0.0, 0.0, 1.0));
        }
        app.update(); // Ball should have moved by more than 0.0001, so collision system should run, and player should have scored.
        app.update(); // Call a second time to ensure that collision system has run after ball movement

        let mut ball_query = app.world.query::<(&PositionComponent, &VelocityComponent, &BallComponent)>();
        for (position, velocity, _) in ball_query.iter(&mut app.world) {
            assert!(position.0.distance(Vec4::ZERO) < 0.1);
            assert!(velocity.0.w == 1.0 || velocity.0.w == -1.0, "Velocity w component should be +1 or -1 but is: {}", velocity.0.w);;
            assert_eq!(velocity.0.length(), 1.0);
        }

        let mut score_query = app.world.query::<&ScoreComponent>();
        for score_component in score_query.iter(&app.world) {
            let (player, score) = (score_component.0, score_component.1);
            if player == Player::Blue {
                assert_eq!(score, 1);
            } else {
                assert_eq!(score, 0);
            }
        }

    }

    #[test]
    fn test_input_handling() {
        let mut app = initialize_pong_plugin_and_load_assets();

        app.world.resource_mut::<Input<KeyCode>>().press(KeyCode::W);
        app.update();
        app.update();

        let mut paddle_query = app.world.query::<(&mut PositionComponent, &PaddleComponent)>();
        for (mut position, paddle) in paddle_query.iter_mut(&mut app.world) {
            if paddle.0 == Player::Blue {
                assert!(position.0.z > 0.0);
                assert!(position.0.z <= ARENA_LENGTH/2.);
            } else {
                assert_eq!(position.0.y, 0.0);
            }
        }

        app.world.resource_mut::<Input<KeyCode>>().press(KeyCode::D);
        app.update();
        app.update();

        let mut paddle_query = app.world.query::<(&mut PositionComponent, &PaddleComponent)>();
        for (mut position, paddle) in paddle_query.iter_mut(&mut app.world) {
            if paddle.0 == Player::Blue {
                assert!(position.0.z > 0.0 && position.0.z <= ARENA_LENGTH/2.);
                assert!(position.0.x > 0.0 && position.0.x <= ARENA_LENGTH/2.);
            } else {
                assert_eq!(position.0.y, 0.0);
            }
        }

        app.world.resource_mut::<Input<KeyCode>>().press(KeyCode::Up);
        app.update();
        app.update();

        let mut paddle_query = app.world.query::<(&mut PositionComponent, &PaddleComponent)>();
        for (mut position, paddle) in paddle_query.iter_mut(&mut app.world) {
            if paddle.0 == Player::Blue {
                assert!(position.0.z > 0.0 && position.0.z <= ARENA_LENGTH/2.);
                assert!(position.0.x > 0.0 && position.0.x <= ARENA_LENGTH/2.);
                assert!(position.0.y > 0.0 && position.0.y <= ARENA_LENGTH/2.);
            } else {
                assert_eq!(position.0.y, 0.0);
            }
        }
    }

    #[test]
    fn test_is_goal_collision() {
        assert_eq!(is_goal_collision(Vec4::ZERO), None);
        assert_eq!(is_goal_collision(Vec4::new(0.0, 0.0, 0.0, (ARENA_LENGTH + GOAL_OFFSET_FROM_ARENA)/2.)), Some(Player::Blue));
        assert_eq!(is_goal_collision(Vec4::new(0.0, 0.0, 0.0, -(ARENA_LENGTH + GOAL_OFFSET_FROM_ARENA)/2.)), Some(Player::Red));
        assert_eq!(is_goal_collision(Vec4::new(1.0, 2.0, 5.0, (ARENA_LENGTH + GOAL_OFFSET_FROM_ARENA + 100.)/2.)), Some(Player::Blue));
        assert_eq!(is_goal_collision(Vec4::new(-5.0, -200.0, 6000.0, -(ARENA_LENGTH + GOAL_OFFSET_FROM_ARENA + 100.)/2.)), Some(Player::Red));
    }

    #[test]
    fn test_get_color_from_w() {
        assert_eq!(get_color_from_w(-ARENA_LENGTH/2., ARENA_LENGTH), Color::BLUE.as_hsla());
        assert_eq!(get_color_from_w(ARENA_LENGTH/2., ARENA_LENGTH), Color::RED.as_hsla());
    }

}
use std::vec;
use rand::{seq::SliceRandom, distributions::Standard};

use bevy::{prelude::*, window::{PresentMode}, gltf::{Gltf, GltfMesh}, asset::LoadState};
use iyes_loopless::prelude::*;

use crate::pong::components::*;
use crate::pong::resources::*;
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
            AmbientLight {
                color: Color::WHITE,
                brightness: 1.0 / 2.0,
            }
        ).insert_resource(Time::default())
        .insert_resource(Input::<KeyCode>::default())
        .add_event::<ScoreEvent>()
        .add_startup_system(load_gltf)
        .add_system(stage_load_system.run_in_state(PongState::LoadingAssets))
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

struct GltfModel(Handle<Gltf>);

// End Resources


// Events

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScoreEvent(Player);

// End Events

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
        for (paddle_position, _) in paddle_query.iter() {
            if is_ball_paddle_collision(ball_position.0, paddle_position.0) {
                ball_velocity.0 = reflect(ball_velocity.0);
            }
        }
    }
    for (mut paddle_position, _) in paddle_query.iter_mut() {
        let clamp_distance = ARENA_WIDTH/2. - PADDLE_WIDTH/2.;
        paddle_position.0 = clamp_3d(
            paddle_position.0, 
            -clamp_distance*Vec3::ONE,
            clamp_distance*Vec3::ONE,
        );
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

fn reflect(vector: Vec4) -> Vec4 {
    let mut to_return = vector;
    to_return.w *= -1.;
    return to_return;
}

fn is_ball_paddle_collision(ball_position: Vec4, paddle_position: Vec4) -> bool {

    let ball_radius = BALL_RADIUS;
    let paddle_radius = PADDLE_WIDTH/2.;

    // Paddle has no depth, so return false if ball is not in the same plane as the paddle.
    if (ball_position.w - paddle_position.w).abs() > ball_radius {
        return false;
    }
    
    // Eliminate cases where even if the paddle were a sphere, it wouldn't intersect the ball.
    if (ball_position.truncate() - paddle_position.truncate()).length() > ball_radius + paddle_radius {
        return false;
    }

    let ball_paddle_distance_x = (ball_position.x - paddle_position.x).abs();
    let ball_paddle_distance_y = (ball_position.y - paddle_position.y).abs();
    let ball_paddle_distance_z = (ball_position.z - paddle_position.z).abs();

    // If we're within a paddle radius in any coordinate, then we're colliding.
    if  (ball_paddle_distance_x < paddle_radius) ||
        (ball_paddle_distance_y < paddle_radius) ||
        (ball_paddle_distance_z < paddle_radius) {
        return true;
    }

    // The remaining case is that we're within a circle radius of a corner of the paddle.
    // Note that ball_paddle_distance is the normalizd position of the paddle (i.e. always in the positive x, y, z area)
    let corner_distance = (Vec3::new(ball_paddle_distance_x, ball_paddle_distance_y, ball_paddle_distance_z) - Vec3::ONE*paddle_radius).length();
    return corner_distance < ball_radius;
    

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
            .add_loopless_state(PongState::LoadingAssets)
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
    fn test_is_ball_paddle_collision() {
        assert!(is_ball_paddle_collision(Vec4::ZERO, Vec4::ZERO));
        assert!(is_ball_paddle_collision(Vec4::new(1., 2., 3., ARENA_LENGTH/2.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.)));
        assert!(!is_ball_paddle_collision(Vec4::new(1., 2., 3., ARENA_LENGTH/2.), Vec4::new(1., 2., 3., -ARENA_LENGTH/2.)));
        
        assert!(is_ball_paddle_collision(Vec4::new(1. + BALL_RADIUS, 2., 3., ARENA_LENGTH/2.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.)));
        assert!(is_ball_paddle_collision(Vec4::new(1., 2. + BALL_RADIUS, 3., ARENA_LENGTH/2.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.)));
        assert!(is_ball_paddle_collision(Vec4::new(1., 2., 3. + BALL_RADIUS, ARENA_LENGTH/2.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.)));
        assert!(is_ball_paddle_collision(Vec4::new(1. + BALL_RADIUS, 2. + BALL_RADIUS, 3. + BALL_RADIUS, ARENA_LENGTH/2.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.)));
        
        let just_beyond_distance = BALL_RADIUS + PADDLE_WIDTH/2. + 0.1;
        assert!(!is_ball_paddle_collision(Vec4::new(1. + just_beyond_distance, 2., 3., ARENA_LENGTH/2.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.)));
        assert!(!is_ball_paddle_collision(Vec4::new(1., 2. + just_beyond_distance, 3., ARENA_LENGTH/2.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.)));
        assert!(!is_ball_paddle_collision(Vec4::new(1., 2., 3. + just_beyond_distance, ARENA_LENGTH/2.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.)));
        
        let max_diagonal = (Vec3::ONE * PADDLE_WIDTH/2.0);
        assert!(is_ball_paddle_collision(Vec4::new(1., 2., 3., ARENA_LENGTH/2.) - (max_diagonal - 0.01).extend(0.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.)));
        assert!(!is_ball_paddle_collision(Vec4::new(1., 2., 3., ARENA_LENGTH/2.) - (max_diagonal + 0.01).extend(0.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.)));
    }

    #[test]
    fn test_get_color_from_w() {
        assert_eq!(get_color_from_w(-ARENA_LENGTH/2., ARENA_LENGTH), Color::BLUE.as_hsla());
        assert_eq!(get_color_from_w(ARENA_LENGTH/2., ARENA_LENGTH), Color::RED.as_hsla());
    }

}
use rand::{seq::SliceRandom, Rng};

use bevy::{prelude::*, gltf::{Gltf, GltfMesh}, asset::LoadState, render::{camera::{RenderTarget}}};
use iyes_loopless::prelude::*;

use crate::pong::components::*;
use crate::pong::resources::*;
use crate::pong::constants::*;
use crate::pong::player::Player;
use crate::pong::axis::Axis;

use super::rotations::{Rotation, Axis4};


pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Time::default())
        .insert_resource(Input::<KeyCode>::default())
        .add_event::<ScoreEvent>()
        .add_enter_system(PongState::InGame, ball_initial_velocity_system)
        .add_system(input_system.run_in_state(PongState::InGame))
        .add_system(movement_system.run_in_state(PongState::InGame))
        .add_system(ai_system.run_in_state(PongState::InGame))
        .add_system(collision_system.run_in_state(PongState::InGame))
        .add_system(projection_system.run_in_state(PongState::InGame))
        .add_system(render_system.run_in_state(PongState::InGame))
        .add_system(render_system.run_in_state(PongState::Paused))
        .add_system(score_system.run_in_state(PongState::InGame));
    }
}

// Run Conditions

// End Run Conditions


// Resources

// End Resources


// Events

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScoreEvent(Player);

// End Events

// Systems

fn ball_initial_velocity_system(
    mut ball_query: Query<(&mut VelocityComponent), With<BallComponent>>,
) {
    for (mut velocity) in ball_query.iter_mut() {
        velocity.0 = roll_initial_velocity();
    }
}

fn input_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut paddle_query: Query<&mut VelocityComponent, With<PlayerInputComponent>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(PongState::Paused));
    }
    for mut velocity in paddle_query.iter_mut() {
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

fn ai_system(
    mut ball_query: Query<(&PositionComponent, &VelocityComponent), With<BallComponent>>,
    mut paddle_query: Query<(&PositionComponent, &mut VelocityComponent, &AIComponent), Without<BallComponent>>,
) {
    for (ball_position, ball_velocity) in ball_query.iter_mut() {
        for (paddle_position, mut paddle_velocity, ai_component) in paddle_query.iter_mut() {
            let paddle_speed = ai_component.0;
            let mut direction = (ball_position.0 - paddle_position.0).truncate();
            direction = direction.normalize();
            paddle_velocity.0 = (direction * paddle_speed).extend(0.);
        }
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
        match is_wall_collision(ball_position.0) {
            Some(axis) => {
                ball_velocity.0 = reflect_on_axis(ball_velocity.0, axis);
            },
            None => {
                // Do nothing
            }
        }
        for (paddle_position, paddle_component) in paddle_query.iter() {
            let paddle_size_modifier = paddle_component.1;
            if is_ball_paddle_collision(ball_position.0, paddle_position.0, paddle_size_modifier) {
                ball_velocity.0 = reflect_w(ball_velocity.0);
            }
        }
    }
    for (mut paddle_position, paddle_component) in paddle_query.iter_mut() {
        let paddle_size_modifier = paddle_component.1;
        let clamp_distance = ARENA_WIDTH/2. - PADDLE_WIDTH * paddle_size_modifier/2.;
        paddle_position.0 = clamp_3d(
            paddle_position.0, 
            -clamp_distance*Vec3::ONE,
            clamp_distance*Vec3::ONE,
        );
    }
}

fn projection_system(
    position_query: Query<(Entity, &PositionComponent), Without<ProjectionComponent>>,
    mut projection_query: Query<(&mut PositionComponent, &ProjectionComponent)>,
) {
    for (mut projection_position, projection_component) in projection_query.iter_mut() {
        let (projection_entity, projection_translation, projection_rotations) = projection_component.unpack();
        let real_entity_position = position_query.iter()
            .filter(|(entity, _)| *entity == projection_entity)
            .next()
            .expect("A Projection Entity exists without a corresponding object.").1;
        
        let mut rotated_position = real_entity_position.0;
        for rotation in projection_rotations.iter() {
            rotated_position = rotation.rotate(rotated_position);
        }
        projection_position.0 = rotated_position + projection_translation;
    }
}

fn render_system(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&mut Transform, &mut MaterialHandleComponent, &PositionComponent, Option<&ProjectionComponent>, Option<&PaddleComponent>), With<NeedsRenderingComponent>>,
) {
    for (mut transform, material, position, maybe_projection, maybe_paddle_component) in query.iter_mut() {
        *transform = Transform::from_translation(position.0.truncate());
        match maybe_paddle_component {
            Some(paddle_component) => {
                let paddle_size_modifier = paddle_component.1;
                transform.scale = Vec3::ONE * paddle_size_modifier;
            },
            None => {
                // Do nothing
            }
        }
        match maybe_projection {
            Some(_) => {
                // Do nothing, let the material get updated by the non-projected w.  
            },
            None => {
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
    let rng = &mut rand::thread_rng();

    let directions = vec![-1., 1.];
    let w_velocity = directions.choose(rng).expect("Directions is never empty.");
    let x_velocity = rng.gen_range(0.0..1.0);
    let y_velocity = rng.gen_range(0.0..1.0);
    let z_velocity = rng.gen_range(0.0..1.0);
    Vec4::new(x_velocity, y_velocity, z_velocity, *w_velocity)
}

fn reflect_on_axis(position: Vec4, axis: Axis) -> Vec4 {
    match axis {
        Axis::X => Vec4::new(-position.x, position.y, position.z, position.w),
        Axis::Y => Vec4::new(position.x, -position.y, position.z, position.w),
        Axis::Z => Vec4::new(position.x, position.y, -position.z, position.w),
        Axis::W => Vec4::new(position.x, position.y, position.z, -position.w),
    }
}

fn reflect_w(vector: Vec4) -> Vec4 {
    reflect_on_axis(vector, Axis::W)
}

fn is_wall_collision(ball_position: Vec4) -> Option<Axis> {
    if ball_position.x.abs() > ARENA_WIDTH/2. {
        Some(Axis::X)
    } else if ball_position.y.abs() > ARENA_WIDTH/2. {
        Some(Axis::Y)
    } else if ball_position.z.abs() > ARENA_WIDTH/2. {
        Some(Axis::Z)
    } else {
        None
    }
}

fn is_ball_paddle_collision(ball_position: Vec4, paddle_position: Vec4, paddle_size_modifier: SizeModifier) -> bool {

    let ball_radius = BALL_RADIUS;
    let paddle_radius = (PADDLE_WIDTH * paddle_size_modifier)/2.;

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
    use bevy::{asset::AssetPlugin, gltf::GltfPlugin, window::WindowPlugin, input::InputPlugin};
    use bevy_egui::EguiPlugin;

    use crate::pong::{ui::UIPlugin, assets::LoadAssetsPlugin};

    use super::*;

    fn initialize_pong_plugin_and_load_assets() -> App {
        let mut app = App::new();
        app
            .add_plugins(MinimalPlugins)
            .add_plugin(AssetPlugin)
            .add_plugin(WindowPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(GltfPlugin)
            .add_loopless_state(PongState::LoadingAssets)
            .add_plugin(LoadAssetsPlugin)
            .add_plugin(UIPlugin)
            .add_plugin(PongPlugin)
            .add_asset::<bevy::pbr::prelude::StandardMaterial>()
            .add_asset::<bevy::render::prelude::Mesh>()
            .add_asset::<bevy::scene::Scene>()
            .add_asset::<Image>();


        app.update();
        assert!(app.world.contains_resource::<GltfModel>());
        std::thread::sleep(std::time::Duration::from_millis(100)); // Allow time for assets to load.
        app.update(); // PongState::LoadingAssets -> PongState::InGame

        return app;
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
        assert!(is_ball_paddle_collision(Vec4::ZERO, Vec4::ZERO, 1.0));
        assert!(is_ball_paddle_collision(Vec4::new(1., 2., 3., ARENA_LENGTH/2.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.), 1.0));
        assert!(!is_ball_paddle_collision(Vec4::new(1., 2., 3., ARENA_LENGTH/2.), Vec4::new(1., 2., 3., -ARENA_LENGTH/2.), 1.0));
        
        assert!(is_ball_paddle_collision(Vec4::new(1. + BALL_RADIUS, 2., 3., ARENA_LENGTH/2.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.), 1.0));
        assert!(is_ball_paddle_collision(Vec4::new(1., 2. + BALL_RADIUS, 3., ARENA_LENGTH/2.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.), 1.0));
        assert!(is_ball_paddle_collision(Vec4::new(1., 2., 3. + BALL_RADIUS, ARENA_LENGTH/2.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.), 1.0));
        assert!(is_ball_paddle_collision(Vec4::new(1. + BALL_RADIUS, 2. + BALL_RADIUS, 3. + BALL_RADIUS, ARENA_LENGTH/2.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.), 1.0));
        
        let just_beyond_distance = BALL_RADIUS + PADDLE_WIDTH/2. + 0.1;
        assert!(!is_ball_paddle_collision(Vec4::new(1. + just_beyond_distance, 2., 3., ARENA_LENGTH/2.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.), 1.0));
        assert!(!is_ball_paddle_collision(Vec4::new(1., 2. + just_beyond_distance, 3., ARENA_LENGTH/2.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.), 1.0));
        assert!(!is_ball_paddle_collision(Vec4::new(1., 2., 3. + just_beyond_distance, ARENA_LENGTH/2.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.), 1.0));
        
        let max_diagonal = (Vec3::ONE * PADDLE_WIDTH/2.0);
        assert!(is_ball_paddle_collision(Vec4::new(1., 2., 3., ARENA_LENGTH/2.) - (max_diagonal - 0.01).extend(0.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.), 1.0));
        assert!(!is_ball_paddle_collision(Vec4::new(1., 2., 3., ARENA_LENGTH/2.) - (max_diagonal + 0.01).extend(0.), Vec4::new(1., 2., 3., ARENA_LENGTH/2.), 1.0));
    }

    #[test]
    fn test_get_color_from_w() {
        assert_eq!(get_color_from_w(-ARENA_LENGTH/2., ARENA_LENGTH), Color::BLUE.as_hsla());
        assert_eq!(get_color_from_w(ARENA_LENGTH/2., ARENA_LENGTH), Color::RED.as_hsla());
    }

}
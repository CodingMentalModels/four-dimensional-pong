use bevy::prelude::*;
use super::player::Player;


#[derive(Component)]
pub struct PlayerInputComponent;

#[derive(Component)]
pub struct BallComponent;

#[derive(Component)]
pub struct PaddleComponent(pub Player);

#[derive(Component)]
pub struct GoalComponent;

#[derive(Component)]
pub struct WallComponent;

#[derive(Component)]
pub struct PositionComponent(pub Vec4);

#[derive(Component)]
pub struct VelocityComponent(pub Vec4);

#[derive(Component)]
pub struct MaterialHandleComponent(pub Handle<StandardMaterial>);

#[derive(Component)]
pub struct NeedsRenderingComponent;

#[derive(Component)]
pub struct ScoreComponent(pub Player, pub usize);
use bevy::prelude::*;
use super::player::Player;


#[derive(Component, Clone, Copy)]
pub struct PlayerInputComponent;

#[derive(Component, Clone, Copy)]
pub struct BallComponent;

#[derive(Component, Clone, Copy)]
pub struct PaddleComponent(pub Player);

#[derive(Component, Clone, Copy)]
pub struct GoalComponent;

#[derive(Component, Clone, Copy)]
pub struct WallComponent;

#[derive(Component, Clone, Copy)]
pub struct RenderTransformComponent(pub Transform);

#[derive(Component, Clone, Copy)]
pub struct PositionComponent(pub Vec4);

#[derive(Component, Clone, Copy)]
pub struct VelocityComponent(pub Vec4);

#[derive(Component, Clone, Copy)]
pub struct ProjectionComponent(pub Entity);

#[derive(Component, Clone)]
pub struct MaterialHandleComponent(pub Handle<StandardMaterial>);

#[derive(Component, Clone, Copy)]
pub struct NeedsRenderingComponent;

#[derive(Component, Clone, Copy)]
pub struct ScoreComponent(pub Player, pub usize);
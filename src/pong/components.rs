use bevy::prelude::*;
use crate::pong::player::*;
use crate::pong::rotations::Rotation;

#[derive(Component, Clone, Copy)]
pub struct PlayerInputComponent;

#[derive(Component, Clone, Copy)]
pub struct AIComponent(pub Target);

#[derive(Component, Clone, Copy)]
pub struct BallComponent;

#[derive(Component, Clone, Copy)]
pub struct PaddleComponent(pub Player);

#[derive(Component, Clone, Copy)]
pub struct GoalComponent;

#[derive(Component, Clone, Copy)]
pub struct WallComponent;

#[derive(Component, Clone, Copy)]
pub struct PositionComponent(pub Vec4);

#[derive(Component, Clone, Copy)]
pub struct VelocityComponent(pub Vec4);

#[derive(Component, Clone)]
pub struct ProjectionComponent(pub Entity, pub Vec4, pub Vec<Rotation>);

impl ProjectionComponent {
    pub fn unpack(&self) -> (Entity, Vec4, Vec<Rotation>) {
        (self.0, self.1, self.2.clone())
    }
}

#[derive(Component, Clone)]
pub struct MaterialHandleComponent(pub Handle<StandardMaterial>);

#[derive(Component, Clone, Copy)]
pub struct NeedsRenderingComponent;

#[derive(Component, Clone, Copy)]
pub struct ScoreComponent(pub Player, pub usize);
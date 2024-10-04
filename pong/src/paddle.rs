use bevy::prelude::*;

use crate::position;

#[derive(Component)]
pub struct Paddle;

#[derive(Bundle)]
pub struct PaddleBundle {
    pub paddle: Paddle,
    pub position: position::Position,
}

impl PaddleBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            paddle: Paddle,
            position: position::Position(Vec2::new(x, y)),
        }
    }
}

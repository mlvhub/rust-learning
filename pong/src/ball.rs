use bevy::prelude::*;

use crate::position;

#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    position: position::Position,
}

impl BallBundle {
    pub fn new() -> Self {
        Self {
            ball: Ball,
            position: position::Position(Vec2::new(0., 0.)),
        }
    }
}

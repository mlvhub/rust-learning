use bevy::prelude::*;

pub const BALL_SIZE: f32 = 5.;
pub const BALL_SPEED: f32 = 2.;

pub const PADDLE_WIDTH: f32 = 10.;
pub const PADDLE_HEIGHT: f32 = 50.;
pub const PADDLE_SPEED: f32 = 5.;

pub const GUTTER_HEIGHT: f32 = 20.;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Component)]
pub struct Shape(pub Vec2);

#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub shape: Shape,
    pub position: Position,
    pub velocity: Velocity,
}

impl BallBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            ball: Ball,
            shape: Shape(Vec2::new(BALL_SIZE, BALL_SIZE)),
            position: Position(Vec2::new(x, y)),
            velocity: Velocity(Vec2::new(BALL_SPEED, BALL_SPEED)),
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ai;

#[derive(Component)]
pub struct Paddle;

#[derive(Bundle)]
pub struct PaddleBundle {
    pub paddle: Paddle,
    pub shape: Shape,
    pub position: Position,
    pub velocity: Velocity,
}

impl PaddleBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            paddle: Paddle,
            shape: Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            position: Position(Vec2::new(x, y)),
            velocity: Velocity(Vec2::new(0., 0.)),
        }
    }
}

#[derive(Component)]
pub struct Gutter;

#[derive(Bundle)]
pub struct GutterBundle {
    pub gutter: Gutter,
    pub shape: Shape,
    pub position: Position,
}

impl GutterBundle {
    pub fn new(x: f32, y: f32, width: f32) -> Self {
        Self {
            gutter: Gutter,
            shape: Shape(Vec2::new(width, GUTTER_HEIGHT)),
            position: Position(Vec2::new(x, y)),
        }
    }
}

pub enum Scorer {
    Ai,
    Player,
}

#[derive(Event)]
pub struct Scored(pub Scorer);

#[derive(Resource, Default)]
pub struct Score {
    pub player: u32,
    pub ai: u32,
}

#[derive(Component)]
pub struct PlayerScore;

#[derive(Component)]
pub struct AiScore;

pub mod bonus;

use bevy::{math::Vec3, prelude::Component};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct MaxSpeed(pub f32);

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Dog;

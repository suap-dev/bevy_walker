use core::f32;

use avian3d::prelude::*;
use bevy::{
    color::palettes::css::{DARK_SALMON, LAWN_GREEN},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_systems(Startup, spawn_ground)
        .add_systems(Startup, spawn_pole)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_generic_lights)
        .add_systems(Update, player_controls)
        .add_systems(Update, update_player_camera)
        .run();
}

#[derive(Component)]
struct Ground;

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const GROUND_RADIUS: f32 = 100.0;
    const GROUND_THICKNESS: f32 = 0.2;

    commands.spawn((
        Ground,
        RigidBody::Static,
        Collider::cylinder(GROUND_RADIUS, GROUND_THICKNESS),
        PbrBundle {
            mesh: meshes.add(Cylinder::new(GROUND_RADIUS, GROUND_THICKNESS)),
            transform: Transform::from_xyz(0.0, -0.1, 0.0),
            material: materials.add(Color::from(LAWN_GREEN)),
            ..default()
        },
    ));
}

#[derive(Component)]
struct Pole;

fn spawn_pole(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const POLE_RADIUS: f32 = 0.5;
    const POLE_HEIGHT: f32 = 6.0;

    commands.spawn((
        Pole,
        RigidBody::Static,
        Collider::cylinder(POLE_RADIUS, POLE_HEIGHT),
        PbrBundle {
            mesh: meshes.add(Cylinder::new(POLE_RADIUS, POLE_HEIGHT)),
            transform: Transform::from_xyz(0.0, 3.0, 0.0),
            material: materials.add(Color::BLACK),
            ..default()
        },
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 20.0, 100.0).looking_at(Vec3::ZERO, Dir3::Y),
        ..default()
    });
}

fn spawn_generic_lights(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 10.0, 0.0).looking_at(Vec3::ZERO, Dir3::Y),
        visibility: Visibility::Visible,
        ..default()
    });
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct AcceleratedMovement {
    max_speed: f32,
    acceleration: f32,
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // player is a cube as of now
    const SIDE_LENGTH: f32 = 1.0;
    const BORDER_RADIUS: f32 = 0.2;

    commands.spawn((
        Player,
        AcceleratedMovement {
            max_speed: 4.0,
            acceleration: 10.0,
        },
        RigidBody::Dynamic,
        // LinearDamping(1.0),
        // AngularDamping(f32::MAX),
        Collider::cuboid(SIDE_LENGTH, SIDE_LENGTH, SIDE_LENGTH),
        PbrBundle {
            mesh: meshes.add(Cuboid::from_length(SIDE_LENGTH)),
            material: materials.add(Color::from(DARK_SALMON)),
            transform: Transform::from_xyz(3.0, 1.0, 3.0),
            ..default()
        },
    ));
}

fn player_controls(
    mut query: Query<(&mut LinearVelocity, &AcceleratedMovement), With<Player>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if let Ok((mut linear_velocity, accelerated_movement)) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keys.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            direction.z -= 1.0;
        }
        if keys.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            direction.z += 1.0;
        }

        if keys.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            direction.x -= 1.0;
        }
        if keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            direction.x += 1.0;
        }

        linear_velocity.x += direction.x * accelerated_movement.acceleration * time.delta_seconds();
        linear_velocity.z += direction.z * accelerated_movement.acceleration * time.delta_seconds();

        // linear_velocity.clamp_length_max(accelerated_movement.max_speed);

        // linear_velocity.x = f32::min(accelerated_movement.max_speed, linear_velocity.x);
        // linear_velocity.x = f32::min(accelerated_movement.max_speed, linear_velocity.y);
    }
}

fn update_player_camera(
    mut camera_transform: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    if let Ok(player_transform) = player_transform.get_single() {
        if let Ok(mut camera_transform) = camera_transform.get_single_mut() {
            camera_transform.rotation = camera_transform.looking_at(player_transform.translation, Dir3::Y).rotation;
        }
    }
}

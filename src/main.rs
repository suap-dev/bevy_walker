use bevy::color::palettes::css::{DARK_SALMON, LAWN_GREEN};
use bevy::math::vec3;
use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_ground)
        .add_systems(Startup, spawn_pole)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_generic_lights)
        .add_systems(Update, player_controls)
        .run();
}

#[derive(Component)]
struct Ground;

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Ground,
        PbrBundle {
            mesh: meshes.add(Cylinder::new(10.0, 0.2)),
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
    commands.spawn((
        Pole,
        PbrBundle {
            mesh: meshes.add(Cylinder::new(0.5, 6.0)),
            transform: Transform::from_xyz(0.0, 3.0, 0.0),
            material: materials.add(Color::BLACK),
            ..default()
        },
    ));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(15.0, 10.0, 15.0).looking_at(Vec3::ZERO, Dir3::Y),
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
struct Player {
    movement_speed: f32,
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player {
            movement_speed: 10.0,
        },
        PbrBundle {
            mesh: meshes.add(Cuboid::from_size(vec3(1.0, 1.0, 1.0))),
            material: materials.add(Color::from(DARK_SALMON)),
            transform: Transform::from_xyz(3.0, 1.0, 3.0),
            ..default()
        },
    ));
}

fn player_controls(
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
) {
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

    let movement_speed = query.single().1.movement_speed;
    let mut transform = query.single_mut().0;

    transform.translation += direction.normalize_or_zero() * movement_speed * time.delta_seconds();

    // let mut transform = single

    // query.single_mut().translation += direction.normalize()*;
}

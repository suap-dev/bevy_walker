use bevy::{input::mouse::MouseMotion, math::vec3, prelude::*};

use crate::{
    components::{MaxSpeed, Player, Position, Swinger},
    setup::MOUSE_SPEED,
};
pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = vec3(1.0, 2.0, 1.0);
    let amplitude = 0.05;
    let position = vec3(0.0, size.y / 2.0 + amplitude, 0.0);
    let transform = Transform::from_translation(position);

    commands.spawn((
        Player,
        Position(position),
        // Rotation::default(),
        Swinger {
            period: 4.0,
            amplitude,
        },
        MaxSpeed(10.0),
        Mesh3d(meshes.add(Cuboid::from_size(size))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        transform,
    ));
}

pub fn controls(
    mut player: Query<(&mut Position, &mut Transform, &MaxSpeed), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    // TODO: separate module for input mappings?
    let move_forward =
        keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW);
    let move_back =
        keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS);
    let strafe_left =
        keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA);
    let strafe_right =
        keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD);
    let jump =
        keyboard_input.pressed(KeyCode::Space);

    if move_forward {
        direction.z -= 1.0;
    }
    if move_back {
        direction.z += 1.0;
    }
    if strafe_right {
        direction.x += 1.0;
    }
    if strafe_left {
        direction.x -= 1.0;
    }
    if jump {
        direction.y += 1.0;
    }

    let mouse_dx: f32 = mouse_motion.read().map(|v2| v2.delta.x).sum::<f32>();

    let (mut position, mut transform, &MaxSpeed(max_speed)) = player.single_mut();

    transform.rotate_y(mouse_dx * -0.005 * MOUSE_SPEED);
    direction = (transform.rotation * direction).normalize_or_zero();

    // we don't want to change the translation of a player directly from here
    // because we also use a Swinger component to swing him up/down a bit;
    // swinger handles updating transform according to the Position component
    position.0 += direction * time.delta_secs() * max_speed;
}

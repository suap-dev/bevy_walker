use std::f32::consts::TAU;

use bevy::{math::vec3, prelude::*};

use crate::components::{Dog, Player};

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let beige_color = materials.add(Color::linear_rgb(
        249.0 / 255.0,
        196.0 / 255.0,
        138.0 / 255.0,
    ));

    // what are radians?
    // co to są radiany?

    // PI = 180 degrees

    // 2*PI to kąt pełny
    // PI to jest kąt półpełny
    // PI/2 to kąt prosty

    // TAU - kąt pełny
    // TAU/2 - kąt półpełny
    // TAU/4 - kąt prosty

    let transform = Transform {
        translation: vec3(3.0, 0.35, 1.0),
        rotation: Quat::from_rotation_x(TAU / 4.0),
        // rotation: Quat::from_rotation_x(1.6),
        // ~ 1.57 = kąt prosty
        // 3.14159265358979323846264338327950288_f32 = kąt półpełny
        scale: Vec3::ONE,
    };

    commands.spawn((
        Dog,
        Mesh3d(meshes.add(Capsule3d::new(0.2, 0.7))),
        MeshMaterial3d(beige_color),
        transform,
    ));
}

pub fn follow_player(
    mut dog_pos: Query<&mut Transform, (With<Dog>, Without<Player>)>,
    player_transform: Query<&Transform, With<Player>>,
) {
    for mut dog_transform in &mut dog_pos {
        let player_transform = player_transform.single();
        let player_position = player_transform.translation;
        let distance_from_player = 2.0;
        let new_dog_position =
            player_position + Vec3::ZERO.with_x(distance_from_player).with_y(-0.75);
        dog_transform.translation = new_dog_position;
    }
}

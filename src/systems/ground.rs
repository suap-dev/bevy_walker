use bevy::prelude::*;

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(40.0, 1.0))),
        MeshMaterial3d(materials.add(Color::linear_rgb(0.0 / 255.0, 250.0 / 255.0, 162.0 / 255.0))),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -0.499, 0.0),
    ));
}

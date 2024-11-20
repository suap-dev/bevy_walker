pub mod startup {
    use bevy::prelude::*;

    use crate::setup::*;

    pub fn spawn(mut commands: Commands) {
        commands.spawn(Camera3dBundle {
            transform: Transform::from_translation(CAMERA_POSITION)
                .looking_at(CAMERA_LOOK_AT, Dir3::Y),
            ..default()
        });
    }
}

pub mod startup {
    use bevy::prelude::*;

    use crate::components::PlayerCamera;

    pub fn spawn(mut commands: Commands) {
        commands.spawn((PlayerCamera, Camera3dBundle::default()));
    }
}

pub mod update {
    use bevy::prelude::*;

    use crate::{
        components::{Player, PlayerCamera, Position},
        setup::CAMERA_SWINGARM,
    };

    pub fn follow_player(
        mut camera_transform: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
        player_coords: Query<(&Position, &Transform), (With<Player>, Without<PlayerCamera>)>,
    ) {
        let mut camera_transform = camera_transform.single_mut();

        // let (&Position(player_position), &Rotation(rotation)) = player_coords.single();
        // camera_transform.translation = player_position + rotation * CAMERA_SWINGARM;
        // *camera_transform = camera_transform.looking_at(player_position, Dir3::Y);
        let (&Position(player_position), &transform) = player_coords.single();
        camera_transform.translation = player_position + transform.rotation * CAMERA_SWINGARM;
        *camera_transform = camera_transform.looking_at(player_position, Dir3::Y);
    }
}

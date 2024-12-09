pub mod startup {
    use bevy::prelude::*;

    use crate::components::PlayerCamera;

    pub fn spawn(mut commands: Commands) {
        commands.spawn((PlayerCamera, Camera3d::default()));
    }
}

pub mod update {
    use bevy::prelude::*;

    use crate::{
        components::{Player, PlayerCamera},
        setup::CAMERA_SWINGARM,
    };

    pub fn follow_player(
        mut camera_transform: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
        player_coords: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
    ) {
        let mut camera_transform = camera_transform.single_mut();
        let &transform = player_coords.single();
        camera_transform.translation = transform.translation + transform.rotation * CAMERA_SWINGARM;
        *camera_transform = camera_transform.looking_at(transform.translation, Dir3::Y);
    }
}

    use bevy::prelude::*;

    use crate::setup::LIGHT_POSITION;

    pub fn spawn(mut commands: Commands) {
        commands.spawn((
            PointLight {
                shadows_enabled: true,
                range: 60.0,
                ..default()
            },
            Transform::from_translation(LIGHT_POSITION),
        ));
    }

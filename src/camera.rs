use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::prelude::*;
use bevy::transform::TransformSystem;

#[derive(Component)]
pub struct CameraListensToInput(bool);

#[derive(Bundle)]
pub struct Camera {
    modifying: CameraListensToInput,

    #[bundle]
    pub camera: Camera3dBundle,
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera {
        modifying: CameraListensToInput(false),
        camera: Camera3dBundle {
            transform: Transform::from_xyz(0.0, 6., 12.0)
                .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            ..Default::default()
        },
    });
}

pub fn update_camera(
    mut events: EventReader<MouseMotion>,
    mut query: Query<(&CameraListensToInput, &mut Transform)>,
) {
    for event in events.iter() {
        for mut pair in query.iter_mut() {
            if pair.0 .0 {
                pair.1.translation += Vec3::new(event.delta.x * -0.05, 0.0, event.delta.y * -0.05);
            }
        }
    }
}

pub fn update_listening(
    mut events: EventReader<MouseButtonInput>,
    mut query: Query<&mut CameraListensToInput>,
) {
    for event in events.iter() {
        for mut camera in query.iter_mut() {
            if event.button == MouseButton::Right {
                camera.0 = !camera.0;
            }
        }
    }
}

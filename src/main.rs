use bevy::prelude::*;
mod camera;
mod terrain;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<terrain::TerrainShader>::default())
        .add_startup_system(setup)
        .add_startup_system(camera::setup_camera)
        .add_startup_system(terrain::setup_terrain)
        .add_system(camera::update_camera)
        .add_system(camera::update_listening)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
}

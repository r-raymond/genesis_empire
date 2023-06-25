use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
mod camera;
mod terrain;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(camera::setup_camera)
        .add_startup_system(terrain::setup_terrain)
        .add_system(camera::update_camera)
        .add_system(camera::update_listening)
        .run();
}

//#[derive(Component)]
//struct Shape;
//
//const X_EXTENT: f32 = 14.5;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //let debug_material = materials.add(StandardMaterial {
    //    base_color_texture: Some(images.add(uv_debug_texture())),
    //    ..default()
    //});

    //let shapes = [
    //    meshes.add(shape::Cube::default().into()),
    //    meshes.add(shape::Box::default().into()),
    //    meshes.add(shape::Capsule::default().into()),
    //    meshes.add(shape::Torus::default().into()),
    //    meshes.add(shape::Cylinder::default().into()),
    //    meshes.add(shape::Icosphere::default().try_into().unwrap()),
    //    meshes.add(shape::UVSphere::default().into()),
    //];

    //let num_shapes = shapes.len();

    //for (i, shape) in shapes.into_iter().enumerate() {
    //    commands.spawn((
    //        PbrBundle {
    //            mesh: shape,
    //            material: debug_material.clone(),
    //            transform: Transform::from_xyz(
    //                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
    //                2.0,
    //                0.0,
    //            )
    //            .with_rotation(Quat::from_rotation_x(-PI / 4.)),
    //            ..default()
    //        },
    //        Shape,
    //    ));
    //}

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

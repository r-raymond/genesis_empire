use bevy::prelude::*;
use std::collections::HashMap;

pub fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh: Mesh = shape::Plane {
        size: 50.0,
        subdivisions: 100,
    }
    .into();

    let mut elevation = HashMap::<(i64, i64), f32>::new();

    for value in &mut mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
        if let bevy::render::mesh::VertexAttributeValues::Float32x3(ref mut vec) = value {
            for i in 0..vec.len() {
                let x = (vec[i][0] * 1000.0) as i64;
                let z = (vec[i][2] * 1000.0) as i64;

                match elevation.get(&(x, z)) {
                    Some(e) => vec[i][1] = *e,
                    None => {
                        vec[i][1] = rand::random::<f32>() * 0.5;
                        elevation.insert((x, z), vec[i][1]);
                    }
                }
            }
            println!("{:?}", vec);
        }
    }

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    });
}

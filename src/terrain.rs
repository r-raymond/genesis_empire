use bevy::prelude::*;

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

    for value in &mut mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
        println!("{:?}", value);
        if let bevy::render::mesh::VertexAttributeValues::Float32x3(ref mut vec) = value {
            println!("{:?}", vec);
            for i in 0..vec.len() {
                vec[i][1] += rand::random::<f32>() * 0.5;
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

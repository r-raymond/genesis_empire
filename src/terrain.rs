use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use std::collections::HashMap;

static HEX_SIZE: f32 = 1.0;

fn pixel_to_hex(x: f32, y: f32) -> (f32, f32) {
    let q = (x * (3.0_f32).sqrt() - y) / 3.0 / HEX_SIZE;
    let r = y * 2.0 / 3.0 / HEX_SIZE;
    (q, r)
}

fn axial_round(x: f32, y: f32) -> (i64, i64) {
    let x_grid = x.round();
    let y_grid = y.round();

    let x_diff = x - x_grid;
    let y_diff = y - y_grid;

    if x_diff.abs() >= y_diff.abs() {
        (
            x_grid as i64 + (x_diff + 0.5 * y_diff).round() as i64,
            y_grid as i64,
        )
    } else {
        (
            x_grid as i64,
            y_grid as i64 + (y_diff + 0.5 * x_diff).round() as i64,
        )
    }
}

pub fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TerrainShader>>,
) {
    let mut mesh: Mesh = shape::Plane {
        size: 50.0,
        subdivisions: 400,
    }
    .into();

    let mut elevation = HashMap::<(i64, i64), f32>::new();

    for value in &mut mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) {
        if let bevy::render::mesh::VertexAttributeValues::Float32x3(ref mut vec) = value {
            for i in 0..vec.len() {
                let (q, r) = pixel_to_hex(vec[i][0], vec[i][2]);
                let (q, r) = axial_round(q, r);

                match elevation.get(&(q, r)) {
                    Some(e) => vec[i][1] = *e,
                    None => {
                        vec[i][1] = 0.5 + rand::random::<f32>() * 0.1;
                        elevation.insert((q, r), vec[i][1]);
                    }
                }
            }
        }
    }

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        material: materials.add(TerrainShader {}),
        ..default()
    });
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct TerrainShader {}

impl Material for TerrainShader {
    fn fragment_shader() -> ShaderRef {
        "shaders/terrain.wgsl".into()
    }
}

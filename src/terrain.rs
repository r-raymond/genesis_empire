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

fn axial_round(q: f32, r: f32) -> (i64, i64) {
    let q_grid = q.round();
    let r_grid = r.round();

    let q_diff = q - q_grid;
    let r_diff = r - r_grid;

    if q_diff.abs() >= r_diff.abs() {
        (
            q_grid as i64 + (q_diff + 0.5 * r_diff).round() as i64,
            r_grid as i64,
        )
    } else {
        (
            q_grid as i64,
            r_grid as i64 + (r_diff + 0.5 * q_diff).round() as i64,
        )
    }
}

pub fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TerrainShader>>,
    assert_server: Res<AssetServer>,
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
                        vec[i][1] = rand::random::<f32>() * 1.0;
                        elevation.insert((q, r), vec[i][1]);
                    }
                }
            }
        }
    }

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        material: materials.add(TerrainShader {
            dirt: Some(assert_server.load("textures/dirt.png")),
            grass: Some(assert_server.load("textures/grass.png")),
            alpha_mode: AlphaMode::Blend,
        }),
        ..default()
    });
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct TerrainShader {
    #[texture(0)]
    #[sampler(1)]
    grass: Option<Handle<Image>>,
    #[texture(2)]
    #[sampler(3)]
    dirt: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

impl Material for TerrainShader {
    fn fragment_shader() -> ShaderRef {
        "shaders/terrain.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}



@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    var grass: vec4<f32> = vec4<f32>(0.0, 0.5, 0.0, 1.0);
    var dirt: vec4<f32> = vec4<f32>(0.5, 0.25, 0.0, 1.0);
    var rock: vec4<f32> = vec4<f32>(0.25, 0.25, 0.25, 1.0);
    var snow: vec4<f32> = vec4<f32>(1.0, 1.0, 1.0, 1.0);

    var color: vec4<f32>;
    var height: f32 = world_position.y;
    if (height < 0.2) {
        color = dirt;
    } else if (height < 0.7) {
        color = grass;
    } else if (height < 0.9) {
        color = rock;
    } else {
        color = snow;
    }
    return color;
}

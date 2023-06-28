const HEX_SIZE: f32 = 1.0;
const LINE_SIZE: f32 = 0.02;

fn pixel_to_hex(pos: vec2<f32>) -> vec2<f32> {
    var q: f32 = (pos.x * sqrt(3.0) - pos.y) / 3.0 / HEX_SIZE;
    var r: f32 = pos.y * 2.0 / 3.0 / HEX_SIZE;
    return vec2<f32>(q, r);
}


@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    var grass: vec4<f32> = vec4<f32>(0.0, 0.5, 0.0, 1.0);
    var dirt: vec4<f32> = vec4<f32>(0.5, 0.25, 0.0, 1.0);
    var rock: vec4<f32> = vec4<f32>(0.25, 0.25, 0.25, 1.0);
    var snow: vec4<f32> = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    var blue: vec4<f32> = vec4<f32>(0.15, 0.97, 0.99, 1.0);

    var hex: vec2<f32> = pixel_to_hex(world_position.xz);

    if (abs(0.5 - abs(hex.x - trunc(hex.x))) < LINE_SIZE || abs(0.5 - abs(hex.y - trunc(hex.y))) < LINE_SIZE) {
        return blue;
    }

    var height: f32 = world_position.y;
    if (height < 0.2) {
        return dirt;
    } else if (height < 0.7) {
        return grass;
    } else if (height < 0.9) {
        return rock;
    } else {
        return snow;
    }
}

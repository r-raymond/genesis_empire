const HEX_SIZE: f32 = 1.0;
const LINE_SIZE: f32 = 0.02;

fn pixel_to_hex(pos: vec2<f32>) -> vec2<f32> {
    var q: f32 = (pos.x * sqrt(3.0) - pos.y) / 3.0 / HEX_SIZE;
    var r: f32 = pos.y * 2.0 / 3.0 / HEX_SIZE;
    return vec2<f32>(q, r);
}

fn close_to_border(pos: vec2<f32>) -> f32 {
    var hex: vec2<f32> = pixel_to_hex(pos);

    var x_grid = round(hex.x);
    var y_grid = round(hex.y);

    var x_diff = hex.x - x_grid;
    var y_diff = hex.y - y_grid;

    var smallest: f32 = 1.0;

    if (x_diff >= 1.0 / 3.0 && y_diff >= 1.0 / 3.0) {
        smallest = min(smallest, abs(y_diff - x_diff));
    }
    if (x_diff <= -1.0 / 3.0 && y_diff <= -1.0 / 3.0) {
        smallest = min(smallest, abs(y_diff - x_diff));
    }
    if (abs(x_diff) >= abs(y_diff)) {
        smallest = min(smallest, abs(0.5 - abs(x_diff + 0.5 * y_diff)));
    } else {
        smallest = min(smallest, abs(0.5 - abs(y_diff + 0.5 * x_diff)));
    }

    return smallest;
}


@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    var grass: vec4<f32> = vec4<f32>(0.0, 0.5, 0.0, 1.0);
    var dirt: vec4<f32> = vec4<f32>(0.5, 0.25, 0.0, 1.0);
    var rock: vec4<f32> = vec4<f32>(0.25, 0.25, 0.25, 1.0);
    var snow: vec4<f32> = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    var grey: vec4<f32> = vec4<f32>(0.6, 0.6, 0.6, 0.6);

    var hex: vec2<f32> = pixel_to_hex(world_position.xz);

    var height: f32 = world_position.y;
    var color: vec4<f32>;
    if (height < 0.2) {
        color = dirt;
    } else if (height < 0.7) {
        color = grass;
    } else if (height < 0.9) {
        color = rock;
    } else {
        color = snow;
    }

    var diff: f32 = close_to_border(world_position.xz);
    if (diff < LINE_SIZE) {
        return mix(grey, color, diff / LINE_SIZE);
    }

    return color;
}

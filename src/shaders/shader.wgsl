// Vertex shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) position: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    if in_vertex_index == u32(0) {
        out.clip_position = vec4<f32>(-1.0, -1.0, 0.0, 1.0);
        out.position = vec2<f32>(-1.0, -1.0);
    } else if in_vertex_index == u32(1) {
        out.clip_position = vec4<f32>(3.0, -1.0, 0.0, 1.0);
        out.position = vec2<f32>(3.0, -1.0);
    } else {
        out.clip_position = vec4<f32>(-1.0, 3.0, 0.0, 1.0);
        out.position = vec2<f32>(-1.0, 3.0);
    }
    return out;
}

// Fragment shader
struct Camera {
    position: vec2<f32>,
    vertical_scale: f32,
    scale_factor: f32,
    vertical_resolution: f32,
    is_multisampling: i32,
    coloring_scheme: i32,
    padding: i32,
};

@group(0) @binding(0) var<uniform> camera: Camera;

fn get_brightness(position: vec2<f32>) -> f32 {
    let x0 = (position.x * camera.vertical_scale * camera.scale_factor) + camera.position.x;
    let y0 = (position.y * camera.vertical_scale) + camera.position.y;

    var x = 0.0;
    var y = 0.0;

    var x2 = 0.0;
    var y2 = 0.0;

    var iteration = 0;
    let max_iteration = 2000;

    loop {
        if x2 + y2 <= 4.0 && iteration < max_iteration {
            y = (x + x) * y + y0;
            x = x2 - y2 + x0;
            x2 = x * x;
            y2 = y * y;
            iteration += 1;
        } else {
            break;
        }
    }

    return f32(iteration) / f32(max_iteration);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    //var brightness = get_brightness(in.position);
    var brightness = 0.0;

    if camera.is_multisampling == 1 {
        let y_offset: f32 = (1.0 / (camera.vertical_resolution / 2.0)) / 3.0;
        let x_offset: f32 = y_offset / camera.scale_factor;

        brightness += get_brightness(in.position);

        brightness += get_brightness((in.position + vec2<f32>(x_offset, y_offset)));
        brightness += get_brightness((in.position + vec2<f32>(x_offset, -y_offset)));
        brightness += get_brightness((in.position + vec2<f32>(-x_offset, y_offset)));
        brightness += get_brightness((in.position + vec2<f32>(-x_offset, -y_offset)));

        brightness += get_brightness((in.position + vec2<f32>(0.0, y_offset)));
        brightness += get_brightness((in.position + vec2<f32>(0.0, -y_offset)));
        brightness += get_brightness((in.position + vec2<f32>(x_offset, 0.0)));
        brightness += get_brightness((in.position + vec2<f32>(-x_offset, 0.0)));

        brightness = (brightness / (9.0));
    } else {
        brightness += get_brightness(in.position);
    }

    switch camera.coloring_scheme {
        case 0 {
            return vec4<f32>(brightness / 8.0, (sin(brightness * 3.14159) * 0.75) + (brightness / 4.0), (sin(brightness * 3.14159) / 2.0) + (brightness / 2.0), 1.0);
        }
        case 1 {
            return vec4<f32>(sin(brightness * 3.14159), sin(brightness * 3.14159), sin(brightness * 3.14159), 1.0);
        }
        case 2 {
            return vec4<f32>(brightness, brightness, brightness, 1.0);
        }
        case 3 {
            return vec4<f32>((1.0 - brightness) / 2.0, (1.0 - brightness) / 6.0, (1.0 - brightness) / 1.5, 1.0);
        }
        default: {
            return vec4<f32>(1.0 - brightness, 1.0 - brightness, 1.0 - brightness, 1.0);
        }
    }
}
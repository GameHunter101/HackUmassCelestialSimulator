struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>
}

struct Uniforms {
    iResolution: vec2<f32>,
    iTime: f32,
    iMouse: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms : Uniforms;

fn map(p : vec3f) -> f32 {
    return length(p) - 1.0;
}

@fragment
fn main(vertex_output: VertexOutput) -> @location(0) vec4f {
    // var uv = vec2f((vertex_output.clip_position * 2. - uniforms.iResolution.xy) / uniforms.iResolution.y);
    let uv = vertex_output.clip_position.xy / uniforms.iResolution;

    // Initialization
    let rayOrigin = vec3f(0., 0., -3.);
    let rayDirection = normalize(vec3(uv, 1.));
    var color = vec3(0.0);

    var totalDist = 0.;

    // Ray marching
    for (var i = 0; i < 80; i++){
        var position : vec3f = rayOrigin + rayDirection * totalDist; // our postion along the ray

        var distance = map(position);

        totalDist += distance;

        color = vec3f(f32(i)) / 80.;

        if (distance < .001 || totalDist > 100.) {
            break;
        };
    }

    // Coloring
    color = vec3(totalDist * .2);

    return vec4(color, 1.0);
}

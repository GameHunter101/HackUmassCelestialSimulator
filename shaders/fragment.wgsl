struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

struct Camera {
    pos: vec3<f32>,
    matrix: mat4x4<f32>,
}

struct Planet {
    mass: f32,
    pos: vec3<f32>,
    padding: f32,
    radius: f32,
}

struct Planets {
    planets: array<Planet,5>
}

struct Uniforms {
    iResolution: vec2<f32>,
    iMouse: vec2<f32>,
    iTime: f32,
    padding: f32,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(1) @binding(0)
var<uniform> planets: Planets;

@group(2) @binding(0)
var<uniform> uniforms : Uniforms;

fn map(p : vec3f) -> f32 {
    // This is our interface for translating the sphere
    var spherePosition = vec3f(2.,2.,0);
    var sphere = sdSphere(p - spherePosition, 1.);

    return sphere;
}

fn sdSphere(position : vec3f, s : f32) -> f32 {
    return length(position) - s;
}

fn rot2D(angle : f32) -> mat2x2<f32>{
    let s = sin(angle);
    let c = cos(angle);
    return mat2x2<f32>(c, -s, s, c);
}

@fragment
fn main(vertex_output: VertexOutput) -> @location(0) vec4f {
    // var uv = vec2f((vertex_output.clip_position * 2. - uniforms.iResolution.xy) / uniforms.iResolution.y);
    let uv = vertex_output.tex_coords;

    // Initialization
    let rayOrigin = vec3f(0., 0., -3.);
    let rayDirection = normalize(vec3(uv * 2.5, 1.));
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

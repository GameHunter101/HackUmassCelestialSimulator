struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

struct Camera {
    matrix: mat3x3<f32>,
    pos: vec3<f32>,
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
    iMouse: vec2<f32>,
    iResolution: vec2<f32>,
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
    var spherePosition = vec3f(0.,0.,-1.5);
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
    let uv = vertex_output.tex_coords * vec2f(uniforms.iResolution.x / uniforms.iResolution.y, 1.0);
    let m = (uniforms.iMouse.xy * 2 - uniforms.iResolution.xy) / uniforms.iResolution.y;
    let FOV = 60 * (3.14159265 / 180);

    // return vec4f(uv, 0.0, 1.0);

    // Initialization
    var rayOrigin = camera.pos;
    var rayDirection = normalize(camera.matrix*vec3f(uv,1/tan(FOV/2)));
    var color = vec3(0.0);

    var totalDist = 0.;

    // var xzSwizzle : vec2;
    // rayOrigin.x *= rot2D(-m.x);
    // rayOrigin.z *= rot2D(-m.x);
    // rayDirection.x *= rot2D(-m.x);
    // rayDirection.z *= rot2D(-m.x);

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

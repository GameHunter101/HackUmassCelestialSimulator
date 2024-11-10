struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

struct Camera {
    pos: vec4<f32>,
    matrix: mat4x4<f32>,
}

struct Planet {
    pos: vec3<f32>,
    radius: f32,
}

struct Planets {
    planets: array<Planet, 128>,
    /* planet_count: u32,
    padding: f32, */
}

struct Uniforms {
    iMouse: vec2<f32>,
    iResolution: vec2<f32>,
    iTime: f32,
    planet_count: u32,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(1) @binding(0)
var<uniform> planets: Planets;

@group(2) @binding(0)
var<uniform> uniforms : Uniforms;

fn map(p : vec3f) -> f32 {
    // This is our interface for translating the sphere

    var spherePosition = planets.planets[1].pos;

    var current_min = sdSphere(p - spherePosition, planets.planets[1].radius);
    for (var i = 2; i < i32(uniforms.planet_count); i++) {
        let sphere_position = planets.planets[i].pos;
        current_min = min(sdSphere(p-sphere_position, planets.planets[i].radius), current_min);
    }

    // return sphere;
    return current_min;
}

fn sdSphere(position : vec3f, s : f32) -> f32 {
    return length(position) - s;
}

fn rot2D(angle : f32) -> mat2x2<f32>{
    let s = sin(angle);
    let c = cos(angle);
    return mat2x2<f32>(c, -s, s, c);
}

fn getNormal(position : vec3f) -> vec3f {
    let d = vec2f(0.01, 0.0);
    let gradientX = map(position + d.xyy) - map(position - d.xyy);
    let gradientY = map(position + d.yxy) - map(position - d.yxy);
    let gradientZ = map(position + d.yyx) - map(position - d.yyx);
    let normal = vec3f(gradientX, gradientY, gradientZ);
    return normalize(normal);
}

@fragment
fn main(vertex_output: VertexOutput) -> @location(0) vec4f {
    let uv = vertex_output.tex_coords * vec2f(uniforms.iResolution.x / uniforms.iResolution.y, 1.0);
    let m = (uniforms.iMouse.xy * 2 - uniforms.iResolution.xy) / uniforms.iResolution.y;
    let FOV = 60 * (3.14159265 / 180);
    
    // return vec4f(uv, 0.0, 1.0);

    let matrix = mat3x3<f32>(
        camera.matrix[0].xyz,
        camera.matrix[1].xyz,
        camera.matrix[2].xyz,
    );

    // let matrix = mat3x3<f32>(vec3f(1.0,0.0,0.0), vec3f(0.0,1.0,0.0), vec3f(0.0,0.0,1.0));

    // Initialization
    var rayOrigin = camera.pos.xyz;
    // var rayOrigin = vec3f(-2.0,0.0,-2.0);
    var rayDirection = normalize(matrix*vec3f(uv,1/tan(FOV/2)));
    var color = vec3(0.0);

    var totalDist = 0.;
    var outNormal : vec3f;

    // var xzSwizzle : vec2;
    // rayOrigin.x *= rot2D(-m.x);
    // rayOrigin.z *= rot2D(-m.x);
    // rayDirection.x *= rot2D(-m.x);
    // rayDirection.z *= rot2D(-m.x);

    // Ray marching
    for (var i = 0; i < 500; i++){
        var position : vec3f = rayOrigin + rayDirection * totalDist; // our postion along the ray
        var normal : vec3f = getNormal(position);

        var distance = map(position);

        totalDist += distance;

        if (distance < .001) {
            outNormal = normal;
            break;
        };
        if (distance > 5000.0) {
            return vec4f(0.0, 0.0, 0.0, 1.0);
        }
    }

    // Coloring and Lighting
    let lightColor = vec3f(1.0); // the color of our light, in this case white
    let lightSource = vec3f(2.5, 2.5, -1.0);
    let diffuseStrength = max(0.0, dot(normalize(lightSource), outNormal));
    let diffuse = lightColor * diffuseStrength;

    let viewSource = normalize(rayOrigin);
    let reflectSource = normalize(reflect(-lightSource, outNormal));
    var specularStrength = max(0.0, dot(viewSource, reflectSource));
    specularStrength = pow(specularStrength, 64.0);
    let specular = specularStrength * lightColor;

    let lighting = diffuse * 0.75 + specular * 0.25;
    color = lighting;

    // Shadows
    let p = rayOrigin + rayDirection * totalDist;
    let lightDirection = normalize(lightSource);
    let distanceToLightSource = length(lightSource - p);
    rayOrigin = p * outNormal * 0.1;
    rayDirection = lightDirection;
    let shadowDistance = totalDist;
    if (shadowDistance < distanceToLightSource) {
        color = color * vec3f(0.25);
    }

    return vec4(color, 1.0);
}

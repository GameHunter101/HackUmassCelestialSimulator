struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>
}

struct Uniforms {
    iResolution: vec2<f32>;
    iTime: f32;
    iMouse: vec2<f32>;
}

[[binding(0), group(0)]] var<uniform> uniforms : Uniforms;

fn map(p : vec3) {
    return length(p) - 1;
}

@fragment
fn main(VertexInput : vec4, VertexOutput : vec4) {
    var uv = vec2((VertexInput.position * 2. - iResolution.xy) / iResolution.y);

    // Initialization
    var rayOrigin = vec3(0, 0, -3);
    var rayDirection = normalize(vec3(uv, 1));
    var color = vec3(0);

    var totalDist = 0.;

    // Ray marching
    for (var i = 0; i < 80; i++){
        var position : vec3 = rayOrigin + rayDirection * totalDist; // our postion along the ray

        var distance = map(position);

        totalDist += distance;

        color = vec3(i) / 80.;

        if (distance < .001 || totalDist > 100.) {break};
    }

    // Coloring
    //color = vec3(totalDist * .2);

    fragColor = vec4(color, 1);
}

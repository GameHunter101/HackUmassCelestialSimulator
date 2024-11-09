struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>
}

@vertex
fn main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4f(model.position, 1.0);
    return out;
}
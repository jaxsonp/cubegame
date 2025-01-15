

// global bindings
struct Camera {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: Camera;


// per mesh bindings
@group(1) @binding(0)
var<uniform> mesh_pos: vec3<f32>;


struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) tex_coord: vec2<f32>
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) tex_coord: vec2<f32>
};

@vertex
fn main(
    vert: VertexInput,
) -> VertexOutput {

    var out: VertexOutput;
    out.clip_pos = camera.view_proj * vec4<f32>(vert.pos + mesh_pos, 1.0);
    out.tex_coord = vert.tex_coord;
    return out;
}

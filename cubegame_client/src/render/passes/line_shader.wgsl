

// global bindings
struct Camera {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: Camera;

@group(1) @binding(0)
var<uniform> pos_offset: vec3<f32>;
@group(1) @binding(1)
var<uniform> color: vec3<f32>;


struct VertexInput {
    @location(0) pos: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
};

@vertex
fn vs_main(
    vert: VertexInput,
) -> VertexOutput {

    var out: VertexOutput;
    out.clip_pos = camera.view_proj * vec4<f32>(vert.pos + pos_offset, 1.0);
    return out;
}

@fragment
fn fs_main(vert: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(color, 1.0);
}
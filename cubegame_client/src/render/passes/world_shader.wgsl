

struct Camera {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: Camera;

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
fn vs_main(
    vert: VertexInput,
) -> VertexOutput {

    var out: VertexOutput;
    out.clip_pos = camera.view_proj * vec4<f32>(vert.pos + mesh_pos, 1.0);
    out.tex_coord = vert.tex_coord;
    return out;
}

@group(0) @binding(1)
var atlas_texture: texture_2d<f32>;
@group(0) @binding(2)
var atlas_sampler: sampler;

// texture altas position [x pos, y pos, x scale, y scale]
@group(1) @binding(1)
var<uniform> atlas_pos: vec4<f32>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let size = textureDimensions(atlas_texture);

    // translating texture coord to atlas
    let coord = vec2<f32>(in.tex_coord.x * atlas_pos[2] + atlas_pos[0], in.tex_coord.y * atlas_pos[3] + atlas_pos[1]);

    return textureSample(atlas_texture, atlas_sampler, coord);
}

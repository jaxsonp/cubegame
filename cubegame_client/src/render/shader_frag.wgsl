
struct Vertex {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) tex_coord: vec2<f32>
};

// texture bindings
@group(1) @binding(0)
var atlas_texture: texture_2d<f32>;
@group(1) @binding(1)
var atlas_sampler: sampler;

// texture altas position [x pos, y pos, x scale, y scale]
@group(2) @binding(1)
var<uniform> atlas_pos: vec4<f32>;

@fragment
fn main(in: Vertex) -> @location(0) vec4<f32> {
    let size = textureDimensions(atlas_texture);

    // translating texture coord to atlas
    let coord = vec2<f32>(in.tex_coord.x * atlas_pos[2] + atlas_pos[0], in.tex_coord.y * atlas_pos[3] + atlas_pos[1]);

    return textureSample(atlas_texture, atlas_sampler, coord);
}
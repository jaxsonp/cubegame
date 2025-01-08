
struct Vertex {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) tex_coord: vec2<f32>
};

// texture bindings
@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;


@fragment
fn main(in: Vertex) -> @location(0) vec4<f32> {
    let color = vec4f(in.tex_coord.r, in.tex_coord.g, 0.0, 1.0);
    return textureSample(texture, texture_sampler, in.tex_coord);
}
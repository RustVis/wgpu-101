
struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) color: vec3<f32>,
	@location(2) tex_coords: vec2<f32>,
}

struct VertexOutput {
	@builtin(position) clip_position: vec4<f32>,
	@location(0) color: vec3<f32>,
	@location(1) tex_coords: vec2<f32>,
};

// Vertex Shader
@vertex
fn vs_main(
	in: VertexInput,
) -> VertexOutput {
	var out: VertexOutput;
	out.color = in.color;
	out.tex_coords = in.tex_coords;
	out.clip_position = vec4<f32>(in.position, 1.0);
	return out;
}

// Fragment Shader
@group(0)
@binding(0)
var container_texture: texture_2d<f32>;

@group(0)
@binding(1)
var texture_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	return textureSample(container_texture, texture_sampler, in.tex_coords);
//	return textureSample(container_texture, texture_sampler, in.tex_coords) * vec4<f32>(in.color, 1.0);
}

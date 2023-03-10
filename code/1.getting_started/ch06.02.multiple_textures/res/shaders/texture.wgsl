
// Vertex Shader
struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) color: vec3<f32>,
	@location(2) tex_coords: vec2<f32>,
}

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) color: vec3<f32>,
	@location(1) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
	in: VertexInput,
) -> VertexOutput {
	var out: VertexOutput;
	out.position = vec4<f32>(in.position, 1.0);
	out.color = in.color;
	out.tex_coords = in.tex_coords;
	return out;
}

// Fragment Shader
struct FragmentInput {
	@location(0) color: vec3<f32>,
	@location(1) tex_coords: vec2<f32>,
};

@group(0)
@binding(0)
var container_texture: texture_2d<f32>;

@group(0)
@binding(1)
var container_sampler: sampler;

@group(0)
@binding(2)
var face_texture: texture_2d<f32>;

@group(0)
@binding(3)
var face_sampler: sampler;

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
	return 
		mix(textureSample(container_texture, container_sampler, in.tex_coords),
			textureSample(face_texture, face_sampler, in.tex_coords),
			0.2);
}

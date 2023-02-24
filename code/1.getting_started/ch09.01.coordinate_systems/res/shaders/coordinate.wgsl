
// Vertex Shader
struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(2) tex_coords: vec2<f32>,
}

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(1) tex_coords: vec2<f32>,
};

struct Uniforms {
	@location(0) transform: mat4x4<f32>,
}

@group(0)
@binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(
	in: VertexInput,
) -> VertexOutput {
	var out: VertexOutput;
	out.tex_coords = in.tex_coords;
	out.position = uniforms.transform * vec4<f32>(in.position, 1.0);
	return out;
}

// Fragment Shader
struct FragmentInput {
	@location(1) tex_coords: vec2<f32>,
};

@group(0)
@binding(1)
var container_texture: texture_2d<f32>;

@group(0)
@binding(2)
var container_sampler: sampler;

@group(0)
@binding(3)
var face_texture: texture_2d<f32>;

@group(0)
@binding(4)
var face_sampler: sampler;

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
	return 
		mix(textureSample(container_texture, container_sampler, in.tex_coords),
			textureSample(face_texture, face_sampler, in.tex_coords),
			0.2);
}

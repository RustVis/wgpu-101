
// Vertex Shader
struct VertexInput {
	@builtin(vertex_index) vertex_index: u32,
	@location(0) position: vec3<f32>,
	@location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) tex_coords: vec2<f32>,
	@location(1) color: vec3<f32>,
};

struct CameraUniform {
	@location(0) view_proj: mat4x4<f32>,
}

@group(0)
@binding(0)
var<uniform> camera_uniform: CameraUniform;

@vertex
fn vs_main(
	in: VertexInput,
) -> VertexOutput {
	var out: VertexOutput;
	out.position = camera_uniform.view_proj * vec4<f32>(in.position, 1.0);
	out.tex_coords = in.tex_coords;
	let delta: f32 = 1.0 / f32(in.vertex_index);
	out.color = vec3(asin(delta), acos(delta), 1.0 -  delta);
	return out;
}

// Fragment Shader
struct FragmentInput {
	@location(0) tex_coords: vec2<f32>,
	@location(1) color: vec3<f32>,
};

@group(1)
@binding(0)
var container_texture: texture_2d<f32>;

@group(1)
@binding(1)
var container_sampler: sampler;

@group(1)
@binding(2)
var face_texture: texture_2d<f32>;

@group(1)
@binding(3)
var face_sampler: sampler;

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
	return vec4(in.color, 1.0);
//	return
//		mix(textureSample(container_texture, container_sampler, in.tex_coords),
//			textureSample(face_texture, face_sampler, in.tex_coords),
//			0.2);
}

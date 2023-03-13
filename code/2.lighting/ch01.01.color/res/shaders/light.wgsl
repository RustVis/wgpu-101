
// Vertex Shader
struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) tex_coords: vec2<f32>,
};

struct CameraUniform {
	@location(0) view_proj: mat4x4<f32>,
};

@group(0)
@binding(0)
var<uniform> camera_uniform: CameraUniform;

struct Uniform {
	@location(0) model: mat4x4<f32>,
};

@vertex
fn vs_main(
	in: VertexInput,
) -> VertexOutput {
	var out: VertexOutput;
	out.position = camera_uniform.view_proj * vec4<f32>(in.position, 1.0);
	out.tex_coords = in.tex_coords;
	return out;
}

// Fragment Shader
struct FragmentInput {
	@location(0) tex_coords: vec2<f32>,
};

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
	return vec4(1.0, 1.0, 1.0, 1.0);
}

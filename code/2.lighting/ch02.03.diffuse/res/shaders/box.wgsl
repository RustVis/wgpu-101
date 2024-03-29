
// Vertex Shader
struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) normal: vec3<f32>,
	@location(2) tex_coords: vec2<f32>,
}

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) normal: vec3<f32>,
	@location(1) tex_coords: vec2<f32>,
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
	out.normal = in.normal;
	out.tex_coords = in.tex_coords;
	return out;
}

// Fragment Shader
struct FragmentInput {
	@builtin(position) position: vec4<f32>,
	@location(0) normal: vec3<f32>,
	@location(1) tex_coords: vec2<f32>,
};

struct BoxUniform {
	@location(0) box_color: vec4<f32>,
	@location(1) light_color: vec4<f32>,
	@location(2) light_pos: vec4<f32>,
	@location(3) ambient_strength: f32,
};

@group(1)
@binding(0)
var<uniform> box_uniform: BoxUniform;

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
	// ambient
	let ambient = box_uniform.ambient_strength * box_uniform.light_color;

  	// diffuse
	let norm = normalize(in.normal);
	let light_dir = normalize(box_uniform.light_pos - in.position).xyz;
	let diff = max(dot(norm, light_dir), 0.0);
	let diffuse = diff * box_uniform.light_color;

	let result = (ambient + diffuse) * box_uniform.box_color;
	return result;
}

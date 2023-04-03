
// Vertex Shader
struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) normal: vec3<f32>,
	@location(2) tex_coords: vec2<f32>,
}

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) frag_pos: vec3<f32>,
	@location(1) normal: vec3<f32>,
	@location(2) tex_coords: vec2<f32>,
	@location(3) view_pos: vec3<f32>,
};

struct CameraUniform {
	@location(0) view_proj: mat4x4<f32>,
	@location(1) view_pos: vec3<f32>,
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
	out.frag_pos = in.position;
	out.normal = in.normal;
	out.tex_coords = in.tex_coords;
	out.view_pos = camera_uniform.view_pos;
	return out;
}

// Fragment Shader
struct FragmentInput {
	@builtin(position) position: vec4<f32>,
	@location(0) frag_pos: vec3<f32>,
	@location(1) normal: vec3<f32>,
	@location(2) tex_coords: vec2<f32>,
	@location(3) view_pos: vec3<f32>,
};

struct BoxUniform {
	@location(0) box_color: vec4<f32>,
	@location(1) light_color: vec4<f32>,
	@location(2) light_pos: vec4<f32>,
	@location(3) ambient_strength: f32,
	@location(4) specular_strength: f32,
	@location(4) shininess_strength: f32,
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
	let light_dir = normalize(box_uniform.light_pos.xyz - in.frag_pos);
	let diff = max(dot(norm, light_dir), 0.0);
	let diffuse = diff * box_uniform.light_color;

	// specular
	let view_dir = normalize(in.view_pos - in.frag_pos);
	let reflect_dir = reflect(-light_dir, norm);
	let spec = pow(max(dot(view_dir, reflect_dir), 0.0), box_uniform.shininess_strength);
	let specular = box_uniform.specular_strength * spec * box_uniform.light_color;

	let result = (ambient + diffuse + specular) * box_uniform.box_color;
	return result;
}

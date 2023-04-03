
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
	out.frag_pos = in.position;
	out.normal = in.normal;
	out.tex_coords = in.tex_coords;
	return out;
}

// Fragment Shader
struct FragmentInput {
	@builtin(position) position: vec4<f32>,
	@location(0) frag_pos: vec3<f32>,
	@location(1) normal: vec3<f32>,
	@location(2) tex_coords: vec2<f32>,
};

struct Material {
	@location(0) ambient: vec3<f32>,
	@location(1) diffuse: vec3<f32>,
	@location(2) specular: vec3<f32>,
	@location(3) shininess: i32,
};

struct Light {
	@location(0) position: vec3<f32>,

	@location(1) ambient: vec3<f32>,
	@location(2) diffuse: vec3<f32>,
	@location(3) specular: vec3<f32>,
};

@group(1)
@binding(0)
var<uniform> material: Material;

@group(1)
@binding(1)
var<uniform> light: Light;

@group(2)
@binding(0)
var box_texture: texture_2d<f32>;

@group(2)
@binding(1)
var texture_sampler: sampler;

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
	let material_diffuse = textureSample(box_texture,
	                                     texture_sampler,
	                                     in.tex_coords).rgb;
	// ambient
	let ambient = light.ambient * material_diffuse;

  	// diffuse
	let norm = normalize(in.normal);
	let light_dir = normalize(light.position.xyz - in.frag_pos);
	let diff = max(dot(norm, light_dir), 0.0);
	let diffuse = light.diffuse * (diff * material_diffuse);

	// specular
	// TODO(Shaohua): Move to uniform
	let view_pos = vec3<f32>(1.0, 1.0, 1.0);
	let view_dir = normalize(view_pos - in.frag_pos);
	let reflect_dir = reflect(-light_dir, norm);
	let spec = pow(max(dot(view_dir, reflect_dir), 0.0), f32(material.shininess));
	let specular = light.specular * (spec * material.specular);

	let result = ambient + diffuse + specular;
	return vec4(result, 1.0);
}

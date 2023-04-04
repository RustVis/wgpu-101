
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

struct Material {
	@location(0) diffuse: vec3<f32>,
	@location(1) specular: vec3<f32>,
	@location(2) shininess: i32,
};

struct Light {
	@location(0) position: vec3<f32>,
	@location(1) direction: vec3<f32>,
	@location(2) cutoff: f32,

	@location(3) ambient: vec3<f32>,
	@location(4) diffuse: vec3<f32>,
	@location(5) specular: vec3<f32>,

	@location(6) constant: f32,
	@location(7) linear: f32,
	@location(8) quadratic: f32,
};

@group(1)
@binding(0)
var<uniform> material: Material;

@group(1)
@binding(1)
var<uniform> light: Light;

@group(2)
@binding(0)
var diffuse_texture: texture_2d<f32>;

@group(2)
@binding(1)
var diffuse_sampler: sampler;

@group(2)
@binding(2)
var specular_texture: texture_2d<f32>;

@group(2)
@binding(3)
var specular_sampler: sampler;

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
	let material_diffuse = textureSample(diffuse_texture,
	                                     diffuse_sampler,
	                                     in.tex_coords).rgb;
	let material_specular = textureSample(specular_texture,
	                                      specular_sampler,
	                                      in.tex_coords).rgb;

	// ambient
	let ambient = light.ambient * material_diffuse;

	let light_dir = normalize(light.position - in.frag_pos);
	let theta = dot(light_dir, normalize(-light.direction));

	if theta <= light.cutoff {
		return vec4(ambient, 1.0);
	}

  	// diffuse
	let norm = normalize(in.normal);
	let diff = max(dot(norm, light_dir), 0.0);
	let diffuse = light.diffuse * diff * material_diffuse;

	// specular
	let view_dir = normalize(in.view_pos - in.frag_pos);
	let reflect_dir = reflect(-light_dir, norm);
	let spec = pow(max(dot(view_dir, reflect_dir), 0.0), f32(material.shininess));
	let specular = light.specular * spec * material_specular;

	// attenuation
	let distance = length(light.position - in.frag_pos);
	let attenuation = 1.0 / (light.constant + light.linear * distance +
		light.quadratic * distance * distance);

	let result = (ambient + diffuse + specular) * attenuation;
	return vec4(result, 1.0);
}

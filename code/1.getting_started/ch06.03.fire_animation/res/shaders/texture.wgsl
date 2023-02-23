
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
	out.color = in.color;
	out.tex_coords = in.tex_coords;
	out.position = vec4<f32>(in.position, 1.0);
	return out;
}

// Fragment Shader
struct FragmentInput {
	@location(0) color: vec3<f32>,
	@location(1) tex_coords: vec2<f32>,
}

@group(0)
@binding(0)
var texture_array: binding_array<texture_2d<f32>>;

@group(0)
@binding(1)
var sampler_array: binding_array<sampler>;

struct Uniforms {
    index: u32,
}

@group(0)
@binding(2)
var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
	return textureSample(texture_array[uniforms.index],
			sampler_array[uniforms.index],
			in.tex_coords);
}

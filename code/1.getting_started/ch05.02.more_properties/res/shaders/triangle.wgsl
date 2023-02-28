
// Vertex Shader
struct VertexInput {
	@builtin(vertex_index) vertex_index: u32,
	@location(0) position: vec3<f32>,
	@location(1) color: vec3<f32>,
}

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) vertex_index: u32,
	@location(1) color: vec3<f32>,
};

struct Uniforms {
	@location(0) colors: array<vec3<f32>, 3>,
};

@group(0)
@binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(
	in: VertexInput,
) -> VertexOutput {
	var out: VertexOutput;
	out.position = vec4<f32>(in.position, 1.0);
	out.vertex_index = in.vertex_index;
	//out.color = in.color;
	out.color = uniforms.colors[in.vertex_index];
	return out;
}

// Fragment Shader
struct FragmentInput {
	@location(0) vertex_index: u32,
	@location(1) color: vec3<f32>,
};

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
	return vec4(in.color, 1.0);
	//var color = uniforms.colors[in.vertex_index];
	//return vec4(color, 1.0);
}


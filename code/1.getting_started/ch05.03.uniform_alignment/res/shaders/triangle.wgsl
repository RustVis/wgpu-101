
// Vertex Shader
struct VertexInput {
	@builtin(vertex_index) vertex_index: u32,
	@location(0) position: vec3<f32>,
	@location(1) color: vec3<f32>,
}

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) vertex_index: u32,
	@location(1) color: vec4<f32>,
};

struct Uniforms {
	@location(0) vec3_val0: vec3<f32>,
	@location(1) vec3_val: vec3<f32>,
	@location(2) f32_val: f32,
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
	if uniforms.vec3_val.x == 1.0 {
		out.color = vec4(uniforms.vec3_val, uniforms.f32_val);
	} else {
		out.color = vec4(1.0);
	}
	return out;
}

// Fragment Shader
struct FragmentInput {
	@location(0) vertex_index: u32,
	@location(1) color: vec4<f32>,
};

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
	return in.color;
}


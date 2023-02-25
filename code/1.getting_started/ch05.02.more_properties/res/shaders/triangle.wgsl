
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

@vertex
fn vs_main(
	in: VertexInput,
) -> VertexOutput {
	var out: VertexOutput;
	out.position = vec4<f32>(in.position, 1.0);
	out.vertex_index = in.vertex_index;
	out.color = in.color;
	return out;
}

// Fragment Shader
struct FragmentInput {
	@location(0) vertex_index: u32,
	@location(1) color: vec3<f32>,
};

struct Uniforms {
	@location(0) color0: vec3<f32>,
	@location(1) color1: vec3<f32>,
	@location(2) color2: vec3<f32>,
};

@group(0)
@binding(0)
var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
	var color = vec3<f32>(1.0, 1.0, 1.0);
	color = uniforms.color0;
//	if (in.vertex_index == 0) {
//		color = uniforms.color0;
//	} else if (in.vertex_index == 1) {
//		color = uniforms.color1;
//	} else {
//		color = uniforms.color2;
//	}
	return vec4<f32>(color, 1.0);
}


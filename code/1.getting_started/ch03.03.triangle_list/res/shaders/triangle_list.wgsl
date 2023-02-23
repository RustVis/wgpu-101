
// Vertex Shader
struct VertexInput {
	@builtin(vertex_index) vertex_index: u32,
};

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) color: vec4<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
	var pos = array<vec2<f32>, 6> (
		// A
		vec2<f32>(-0.5, 0.0),
		// B
		vec2<f32>(0.0, 0.0),
		// C
		vec2<f32>(-0.25, 0.25),
		// D
		vec2<f32>(0.25, 0.25),
		// E
		vec2<f32>(0.0, 0.5),
		// F
		vec2<f32>(0.50, 0.75),
	);
	var color : array<vec3<f32>, 6> = array<vec3<f32>, 6>(
		vec3<f32>(1.0, 0.0, 0.0),
		vec3<f32>(0.0, 1.0, 0.0),
		vec3<f32>(0.0, 0.0, 1.0),
		vec3<f32>(1.0, 0.0, 0.0),
		vec3<f32>(0.0, 1.0, 0.0),
		vec3<f32>(0.0, 0.0, 1.0),
	);
	var output: VertexOutput;
	output.position = vec4<f32>(pos[in.vertex_index], 0.0, 1.0);
	output.color = vec4<f32>(color[in.vertex_index], 1.0);

	return output;
}

// Fragment Shader
struct FragmentInput {
	@location(0) color: vec4<f32>,
};

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
	return in.color;
}

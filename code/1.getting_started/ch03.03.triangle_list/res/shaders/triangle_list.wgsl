
// Vertex Shader
struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) v_color: vec4<f32>,
};

@vertex
fn vs_main(
	@builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
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
	output.position = vec4<f32>(pos[in_vertex_index], 0.0, 1.0);
	output.v_color = vec4<f32>(color[in_vertex_index], 1.0);

	return output;
}

// Fragment Shader
@fragment
fn fs_main(@location(0) v_color: vec4<f32>) -> @location(0) vec4<f32> {
	return v_color;
}

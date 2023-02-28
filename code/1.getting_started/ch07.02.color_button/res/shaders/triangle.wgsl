
// Vertex Shader
struct VertexInput {
	@location(0) position: vec3<f32>,
}

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(
	model: VertexInput,
) -> VertexOutput {
	var out: VertexOutput;
	out.position = vec4<f32>(model.position, 1.0);
	return out;
}

// Fragment Shader
struct Uniforms {
	@location(0) color: vec3<f32>,
};

@group(0)
@binding(0)
var<uniform> uniforms: Uniforms;

@fragment
fn fs_main() -> @location(0) vec4<f32> {
	return vec4<f32>(uniforms.color, 1.0);
}


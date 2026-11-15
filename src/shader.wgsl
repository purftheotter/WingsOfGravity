// Vertex Shader
struct CameraUniform {
  view_proj: mat4x4<f32>,
};

@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) tex_coords: vec2<f32>,
  @location(1) color: vec4<f32>,
};

struct InstanceInput {
  @location(2) position: vec2<f32>,
  @location(3) rotation: f32,
  @location(4) size: vec2<f32>,
  @location(5) uv_rect: vec4<f32>,
  @location(6) color: vec4<f32>
};

@vertex
fn vs_main(
  model: VertexInput,
  instance: InstanceInput,
) -> VertexOutput {
  let c = cos(instance.rotation);
  let s = sin(instance.rotation);

  let local = model.position.xy * instance.size;
  let rotated = vec2<f32>(
  local.x * c - local.y * s,
  local.x * s + local.y * c,
  );
  let world = rotated + instance.position;

  var out: VertexOutput;
  out.tex_coords = instance.uv_rect.xy + model.tex_coords * instance.uv_rect.zw;
  out.clip_position = camera.view_proj * vec4<f32>(world, model.position.z, 1.0);
  out.color = instance.color;
  return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords) * in.color;
}


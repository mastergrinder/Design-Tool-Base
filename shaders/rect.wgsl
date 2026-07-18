struct ViewUniformsRaw {
    c0: vec4<f32>,
    c1: vec4<f32>,
    c2: vec4<f32>,
    viewport: vec4<f32>,
}

@group(0) @binding(0) var<uniform> view: ViewUniformsRaw;

struct VertexInput {
    @location(0) pos: vec2<f32>,
    @location(1) rect: vec4<f32>,
    @location(2) color: vec4<f32>,
    @location(3) params: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) local: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) radius: f32,
    @location(4) selected: f32,
}

fn apply_view(p: vec2<f32>) -> vec4<f32> {
    let m = mat3x3<f32>(view.c0.xyz, view.c1.xyz, view.c2.xyz);
    let clip = m * vec3<f32>(p, 1.0);
    return vec4<f32>(clip.xy, 0.0, 1.0);
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let origin = in.rect.xy;
    let size = in.rect.zw;
    let world = origin + in.pos * size;
    out.clip_pos = apply_view(world);
    out.color = vec4<f32>(in.color.rgb, in.color.a * in.params.y);
    out.local = in.pos * size;
    out.size = size;
    out.radius = in.params.x;
    out.selected = in.params.z;
    return out;
}

fn sd_rounded_box(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + vec2<f32>(r);
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let half = in.size * 0.5;
    let p = in.local - half;
    let r = min(in.radius, min(half.x, half.y));
    let d = sd_rounded_box(p, half, r);
    let aa = max(fwidth(d), 0.001);
    let alpha = 1.0 - smoothstep(-aa, aa, d);
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}

@fragment
fn fs_outline(in: VertexOutput) -> @location(0) vec4<f32> {
    if (in.selected < 0.5) {
        discard;
    }
    let half = in.size * 0.5;
    let p = in.local - half;
    let r = min(in.radius, min(half.x, half.y));
    let d = sd_rounded_box(p, half, r);
    let thickness = 1.5 / max(view.viewport.z, 0.001);
    let aa = max(fwidth(d), 0.001);
    let outer = 1.0 - smoothstep(-aa, aa, d);
    let inner = 1.0 - smoothstep(-aa, aa, d + thickness);
    let edge = clamp(outer - inner, 0.0, 1.0);
    return vec4<f32>(0.15, 0.45, 0.95, edge);
}

//! Built-in procedural shader catalog (WGSL fragment bodies).

#[derive(Clone, Copy, Debug)]
pub struct ShaderMeta {
    pub id: u32,
    pub name: &'static str,
    pub category: &'static str,
    /// CSS-ish preview gradient for the gallery card.
    pub preview: &'static str,
}

pub struct ShaderDef {
    pub meta: ShaderMeta,
    /// Fragment shader entry that writes @location(0) vec4 — receives VertexOutput.
    pub fragment_wgsl: &'static str,
}

const COMMON_VS: &str = r#"
struct ViewRaw {
    c0: vec4<f32>,
    c1: vec4<f32>,
    c2: vec4<f32>,
    viewport: vec4<f32>,
}

struct DrawUniforms {
    rect: vec4<f32>,
    time_opacity: vec4<f32>,
    mouse: vec4<f32>,
}

@group(0) @binding(0) var<uniform> view: ViewRaw;
@group(0) @binding(1) var<uniform> draw: DrawUniforms;

struct VertexInput {
    @location(0) pos: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) local: vec2<f32>,
}

fn apply_view(p: vec2<f32>) -> vec4<f32> {
    let m = mat3x3<f32>(view.c0.xyz, view.c1.xyz, view.c2.xyz);
    let clip = m * vec3<f32>(p, 1.0);
    return vec4<f32>(clip.xy, 0.0, 1.0);
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let origin = draw.rect.xy;
    let size = draw.rect.zw;
    let world = origin + in.pos * size;
    out.clip_pos = apply_view(world);
    out.uv = in.pos;
    out.local = in.pos * size;
    return out;
}
"#;

fn wrap(fragment_body: &str) -> String {
    format!(
        "{COMMON_VS}

struct FragIn {{
    @location(0) uv: vec2<f32>,
    @location(1) local: vec2<f32>,
}}

fn aspect_uv(uv: vec2<f32>) -> vec2<f32> {{
    let size = draw.rect.zw;
    var p = uv * 2.0 - 1.0;
    p.x *= size.x / max(size.y, 1.0);
    return p;
}}

{fragment_body}
"
    )
}

macro_rules! shader {
    ($id:expr, $name:expr, $cat:expr, $preview:expr, $body:expr) => {
        ShaderDef {
            meta: ShaderMeta {
                id: $id,
                name: $name,
                category: $cat,
                preview: $preview,
            },
            fragment_wgsl: $body,
        }
    };
}

pub fn catalog() -> Vec<ShaderDef> {
    vec![
        shader!(
            0,
            "Plasma",
            "Abstract",
            "linear-gradient(135deg,#ff6b9d,#c44dff,#4d9fff)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let p = aspect_uv(in.uv);
    let v = sin(p.x * 3.0 + t) + sin(p.y * 3.0 + t * 1.3) + sin((p.x + p.y) * 2.0 + t * 0.7);
    let c = vec3<f32>(0.5 + 0.5 * sin(v + vec3<f32>(0.0, 2.0, 4.0)));
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            1,
            "Gradient Wave",
            "Abstract",
            "linear-gradient(180deg,#1a1a2e,#e94560)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let y = in.uv.y + 0.08 * sin(in.uv.x * 12.0 + t * 2.0);
    let c = mix(vec3<f32>(0.08, 0.09, 0.18), vec3<f32>(0.91, 0.27, 0.38), smoothstep(0.2, 0.8, y));
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            2,
            "Checker Pulse",
            "Pattern",
            "repeating-conic-gradient(#222 0% 25%, #eee 0% 50%)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let n = 8.0 + 2.0 * sin(t);
    let cell = floor(in.uv * n);
    let checker = (cell.x + cell.y) % 2.0;
    let c = mix(vec3<f32>(0.12), vec3<f32>(0.92), checker);
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            3,
            "Soft Noise",
            "Organic",
            "linear-gradient(135deg,#2d3436,#636e72,#b2bec3)",
            r#"
fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}
fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(hash(i), hash(i + vec2<f32>(1.0, 0.0)), u.x),
        mix(hash(i + vec2<f32>(0.0, 1.0)), hash(i + vec2<f32>(1.0, 1.0)), u.x),
        u.y
    );
}
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let p = in.uv * 6.0 + vec2<f32>(t * 0.2, t * 0.15);
    let n = noise(p) * 0.5 + noise(p * 2.0) * 0.25 + noise(p * 4.0) * 0.125;
    let c = vec3<f32>(0.25 + n * 0.55);
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            4,
            "Ripple",
            "Organic",
            "radial-gradient(circle,#74b9ff,#0984e3,#2d3436)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let p = aspect_uv(in.uv);
    let d = length(p);
    let w = sin(d * 18.0 - t * 4.0) * 0.5 + 0.5;
    let c = mix(vec3<f32>(0.05, 0.12, 0.22), vec3<f32>(0.45, 0.75, 1.0), w);
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            5,
            "Spiral",
            "Abstract",
            "conic-gradient(from 180deg,#fd79a8,#fdcb6e,#00cec9)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let p = aspect_uv(in.uv);
    let a = atan2(p.y, p.x);
    let r = length(p);
    let v = sin(a * 5.0 + r * 12.0 - t * 3.0) * 0.5 + 0.5;
    let c = vec3<f32>(0.5 + 0.5 * sin(v * 6.28 + vec3<f32>(0.0, 2.1, 4.2)));
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            6,
            "Rainbow Sweep",
            "Color",
            "linear-gradient(90deg,red,orange,yellow,green,cyan,blue,violet)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let h = fract(in.uv.x + t * 0.15);
    let c = 0.5 + 0.5 * cos(6.28318 * (h + vec3<f32>(0.0, 0.33, 0.67)));
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            7,
            "Metaballs",
            "Organic",
            "radial-gradient(circle at 30% 40%,#a29bfe,#6c5ce7,#2d3436)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let p = aspect_uv(in.uv);
    var f = 0.0;
    for (var i = 0; i < 5; i++) {
        let fi = f32(i);
        let c = vec2<f32>(sin(t * (0.7 + fi * 0.2) + fi), cos(t * (0.9 + fi * 0.15) + fi * 1.7)) * 0.55;
        f += 0.12 / (length(p - c) + 0.02);
    }
    let v = smoothstep(0.8, 1.2, f);
    let col = mix(vec3<f32>(0.08, 0.08, 0.12), vec3<f32>(0.55, 0.45, 0.95), v);
    return vec4<f32>(col, draw.time_opacity.y);
}
"#
        ),
        shader!(
            8,
            "Starfield",
            "Space",
            "radial-gradient(circle,#2c3e50,#000)",
            r#"
fn hash21(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let uv = in.uv * 40.0;
    let id = floor(uv);
    let f = fract(uv) - 0.5;
    let n = hash21(id);
    let twinkle = 0.5 + 0.5 * sin(t * (2.0 + n * 5.0) + n * 20.0);
    let d = length(f);
    let star = smoothstep(0.08, 0.0, d) * step(0.92, n) * twinkle;
    let bg = vec3<f32>(0.02, 0.03, 0.06);
    return vec4<f32>(bg + vec3<f32>(star), draw.time_opacity.y);
}
"#
        ),
        shader!(
            9,
            "Fire",
            "Organic",
            "linear-gradient(180deg,#fff,#fdcb6e,#e17055,#2d3436)",
            r#"
fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}
fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + vec2<f32>(1.0, 0.0)), u.x), mix(hash(i + vec2<f32>(0.0, 1.0)), hash(i + vec2<f32>(1.0, 1.0)), u.x), u.y);
}
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    var p = in.uv;
    p.y = 1.0 - p.y;
    let n = noise(vec2<f32>(p.x * 4.0, p.y * 3.0 - t * 2.0));
    let flame = smoothstep(0.15, 0.7, p.y + n * 0.35 - 0.15);
    let c = mix(vec3<f32>(0.05, 0.02, 0.0), vec3<f32>(1.0, 0.45, 0.05), flame);
    c = mix(c, vec3<f32>(1.0, 0.9, 0.4), smoothstep(0.7, 1.0, flame));
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            10,
            "Water",
            "Organic",
            "linear-gradient(135deg,#0984e3,#74b9ff,#dff9fb)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let p = in.uv;
    let w1 = sin(p.x * 20.0 + t * 2.0) * 0.02;
    let w2 = sin(p.y * 16.0 - t * 1.5) * 0.02;
    let q = p + vec2<f32>(w1, w2);
    let c = mix(vec3<f32>(0.05, 0.25, 0.45), vec3<f32>(0.5, 0.8, 0.95), q.y + 0.1 * sin(q.x * 10.0 + t));
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            11,
            "Aurora",
            "Space",
            "linear-gradient(180deg,#000,#00b894,#6c5ce7,#000)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let p = aspect_uv(in.uv);
    let band = sin(p.x * 3.0 + t) * 0.3 + sin(p.x * 7.0 - t * 1.5) * 0.15;
    let y = p.y - band;
    let g = exp(-y * y * 8.0);
    let c = vec3<f32>(0.1, 0.7, 0.45) * g + vec3<f32>(0.4, 0.2, 0.8) * exp(-(y - 0.2) * (y - 0.2) * 12.0);
    let bg = vec3<f32>(0.02, 0.02, 0.05);
    return vec4<f32>(bg + c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            12,
            "Hex Grid",
            "Pattern",
            "linear-gradient(135deg,#2d3436,#00cec9)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    var p = in.uv * vec2<f32>(12.0, 10.0);
    let r = vec2<f32>(1.0, 1.732);
    let h = r * 0.5;
    let a = (p % r) - h;
    let b = ((p - h) % r) - h;
    let gv = select(b, a, length(a) < length(b));
    let d = max(abs(gv.x) * 0.866 + abs(gv.y) * 0.5, abs(gv.y)) / 0.5;
    let edge = smoothstep(0.92, 1.0, d);
    let pulse = 0.5 + 0.5 * sin(t * 2.0 + in.uv.x * 6.0);
    let c = mix(vec3<f32>(0.08, 0.12, 0.14), vec3<f32>(0.0, 0.8, 0.75) * pulse, edge);
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            13,
            "Tunnel",
            "Abstract",
            "radial-gradient(circle,#fd79a8,#2d3436)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let p = aspect_uv(in.uv);
    let a = atan2(p.y, p.x);
    let r = length(p);
    let uv = vec2<f32>(a / 3.14159, 0.5 / (r + 0.05) + t * 0.4);
    let stripes = step(0.5, fract(uv.x * 6.0 + uv.y * 4.0));
    let c = mix(vec3<f32>(0.1, 0.05, 0.12), vec3<f32>(0.95, 0.4, 0.55), stripes);
    c *= smoothstep(1.4, 0.1, r);
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            14,
            "Dot Matrix",
            "Pattern",
            "radial-gradient(circle at 20% 20%,#fff,#636e72)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let grid = in.uv * 24.0;
    let f = fract(grid) - 0.5;
    let wave = 0.5 + 0.5 * sin(t * 3.0 + floor(grid.x) * 0.4 + floor(grid.y) * 0.3);
    let d = length(f);
    let dot = smoothstep(0.35 * wave, 0.1 * wave, d);
    let c = mix(vec3<f32>(0.15), vec3<f32>(0.95), dot);
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            15,
            "Waveform",
            "Audio",
            "linear-gradient(180deg,#000,#00b894,#000)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let x = in.uv.x;
    let amp = 0.15 * sin(x * 40.0 + t * 4.0) + 0.08 * sin(x * 90.0 - t * 6.0) + 0.05 * sin(x * 20.0 + t);
    let y = abs(in.uv.y - 0.5);
    let line = smoothstep(0.02, 0.0, y - abs(amp));
    let glow = exp(-y * 20.0) * 0.3;
    let c = vec3<f32>(0.0, 0.7, 0.55) * (line + glow);
    return vec4<f32>(c + vec3<f32>(0.02), draw.time_opacity.y);
}
"#
        ),
        shader!(
            16,
            "Kaleidoscope",
            "Abstract",
            "conic-gradient(#e17055,#fdcb6e,#00b894,#6c5ce7)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    var p = aspect_uv(in.uv);
    let a = atan2(p.y, p.x);
    let r = length(p);
    let seg = 6.28318 / 8.0;
    var ang = abs(a - floor(a / seg + 0.5) * seg);
    let q = vec2<f32>(cos(ang), sin(ang)) * r;
    let v = sin(q.x * 10.0 + t) * sin(q.y * 10.0 - t);
    let c = 0.5 + 0.5 * cos(v * 3.0 + t + vec3<f32>(0.0, 2.0, 4.0));
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            17,
            "Brick",
            "Pattern",
            "linear-gradient(180deg,#dfe6e9,#b2bec3,#636e72)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let scale = vec2<f32>(8.0, 12.0);
    var uv = in.uv * scale;
    let row = floor(uv.y);
    if ((row % 2.0) > 0.5) {
        uv.x += 0.5;
    }
    let f = fract(uv);
    let mortar = step(0.92, f.x) + step(0.88, f.y);
    let brick = vec3<f32>(0.65, 0.28, 0.22) * (0.85 + 0.15 * fract(sin(dot(floor(uv), vec2<f32>(12.1, 4.7))) * 43758.5));
    let c = mix(brick, vec3<f32>(0.75), clamp(mortar, 0.0, 1.0));
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            18,
            "Soft Glow",
            "Light",
            "radial-gradient(circle,#ffeaa7,#fdcb6e,#2d3436)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let p = aspect_uv(in.uv);
    let d = length(p - vec2<f32>(sin(t) * 0.2, cos(t * 0.7) * 0.15));
    let g = exp(-d * d * 4.0);
    let c = vec3<f32>(1.0, 0.85, 0.4) * g + vec3<f32>(0.05, 0.06, 0.1);
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            19,
            "Voronoi",
            "Organic",
            "linear-gradient(135deg,#fd79a8,#a29bfe,#74b9ff)",
            r#"
fn hash2(p: vec2<f32>) -> vec2<f32> {
    let q = vec2<f32>(dot(p, vec2<f32>(127.1, 311.7)), dot(p, vec2<f32>(269.5, 183.3)));
    return fract(sin(q) * 43758.5453);
}
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let p = in.uv * 6.0;
    let n = floor(p);
    let f = fract(p);
    var md = 8.0;
    var mr = vec2<f32>(0.0);
    for (var j = -1; j <= 1; j++) {
        for (var i = -1; i <= 1; i++) {
            let g = vec2<f32>(f32(i), f32(j));
            let o = hash2(n + g);
            let r = g + 0.5 + 0.5 * sin(t + 6.2831 * o) - f;
            let d = dot(r, r);
            if (d < md) { md = d; mr = o; }
        }
    }
    let c = 0.5 + 0.5 * cos(6.2831 * mr.x + vec3<f32>(0.0, 1.0, 2.0));
    return vec4<f32>(c * (0.6 + 0.4 * sqrt(md)), draw.time_opacity.y);
}
"#
        ),
        shader!(
            20,
            "Scanlines",
            "Retro",
            "repeating-linear-gradient(#000,#111 2px,#222 4px)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let scan = step(0.5, fract(in.uv.y * 80.0));
    let sweep = smoothstep(0.0, 0.05, abs(fract(in.uv.y - t * 0.2) - 0.5));
    var c = vec3<f32>(0.1, 0.9, 0.3) * (0.3 + 0.7 * scan);
    c += vec3<f32>(0.2) * (1.0 - sweep) * 0.4;
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            21,
            "Neon Grid",
            "Retro",
            "linear-gradient(#0a0a12,#ff00aa)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    var p = in.uv;
    p.y = 1.0 - p.y;
    let perspective = 0.5 / (p.y + 0.15);
    let gx = abs(fract((p.x - 0.5) * perspective * 8.0 + t * 0.5) - 0.5);
    let gy = abs(fract(perspective * 4.0 - t) - 0.5);
    let grid = smoothstep(0.05, 0.0, min(gx, gy));
    let fade = smoothstep(0.0, 0.4, p.y);
    let c = vec3<f32>(1.0, 0.1, 0.6) * grid * fade + vec3<f32>(0.04, 0.02, 0.08);
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            22,
            "Ink Blot",
            "Organic",
            "radial-gradient(circle,#2d3436,#636e72,#dfe6e9)",
            r#"
fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}
fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + vec2<f32>(1.0, 0.0)), u.x), mix(hash(i + vec2<f32>(0.0, 1.0)), hash(i + vec2<f32>(1.0, 1.0)), u.x), u.y);
}
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let p = aspect_uv(in.uv);
    let n = noise(p * 3.0 + t * 0.1) * 0.4;
    let d = length(p) + n;
    let blot = 1.0 - smoothstep(0.35, 0.55, d);
    let c = mix(vec3<f32>(0.9), vec3<f32>(0.08), blot);
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            23,
            "Sunset",
            "Color",
            "linear-gradient(180deg,#fdcb6e,#e17055,#6c5ce7,#2d3436)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let y = in.uv.y;
    let sun_p = vec2<f32>(0.5 + 0.05 * sin(t * 0.3), 0.55);
    let sun = exp(-length((in.uv - sun_p) * vec2<f32>(1.0, 1.4)) * 12.0);
    var c = mix(vec3<f32>(0.15, 0.1, 0.3), vec3<f32>(0.95, 0.45, 0.25), smoothstep(0.3, 0.7, 1.0 - y));
    c = mix(c, vec3<f32>(1.0, 0.85, 0.4), sun);
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            24,
            "Circuit",
            "Tech",
            "linear-gradient(135deg,#00cec9,#0984e3,#2d3436)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let g = in.uv * 20.0;
    let f = fract(g);
    let line = step(0.92, max(f.x, f.y));
    let pulse = 0.5 + 0.5 * sin(t * 4.0 + floor(g.x) + floor(g.y));
    let node = smoothstep(0.15, 0.05, length(f - 0.5)) * step(0.7, fract(sin(dot(floor(g), vec2<f32>(12.1, 4.1))) * 99.0));
    let c = vec3<f32>(0.05, 0.1, 0.12) + vec3<f32>(0.0, 0.8, 0.75) * (line * 0.5 * pulse + node);
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            25,
            "Marble",
            "Organic",
            "linear-gradient(135deg,#fff,#dfe6e9,#b2bec3)",
            r#"
fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}
fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + vec2<f32>(1.0, 0.0)), u.x), mix(hash(i + vec2<f32>(0.0, 1.0)), hash(i + vec2<f32>(1.0, 1.0)), u.x), u.y);
}
fn fbm(p: vec2<f32>) -> f32 {
    var v = 0.0;
    var a = 0.5;
    var q = p;
    for (var i = 0; i < 5; i++) {
        v += a * noise(q);
        q *= 2.0;
        a *= 0.5;
    }
    return v;
}
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x * 0.1;
    let p = in.uv * 3.0 + t;
    let n = fbm(p);
    let vein = smoothstep(0.45, 0.55, abs(sin(p.x * 2.0 + n * 4.0)));
    let c = mix(vec3<f32>(0.92, 0.9, 0.88), vec3<f32>(0.35, 0.4, 0.45), vein);
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            26,
            "Polar Rings",
            "Abstract",
            "radial-gradient(circle,#fff,#6c5ce7,#2d3436)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let p = aspect_uv(in.uv);
    let r = length(p);
    let rings = sin(r * 30.0 - t * 3.0) * 0.5 + 0.5;
    let c = mix(vec3<f32>(0.1, 0.08, 0.2), vec3<f32>(0.7, 0.55, 1.0), rings) * smoothstep(1.2, 0.2, r);
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
        shader!(
            27,
            "Chrome",
            "Light",
            "linear-gradient(135deg,#fff,#b2bec3,#636e72,#dfe6e9)",
            r#"
@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {
    let t = draw.time_opacity.x;
    let p = aspect_uv(in.uv);
    let n = normalize(vec3<f32>(p, 0.8));
    let l = normalize(vec3<f32>(sin(t), cos(t * 0.7), 1.0));
    let spec = pow(max(dot(n, l), 0.0), 32.0);
    let base = 0.4 + 0.3 * n.y;
    let c = vec3<f32>(base) + vec3<f32>(spec);
    return vec4<f32>(c, draw.time_opacity.y);
}
"#
        ),
    ]
}

pub fn compiled_sources() -> Vec<(ShaderMeta, String)> {
    catalog()
        .into_iter()
        .map(|s| (s.meta, wrap(s.fragment_wgsl)))
        .collect()
}

pub fn meta_list() -> Vec<ShaderMeta> {
    catalog().into_iter().map(|s| s.meta).collect()
}

pub fn outline_source() -> String {
    format!(
        "{COMMON_VS}

struct FragIn {{
    @location(0) uv: vec2<f32>,
    @location(1) local: vec2<f32>,
}}

@fragment
fn fs_main(in: FragIn) -> @location(0) vec4<f32> {{
    let size = draw.rect.zw;
    let half = size * 0.5;
    let p = in.local - half;
    let d = max(abs(p.x) - half.x, abs(p.y) - half.y);
    let zoom = max(view.viewport.z, 0.001);
    let t_blue = 1.7 / zoom;
    let t_white = 3.1 / zoom;
    let aa = max(fwidth(d), 0.001);
    let blue = smoothstep(-aa, aa, d) * (1.0 - smoothstep(-aa, aa, d - t_blue));
    let white = smoothstep(-aa, aa, d - t_blue) * (1.0 - smoothstep(-aa, aa, d - t_white));
    var out_c = vec4<f32>(1.0, 1.0, 1.0, white * 0.88);
    out_c = out_c * (1.0 - blue) + vec4<f32>(0.25, 0.48, 1.0, 0.96) * blue;
    return out_c;
}}
"
    )
}

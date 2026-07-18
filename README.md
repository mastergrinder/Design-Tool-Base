# WebGPU Design Canvas

A minimal, high-performance visual design canvas built on a retained-mode graphics engine.

The UI is intentionally quiet — light gray chrome, no grid, no decoration. The canvas is the product. Underneath is a Rust → WebAssembly → WebGPU engine designed to stay responsive as scene complexity grows.

---
<img width="1911" height="941" alt="image" src="https://github.com/user-attachments/assets/3aa2cc3a-6ef6-48f1-b5e6-3fb06faf2f61" />

## Features

- **WebGPU rendering** via `wgpu` inside WASM (not Canvas 2D / SVG / per-layer DOM)
- **Retained-mode scene** with structure-of-arrays storage
- **Rectangles & frames** — create, select, drag, edit properties
- **28 built-in animated shaders** — click to add from the Properties panel when nothing is selected
- **Camera** — pan and cursor-centered zoom
- **Minimal editor UI** — layers, properties, toolbar; React never owns scene pixels

---



## Architecture

```text
┌─────────────────────────────────────────┐
│  apps/editor  (React + TypeScript)      │
│  Toolbar · Layers · Properties · Input  │
└──────────────────┬──────────────────────┘
                   │ FrameInput / commands
                   ▼
┌─────────────────────────────────────────┐
│  engine/wasm  (Rust → WebAssembly)      │
│  Scene · Camera · Selection · Commands  │
│  Renderer (wgpu) · Shader library       │
└──────────────────┬──────────────────────┘
                   ▼
              WebGPU (GPU)
```


| Layer           | Responsibility                       |
| --------------- | ------------------------------------ |
| **Editor UI**   | DOM chrome, input collection, panels |
| **WASM engine** | Document, selection, transforms, GPU |
| **WebGPU**      | Draw calls, shaders, buffers         |


The browser is the presentation layer. Scene state and rendering live in Rust so the app does not shuttle large scene graphs through JavaScript every frame.

---



## Requirements


| Tool                                                         | Notes                                      |
| ------------------------------------------------------------ | ------------------------------------------ |
| [Rust](https://rustup.rs/)                                   | Stable toolchain                           |
| `wasm32-unknown-unknown`                                     | `rustup target add wasm32-unknown-unknown` |
| [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) | Builds the engine to WASM                  |
| [Node.js](https://nodejs.org/) 20+                           | Runtime for the editor                     |
| [pnpm](https://pnpm.io/) 11+                                 | Package manager                            |
| Chrome or Edge                                               | WebGPU required                            |


Optional: [wasm-opt](https://github.com/WebAssembly/binaryen) (used by wasm-pack when available).

---



## Quick start

```bash
# 1. Clone
git clone https://github.com/mastergrinder/Design-Tool-Base.git
cd Design-Tool-Base

# 2. Install JS dependencies
pnpm install

# 3. Allow native build scripts (pnpm 11)
pnpm approve-builds esbuild "@swc/core"

# 4. Ensure the WASM target is installed
rustup target add wasm32-unknown-unknown

# 5. Build the engine and start the editor
pnpm dev
```

Open **[http://localhost:5173](http://localhost:5173)**

> First run compiles the Rust engine with `wasm-pack` (can take a minute). Later runs are much faster.

---



## Scripts


| Command            | Description                                         |
| ------------------ | --------------------------------------------------- |
| `pnpm dev`         | Build WASM, then start Vite dev server              |
| `pnpm build:wasm`  | Build only the Rust/WASM engine                     |
| `pnpm build`       | Production build (WASM + editor)                    |
| `pnpm preview`     | Preview the production editor build                 |
| `pnpm test:engine` | Run Rust unit tests (core, scene, selection, input) |


---



## Controls


| Action        | Input                                           |
| ------------- | ----------------------------------------------- |
| Pan           | Middle-mouse drag, or **Space** + left drag     |
| Zoom          | **Ctrl/Cmd** + mouse wheel (centered on cursor) |
| Select        | Left click                                      |
| Multi-select  | **Shift** + click                               |
| Move          | Drag selected layer(s)                          |
| Deselect      | Click empty canvas, or **Escape**               |
| Add rectangle | Toolbar, or **Ctrl/Cmd+R**                      |
| Add frame     | Toolbar, or **Ctrl/Cmd+F**                      |
| Add shader    | Properties panel (when nothing is selected)     |


---



## Repository layout

```text
apps/editor/           React + Vite editor shell
engine/
  core/                Math, camera, coordinates, dirty flags
  scene/               SoA scene graph (rects, frames, shaders)
  renderer/            wgpu device, rect + shader pipelines
  input/               Normalized pointer / keyboard input
  selection/           Hit testing
  wasm/                wasm-bindgen API surface
packages/protocol/     Shared TypeScript types (UI ↔ engine)
shaders/               Reference WGSL (rect pipeline)
docs/                  Architecture notes
benchmarks/            Reserved for scale benchmarks
native/cpp/            Reserved for future native hot paths
```

---



## Design principles

1. **The engine owns the scene** — React must not store per-node render state.
2. **GPU-first** — one canvas, batched / instanced draws, WGSL materials.
3. **Dirty updates** — avoid rebuilding the world for a one-pixel move.
4. **Scale-ready storage** — SoA layouts and APIs that can grow to 10k+ layers.
5. **Minimal UI** — no dashboard chrome; the canvas stays primary.

See [docs/architecture.md](docs/architecture.md) for frame-loop and ownership details.

---



## Browser support

WebGPU is required. Use a recent **Chrome** or **Edge** build. If WebGPU is unavailable, the editor shows an error on the canvas instead of crashing the page.

---



## Quick Note:  
- The User Interface was rushed and this is meant to be a base for everyone looking to build a design tool with a very powerful Canvas.  
  
Feel free to open a PR and contribute I will review everything very fast!

---



## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

---



## License

[MIT](LICENSE)

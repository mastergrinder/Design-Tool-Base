# Contributing

Thanks for helping improve WebGPU Design Canvas.

## Development setup

Follow the **Quick start** in [README.md](README.md). You need Rust (`wasm32-unknown-unknown`), `wasm-pack`, Node 20+, and pnpm 11+.

```bash
pnpm install
pnpm approve-builds esbuild "@swc/core"
pnpm dev
```

## Project conventions

- **Engine code** lives in `engine/` (Rust). Prefer data-oriented layouts and low allocation in hot paths.
- **UI code** lives in `apps/editor/`. React renders chrome only — never the scene graph.
- Keep the visual language minimal: light gray surfaces, no decorative grids or shadows on the canvas.
- Do not add C++ under `native/cpp/` unless there is a measured performance reason.

## Checks before opening a PR

```bash
pnpm build:wasm
pnpm test:engine
pnpm --filter @webgpu-canvas/editor exec tsc --noEmit
```

Manual smoke test in Chrome/Edge:

1. App loads with a gray WebGPU canvas
2. Create rectangle / frame / shader
3. Select, drag, pan, zoom
4. Escape clears selection and shows the shader gallery

## Pull requests

- Keep changes focused; prefer small PRs.
- Describe *why* the change exists.
- Update the README or `docs/` when behavior or setup steps change.

## License

By contributing, you agree that your contributions are licensed under the MIT License.

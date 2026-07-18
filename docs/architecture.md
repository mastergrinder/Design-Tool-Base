# Architecture

## Ownership

| Owner | Owns |
| --- | --- |
| **Rust / WASM engine** | Document, camera, selection, transforms, GPU resources, frame work |
| **React editor** | DOM chrome (toolbar, layers, properties), browser input capture |

React must not store per-node render state. Panels read a compact `UiSnapshot` from the engine when something panel-relevant changes.

## Frame loop

```text
Browser input
    → Engine.frame(FrameInput)
        → apply pan / zoom / select / drag
        → update dirty world transforms
        → if shaders visible: advance time
        → render_frame (WebGPU)
        → return UiSnapshot (needs_redraw when UI should update)
```

Static scenes skip continuous GPU work when nothing is dirty. Visible shader layers keep the render loop alive for animation.

## Scene storage

Nodes are stored as parallel arrays (structure-of-arrays): transforms, sizes, fills, visibility, layer types, shader ids. This supports dense iteration and is the foundation for later spatial indexing and GPU buffer uploads.

## Rendering

- **Rectangles / frames** — shared unit quad + per-instance attributes
- **Shaders** — catalog of WGSL programs with a shared vertex stage and per-draw uniforms (time, opacity, mouse, rect)
- Paint order follows the document root list (later = on top)
- Selection outlines are a separate fragment path

## Scaling path

Current hit-testing is linear. The public APIs are shaped so later work can swap in:

- R-tree / BVH spatial index
- Frustum / hierarchy culling
- Persistent GPU buffers with partial updates
- Virtualized layer list in the UI

See the root README roadmap for planned milestones.

import type { UiSnapshot } from "@webgpu-canvas/protocol";

/** Thin handle over the wasm-bindgen Engine class. */
export interface EngineHandle {
  create_rectangle(x: number, y: number, w: number, h: number, fillHex: number): number;
  create_frame(x: number, y: number, w: number, h: number): number;
  create_shader_layer(shaderId: number, x: number, y: number, w: number, h: number): number;
  clear_selection(): void;
  select_node(id: number, additive: boolean): void;
  set_visibility(id: number, visible: boolean): void;
  set_locked(id: number, locked: boolean): void;
  set_selected_property(key: string, value: number): void;
  set_selected_fill(hex: number): void;
  zoom_by(factor: number): void;
  reset_camera(): void;
  resize(cssW: number, cssH: number, pixelW: number, pixelH: number): void;
  frame(input: unknown): Promise<UiSnapshot> | UiSnapshot;
  get_ui_snapshot(): UiSnapshot;
}

export async function loadEngine(): Promise<{
  Engine: new () => EngineHandle & {
    init(canvas: HTMLCanvasElement): Promise<void>;
  };
}> {
  const mod = await import("../wasm-pkg/engine.js");
  await mod.default();
  return mod as unknown as {
    Engine: new () => EngineHandle & {
      init(canvas: HTMLCanvasElement): Promise<void>;
    };
  };
}

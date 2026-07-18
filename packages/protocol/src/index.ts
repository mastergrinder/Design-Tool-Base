/** Shared editor ↔ engine protocol types (mirrors Rust serde shapes). */

export interface FrameInput {
  pointer_x: number;
  pointer_y: number;
  pointer_down: boolean;
  pointer_pressed: boolean;
  pointer_released: boolean;
  button: number;
  wheel_delta_y: number;
  ctrl: boolean;
  shift: boolean;
  space: boolean;
  meta: boolean;
}

export interface UiNode {
  id: number;
  name: string;
  layer_type: string;
  visible: boolean;
  locked: boolean;
  x: number;
  y: number;
  width: number;
  height: number;
  fill: [number, number, number, number];
  radius: number;
  opacity: number;
  shader_id: number;
  shader_name: string;
}

export interface UiShaderInfo {
  id: number;
  name: string;
  category: string;
  preview: string;
}

export interface UiSnapshot {
  layers: UiNode[];
  selection: number[];
  zoom: number;
  camera_x: number;
  camera_y: number;
  needs_redraw: boolean;
  shaders: UiShaderInfo[];
}

export function emptyFrameInput(): FrameInput {
  return {
    pointer_x: 0,
    pointer_y: 0,
    pointer_down: false,
    pointer_pressed: false,
    pointer_released: false,
    button: 0,
    wheel_delta_y: 0,
    ctrl: false,
    shift: false,
    space: false,
    meta: false,
  };
}

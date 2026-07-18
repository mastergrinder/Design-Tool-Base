import type { UiShaderInfo, UiSnapshot } from "@webgpu-canvas/protocol";
import { ShaderIcon } from "./Icons";

interface Props {
  snapshot: UiSnapshot | null;
  onSetProperty: (key: string, value: number) => void;
  onSetFill: (hex: number) => void;
  onAddShader: (shaderId: number) => void;
}

function rgbaToHex(fill: [number, number, number, number]): string {
  const r = Math.round(fill[0] * 255)
    .toString(16)
    .padStart(2, "0");
  const g = Math.round(fill[1] * 255)
    .toString(16)
    .padStart(2, "0");
  const b = Math.round(fill[2] * 255)
    .toString(16)
    .padStart(2, "0");
  return `#${r}${g}${b}`;
}

function hexToNumber(hex: string): number {
  return parseInt(hex.replace("#", ""), 16);
}

function ShaderGallery({
  shaders,
  onAdd,
}: {
  shaders: UiShaderInfo[];
  onAdd: (id: number) => void;
}) {
  const categories = Array.from(new Set(shaders.map((s) => s.category)));

  return (
    <div className="shaders">
      <div className="shaders__intro">
        <ShaderIcon size={14} />
        <span>Shaders</span>
      </div>
      <p className="shaders__hint">Click a shader to add it to the canvas.</p>
      {categories.map((cat) => (
        <div key={cat} className="shaders__group">
          <div className="props__label">{cat}</div>
          <div className="shaders__grid">
            {shaders
              .filter((s) => s.category === cat)
              .map((s) => (
                <button
                  key={s.id}
                  type="button"
                  className="shader-card"
                  onClick={() => onAdd(s.id)}
                  title={`Add ${s.name}`}
                >
                  <div
                    className="shader-card__preview"
                    style={{ background: s.preview }}
                  />
                  <div className="shader-card__name">{s.name}</div>
                </button>
              ))}
          </div>
        </div>
      ))}
    </div>
  );
}

export function PropertiesPanel({
  snapshot,
  onSetProperty,
  onSetFill,
  onAddShader,
}: Props) {
  const selectedId = snapshot?.selection[0];
  const node = snapshot?.layers.find((l) => l.id === selectedId);
  const shaders = snapshot?.shaders ?? [];

  if (!node) {
    return (
      <aside className="panel panel--right">
        <div className="panel__header">Properties</div>
        <div className="panel__body panel__body--props">
          <ShaderGallery shaders={shaders} onAdd={onAddShader} />
        </div>
      </aside>
    );
  }

  const isShader = node.layer_type === "Shader";
  const isFrame = node.layer_type === "Frame";

  return (
    <aside className="panel panel--right">
      <div className="panel__header">Properties</div>
      <div className="props">
        <div className="props__title">
          {isShader ? node.shader_name || "Shader" : node.layer_type}
        </div>

        <div className="props__group">
          <div className="props__label">Position</div>
          <label className="props__row">
            <span>X</span>
            <input
              type="number"
              value={Math.round(node.x * 100) / 100}
              onChange={(e) => onSetProperty("x", Number(e.target.value))}
            />
          </label>
          <label className="props__row">
            <span>Y</span>
            <input
              type="number"
              value={Math.round(node.y * 100) / 100}
              onChange={(e) => onSetProperty("y", Number(e.target.value))}
            />
          </label>
        </div>

        <div className="props__group">
          <div className="props__label">Size</div>
          <label className="props__row">
            <span>W</span>
            <input
              type="number"
              min={1}
              value={Math.round(node.width * 100) / 100}
              onChange={(e) => onSetProperty("width", Number(e.target.value))}
            />
          </label>
          <label className="props__row">
            <span>H</span>
            <input
              type="number"
              min={1}
              value={Math.round(node.height * 100) / 100}
              onChange={(e) => onSetProperty("height", Number(e.target.value))}
            />
          </label>
        </div>

        {!isShader && (
          <div className="props__group">
            <div className="props__label">Fill</div>
            <label className="props__row">
              <span />
              <input
                type="color"
                value={rgbaToHex(node.fill)}
                onChange={(e) => onSetFill(hexToNumber(e.target.value))}
              />
            </label>
          </div>
        )}

        {!isShader && !isFrame && (
          <div className="props__group">
            <div className="props__label">Radius</div>
            <label className="props__row">
              <span>R</span>
              <input
                type="number"
                min={0}
                value={Math.round(node.radius * 100) / 100}
                onChange={(e) => onSetProperty("radius", Number(e.target.value))}
              />
            </label>
          </div>
        )}

        <div className="props__group">
          <div className="props__label">Opacity</div>
          <label className="props__row">
            <span>%</span>
            <input
              type="number"
              min={0}
              max={100}
              value={Math.round(node.opacity * 100)}
              onChange={(e) =>
                onSetProperty("opacity", Number(e.target.value) / 100)
              }
            />
          </label>
        </div>

        {isShader && (
          <div className="props__group">
            <div className="props__label">Shader</div>
            <div className="props__muted">{node.shader_name}</div>
          </div>
        )}
      </div>
    </aside>
  );
}

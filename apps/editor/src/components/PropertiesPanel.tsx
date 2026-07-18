import type { UiShaderInfo, UiSnapshot } from "@webgpu-canvas/protocol";
import { LayerTypeIcon, PropertiesHeaderIcon, ShaderIcon } from "./Icons";

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
      <p className="shaders__hint">Click to place a live GPU shader on the canvas.</p>
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
        <div className="panel__header">
          <span className="panel__header-icon">
            <PropertiesHeaderIcon size={12} />
          </span>
          Properties
        </div>
        <div className="panel__body panel__body--props">
          <ShaderGallery shaders={shaders} onAdd={onAddShader} />
        </div>
      </aside>
    );
  }

  const isShader = node.layer_type === "Shader";
  const isFrame = node.layer_type === "Frame";
  const opacityPct = Math.round(node.opacity * 100);

  return (
    <aside className="panel panel--right">
      <div className="panel__header">
        <span className="panel__header-icon">
          <PropertiesHeaderIcon size={12} />
        </span>
        Properties
      </div>
      <div className="props">
        <div className="props__title">
          <LayerTypeIcon type={node.layer_type} size={15} />
          {isShader ? node.shader_name || "Shader" : node.layer_type}
        </div>

        <div className="props__group">
          <div className="props__label">Position</div>
          <div className="props__pair">
            <label className="props__field">
              <span>X</span>
              <input
                type="number"
                value={Math.round(node.x * 100) / 100}
                onChange={(e) => onSetProperty("x", Number(e.target.value))}
              />
            </label>
            <label className="props__field">
              <span>Y</span>
              <input
                type="number"
                value={Math.round(node.y * 100) / 100}
                onChange={(e) => onSetProperty("y", Number(e.target.value))}
              />
            </label>
          </div>
        </div>

        <div className="props__group">
          <div className="props__label">Size</div>
          <div className="props__pair">
            <label className="props__field">
              <span>W</span>
              <input
                type="number"
                min={1}
                value={Math.round(node.width * 100) / 100}
                onChange={(e) => onSetProperty("width", Number(e.target.value))}
              />
            </label>
            <label className="props__field">
              <span>H</span>
              <input
                type="number"
                min={1}
                value={Math.round(node.height * 100) / 100}
                onChange={(e) => onSetProperty("height", Number(e.target.value))}
              />
            </label>
          </div>
        </div>

        {!isShader && (
          <div className="props__group">
            <div className="props__label">Fill</div>
            <label className="props__swatch">
              <input
                type="color"
                value={rgbaToHex(node.fill)}
                onChange={(e) => onSetFill(hexToNumber(e.target.value))}
              />
              <span>{rgbaToHex(node.fill).toUpperCase()}</span>
            </label>
          </div>
        )}

        {!isShader && !isFrame && (
          <div className="props__group">
            <div className="props__label">Corner radius</div>
            <label className="props__field props__field--solo">
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
          <div className="props__label">
            Opacity
            <span className="props__label-value">{opacityPct}%</span>
          </div>
          <input
            className="props__slider"
            type="range"
            min={0}
            max={100}
            value={opacityPct}
            onChange={(e) =>
              onSetProperty("opacity", Number(e.target.value) / 100)
            }
          />
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

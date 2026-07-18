import type { UiSnapshot } from "@webgpu-canvas/protocol";
import { LayerTypeIcon } from "./Icons";

interface Props {
  snapshot: UiSnapshot | null;
  onSelect: (id: number, additive: boolean) => void;
  onToggleVisible: (id: number, visible: boolean) => void;
  onToggleLocked: (id: number, locked: boolean) => void;
}

export function LayersPanel({
  snapshot,
  onSelect,
  onToggleVisible,
  onToggleLocked,
}: Props) {
  const layers = snapshot?.layers ?? [];
  const selection = new Set(snapshot?.selection ?? []);

  return (
    <aside className="panel panel--left">
      <div className="panel__header">Layers</div>
      <div className="panel__body">
        {layers.map((layer) => {
          const selected = selection.has(layer.id);
          return (
            <div
              key={layer.id}
              className={`layer-row${selected ? " is-selected" : ""}`}
              onClick={(e) => onSelect(layer.id, e.shiftKey)}
            >
              <button
                type="button"
                className="layer-row__toggle"
                title={layer.visible ? "Hide" : "Show"}
                onClick={(e) => {
                  e.stopPropagation();
                  onToggleVisible(layer.id, !layer.visible);
                }}
              >
                {layer.visible ? "●" : "○"}
              </button>
              <button
                type="button"
                className="layer-row__toggle"
                title={layer.locked ? "Unlock" : "Lock"}
                onClick={(e) => {
                  e.stopPropagation();
                  onToggleLocked(layer.id, !layer.locked);
                }}
              >
                {layer.locked ? "▮" : "▯"}
              </button>
              <span className="layer-row__icon" title={layer.layer_type}>
                <LayerTypeIcon type={layer.layer_type} />
              </span>
              <div className="layer-row__name">{layer.name}</div>
            </div>
          );
        })}
      </div>
    </aside>
  );
}

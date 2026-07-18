import type { UiSnapshot } from "@webgpu-canvas/protocol";
import {
  EyeClosedIcon,
  EyeOpenIcon,
  LayerTypeIcon,
  LayersHeaderIcon,
  LockedIcon,
  UnlockedIcon,
} from "./Icons";

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
      <div className="panel__header">
        <span className="panel__header-icon">
          <LayersHeaderIcon size={12} />
        </span>
        Layers
      </div>
      <div className="panel__body">
        {layers.length === 0 ? (
          <div className="props__empty">No layers yet</div>
        ) : (
          layers.map((layer) => {
            const selected = selection.has(layer.id);
            return (
              <div
                key={layer.id}
                className={`layer-row${selected ? " is-selected" : ""}`}
                onClick={(e) => onSelect(layer.id, e.shiftKey)}
              >
                <button
                  type="button"
                  className={`layer-row__toggle${layer.visible ? "" : " is-off"}`}
                  title={layer.visible ? "Hide" : "Show"}
                  onClick={(e) => {
                    e.stopPropagation();
                    onToggleVisible(layer.id, !layer.visible);
                  }}
                >
                  {layer.visible ? (
                    <EyeOpenIcon size={14} />
                  ) : (
                    <EyeClosedIcon size={14} />
                  )}
                </button>
                <button
                  type="button"
                  className={`layer-row__toggle${layer.locked ? "" : " is-off"}`}
                  title={layer.locked ? "Unlock" : "Lock"}
                  onClick={(e) => {
                    e.stopPropagation();
                    onToggleLocked(layer.id, !layer.locked);
                  }}
                >
                  {layer.locked ? (
                    <LockedIcon size={13} />
                  ) : (
                    <UnlockedIcon size={13} />
                  )}
                </button>
                <span className="layer-row__icon" title={layer.layer_type}>
                  <LayerTypeIcon type={layer.layer_type} size={14} />
                </span>
                <div className="layer-row__name">{layer.name}</div>
              </div>
            );
          })
        )}
      </div>
    </aside>
  );
}

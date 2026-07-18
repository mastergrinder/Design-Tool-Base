import {
  FitViewIcon,
  FrameIcon,
  HandToolIcon,
  RectangleIcon,
  SelectToolIcon,
  ZoomInIcon,
  ZoomMetaIcon,
  ZoomOutIcon,
} from "./Icons";

export type EditorTool = "select" | "pan";

interface Props {
  zoom: number;
  tool: EditorTool;
  onToolChange: (tool: EditorTool) => void;
  onAddRectangle: () => void;
  onAddFrame: () => void;
  onZoomIn: () => void;
  onZoomOut: () => void;
  onResetView: () => void;
  ready: boolean;
}

export function Toolbar({
  zoom,
  tool,
  onToolChange,
  onAddRectangle,
  onAddFrame,
  onZoomIn,
  onZoomOut,
  onResetView,
  ready,
}: Props) {
  return (
    <header className="toolbar">
      <div className="toolbar__brand">
        <span className="toolbar__mark" aria-hidden />
        Canvas
      </div>

      <div className="toolbar__sep" />

      <div className="toolbar__group">
        <button
          type="button"
          className={`toolbar__icon-btn${tool === "select" ? " is-active" : ""}`}
          disabled={!ready}
          title="Select (V)"
          onClick={() => onToolChange("select")}
        >
          <SelectToolIcon size={15} />
        </button>
        <button
          type="button"
          className={`toolbar__icon-btn${tool === "pan" ? " is-active" : ""}`}
          disabled={!ready}
          title="Hand / pan (H) — or hold Space"
          onClick={() => onToolChange("pan")}
        >
          <HandToolIcon size={15} />
        </button>
      </div>

      <div className="toolbar__sep" />

      <div className="toolbar__group">
        <button
          type="button"
          className="toolbar__btn"
          onClick={onAddRectangle}
          disabled={!ready}
          title="Rectangle (⌘R)"
        >
          <RectangleIcon size={15} />
          Rectangle
        </button>
        <button
          type="button"
          className="toolbar__btn"
          onClick={onAddFrame}
          disabled={!ready}
          title="Frame (⌘F)"
        >
          <FrameIcon size={15} />
          Frame
        </button>
      </div>

      <div className="toolbar__zoom">
        <button
          type="button"
          className="toolbar__icon-btn"
          onClick={onZoomOut}
          disabled={!ready}
          title="Zoom out"
        >
          <ZoomOutIcon size={14} />
        </button>
        <button
          type="button"
          className="toolbar__zoom-label"
          onClick={onResetView}
          disabled={!ready}
          title="Reset view"
        >
          <ZoomMetaIcon size={13} />
          {Math.round(zoom * 100)}%
        </button>
        <button
          type="button"
          className="toolbar__icon-btn"
          onClick={onZoomIn}
          disabled={!ready}
          title="Zoom in"
        >
          <ZoomInIcon size={14} />
        </button>
        <button
          type="button"
          className="toolbar__icon-btn"
          onClick={onResetView}
          disabled={!ready}
          title="Fit / reset view"
        >
          <FitViewIcon size={14} />
        </button>
      </div>
    </header>
  );
}

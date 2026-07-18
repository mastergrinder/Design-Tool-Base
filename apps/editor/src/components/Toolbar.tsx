import { FrameIcon, RectangleIcon } from "./Icons";

interface Props {
  zoom: number;
  onAddRectangle: () => void;
  onAddFrame: () => void;
  ready: boolean;
}

export function Toolbar({ zoom, onAddRectangle, onAddFrame, ready }: Props) {
  return (
    <header className="toolbar">
      <div className="toolbar__brand">Canvas</div>
      <div className="toolbar__sep" />
      <button
        type="button"
        className="toolbar__btn"
        onClick={onAddRectangle}
        disabled={!ready}
        title="Add rectangle"
      >
        <RectangleIcon size={13} />
        Rectangle
      </button>
      <button
        type="button"
        className="toolbar__btn"
        onClick={onAddFrame}
        disabled={!ready}
        title="Add frame"
      >
        <FrameIcon size={13} />
        Frame
      </button>
      <div className="toolbar__meta">{Math.round(zoom * 100)}%</div>
    </header>
  );
}

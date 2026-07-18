import { useCallback, useEffect, useRef, useState } from "react";
import type { UiSnapshot } from "@webgpu-canvas/protocol";
import { CanvasView } from "./components/CanvasView";
import { LayersPanel } from "./components/LayersPanel";
import { PropertiesPanel } from "./components/PropertiesPanel";
import { Toolbar, type EditorTool } from "./components/Toolbar";
import type { EngineHandle } from "./engine/bridge";

export function App() {
  const engineRef = useRef<EngineHandle | null>(null);
  const layerCountRef = useRef(0);
  const [snapshot, setSnapshot] = useState<UiSnapshot | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [ready, setReady] = useState(false);
  const [tool, setTool] = useState<EditorTool>("select");

  const onSnapshot = useCallback((snap: UiSnapshot) => {
    layerCountRef.current = snap.layers.length;
    setSnapshot(snap);
  }, []);

  const onReady = useCallback((engine: EngineHandle) => {
    engineRef.current = engine;
    setReady(true);
  }, []);

  const onError = useCallback((message: string) => {
    setError(message);
  }, []);

  const createRect = useCallback(() => {
    const engine = engineRef.current;
    if (!engine) return;
    const n = layerCountRef.current + 1;
    const x = -180 + (n % 5) * 40;
    const y = -100 + (n % 7) * 30;
    const colors = [0xffffff, 0xf2f2f2, 0xe8eef8, 0xf7f0e8, 0xeaf5ea];
    engine.create_rectangle(x, y, 320, 200, colors[n % colors.length]!);
  }, []);

  const createFrame = useCallback(() => {
    const engine = engineRef.current;
    if (!engine) return;
    const n = layerCountRef.current + 1;
    const x = -240 + (n % 4) * 36;
    const y = -160 + (n % 5) * 28;
    engine.create_frame(x, y, 480, 320);
  }, []);

  const createShader = useCallback((shaderId: number) => {
    const engine = engineRef.current;
    if (!engine) return;
    const n = layerCountRef.current + 1;
    const x = -160 + (n % 6) * 28;
    const y = -100 + (n % 5) * 24;
    engine.create_shader_layer(shaderId, x, y, 360, 240);
  }, []);

  const zoomIn = useCallback(() => {
    engineRef.current?.zoom_by(1.15);
  }, []);

  const zoomOut = useCallback(() => {
    engineRef.current?.zoom_by(1 / 1.15);
  }, []);

  const resetView = useCallback(() => {
    engineRef.current?.reset_camera();
  }, []);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      const target = e.target as HTMLElement | null;
      const typing =
        target &&
        (target.tagName === "INPUT" ||
          target.tagName === "TEXTAREA" ||
          target.isContentEditable);
      if (typing) return;

      if (e.code === "KeyV" && !e.ctrlKey && !e.metaKey && !e.altKey) {
        setTool("select");
      }
      if (e.code === "KeyH" && !e.ctrlKey && !e.metaKey && !e.altKey) {
        setTool("pan");
      }
      if (e.code === "KeyR" && (e.ctrlKey || e.metaKey) && !e.shiftKey) {
        e.preventDefault();
        createRect();
      }
      if (e.code === "KeyF" && (e.ctrlKey || e.metaKey)) {
        e.preventDefault();
        createFrame();
      }
      if (e.code === "Digit0" && (e.ctrlKey || e.metaKey)) {
        e.preventDefault();
        resetView();
      }
      if (e.code === "Equal" && (e.ctrlKey || e.metaKey)) {
        e.preventDefault();
        zoomIn();
      }
      if (e.code === "Minus" && (e.ctrlKey || e.metaKey)) {
        e.preventDefault();
        zoomOut();
      }
      if (e.code === "Escape") {
        if (tool === "pan") {
          setTool("select");
        } else {
          engineRef.current?.clear_selection();
        }
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [createRect, createFrame, resetView, zoomIn, zoomOut, tool]);

  return (
    <div className="app">
      <Toolbar
        zoom={snapshot?.zoom ?? 1}
        tool={tool}
        onToolChange={setTool}
        onAddRectangle={createRect}
        onAddFrame={createFrame}
        onZoomIn={zoomIn}
        onZoomOut={zoomOut}
        onResetView={resetView}
        ready={ready}
      />
      <div className="workspace">
        <LayersPanel
          snapshot={snapshot}
          onSelect={(id, additive) => engineRef.current?.select_node(id, additive)}
          onToggleVisible={(id, visible) =>
            engineRef.current?.set_visibility(id, visible)
          }
          onToggleLocked={(id, locked) =>
            engineRef.current?.set_locked(id, locked)
          }
        />
        <CanvasView
          tool={tool}
          onReady={onReady}
          onSnapshot={onSnapshot}
          onError={onError}
          error={error}
        />
        <PropertiesPanel
          snapshot={snapshot}
          onSetProperty={(key, value) =>
            engineRef.current?.set_selected_property(key, value)
          }
          onSetFill={(hex) => engineRef.current?.set_selected_fill(hex)}
          onAddShader={createShader}
        />
      </div>
    </div>
  );
}

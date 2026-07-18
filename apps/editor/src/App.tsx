import { useCallback, useEffect, useRef, useState } from "react";
import type { UiSnapshot } from "@webgpu-canvas/protocol";
import { CanvasView } from "./components/CanvasView";
import { LayersPanel } from "./components/LayersPanel";
import { PropertiesPanel } from "./components/PropertiesPanel";
import { Toolbar } from "./components/Toolbar";
import type { EngineHandle } from "./engine/bridge";

export function App() {
  const engineRef = useRef<EngineHandle | null>(null);
  const layerCountRef = useRef(0);
  const [snapshot, setSnapshot] = useState<UiSnapshot | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [ready, setReady] = useState(false);

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

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.code === "KeyR" && (e.ctrlKey || e.metaKey) && !e.shiftKey) {
        e.preventDefault();
        createRect();
      }
      if (e.code === "KeyF" && (e.ctrlKey || e.metaKey)) {
        e.preventDefault();
        createFrame();
      }
      if (e.code === "Escape") {
        engineRef.current?.clear_selection();
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [createRect, createFrame]);

  return (
    <div className="app">
      <Toolbar
        zoom={snapshot?.zoom ?? 1}
        onAddRectangle={createRect}
        onAddFrame={createFrame}
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

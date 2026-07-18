import { useEffect, useRef, useState } from "react";
import { emptyFrameInput, type FrameInput, type UiSnapshot } from "@webgpu-canvas/protocol";
import { loadEngine, type EngineHandle } from "../engine/bridge";
import type { EditorTool } from "./Toolbar";

interface Props {
  tool: EditorTool;
  onReady: (engine: EngineHandle) => void;
  onSnapshot: (snap: UiSnapshot) => void;
  onError: (message: string) => void;
  error: string | null;
}

export function CanvasView({ tool, onReady, onSnapshot, onError, error }: Props) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const engineRef = useRef<EngineHandle | null>(null);
  const inputRef = useRef<FrameInput>(emptyFrameInput());
  const keysRef = useRef({ space: false, ctrl: false, shift: false, meta: false });
  const toolRef = useRef<EditorTool>(tool);
  const [showHint, setShowHint] = useState(true);

  useEffect(() => {
    toolRef.current = tool;
    const canvas = canvasRef.current;
    if (!canvas) return;
    if (tool === "pan" || keysRef.current.space) {
      canvas.style.cursor = "grab";
    } else {
      canvas.style.cursor = "default";
    }
  }, [tool]);

  useEffect(() => {
    const t = window.setTimeout(() => setShowHint(false), 4500);
    return () => window.clearTimeout(t);
  }, []);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    let raf = 0;
    let disposed = false;
    let pointerDown = false;
    let pressedThisFrame = false;
    let releasedThisFrame = false;
    let button = 0;
    let wheelDelta = 0;

    const isPanActive = () => toolRef.current === "pan" || keysRef.current.space;

    const syncSize = () => {
      const parent = canvas.parentElement;
      if (!parent || !engineRef.current) return;
      const rect = parent.getBoundingClientRect();
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      const cssW = Math.max(1, Math.floor(rect.width));
      const cssH = Math.max(1, Math.floor(rect.height));
      const pixelW = Math.max(1, Math.floor(cssW * dpr));
      const pixelH = Math.max(1, Math.floor(cssH * dpr));
      if (canvas.width !== pixelW || canvas.height !== pixelH) {
        canvas.width = pixelW;
        canvas.height = pixelH;
      }
      engineRef.current.resize(cssW, cssH, pixelW, pixelH);
    };

    const pointerPos = (e: PointerEvent) => {
      const rect = canvas.getBoundingClientRect();
      return {
        x: e.clientX - rect.left,
        y: e.clientY - rect.top,
      };
    };

    const onPointerDown = (e: PointerEvent) => {
      setShowHint(false);
      canvas.setPointerCapture(e.pointerId);
      const p = pointerPos(e);
      pointerDown = true;
      pressedThisFrame = true;
      button = e.button;
      inputRef.current.pointer_x = p.x;
      inputRef.current.pointer_y = p.y;
      if (isPanActive() || e.button === 1) {
        canvas.style.cursor = "grabbing";
      }
      e.preventDefault();
    };

    const onPointerMove = (e: PointerEvent) => {
      const p = pointerPos(e);
      inputRef.current.pointer_x = p.x;
      inputRef.current.pointer_y = p.y;
    };

    const onPointerUp = (e: PointerEvent) => {
      const p = pointerPos(e);
      pointerDown = false;
      releasedThisFrame = true;
      inputRef.current.pointer_x = p.x;
      inputRef.current.pointer_y = p.y;
      canvas.style.cursor = isPanActive() ? "grab" : "default";
    };

    const onWheel = (e: WheelEvent) => {
      if (e.ctrlKey || e.metaKey) {
        e.preventDefault();
        wheelDelta += e.deltaY;
      }
    };

    const onKeyDown = (e: KeyboardEvent) => {
      if (e.code === "Space") {
        keysRef.current.space = true;
        canvas.style.cursor = pointerDown ? "grabbing" : "grab";
        e.preventDefault();
      }
      keysRef.current.ctrl = e.ctrlKey;
      keysRef.current.shift = e.shiftKey;
      keysRef.current.meta = e.metaKey;
    };

    const onKeyUp = (e: KeyboardEvent) => {
      if (e.code === "Space") {
        keysRef.current.space = false;
        canvas.style.cursor = toolRef.current === "pan" ? "grab" : "default";
      }
      keysRef.current.ctrl = e.ctrlKey;
      keysRef.current.shift = e.shiftKey;
      keysRef.current.meta = e.metaKey;
    };

    const onBlur = () => {
      keysRef.current = { space: false, ctrl: false, shift: false, meta: false };
      pointerDown = false;
      canvas.style.cursor = toolRef.current === "pan" ? "grab" : "default";
    };

    canvas.addEventListener("pointerdown", onPointerDown);
    canvas.addEventListener("pointermove", onPointerMove);
    canvas.addEventListener("pointerup", onPointerUp);
    canvas.addEventListener("pointercancel", onPointerUp);
    canvas.addEventListener("wheel", onWheel, { passive: false });
    window.addEventListener("keydown", onKeyDown);
    window.addEventListener("keyup", onKeyUp);
    window.addEventListener("blur", onBlur);

    const ro = new ResizeObserver(() => syncSize());
    if (canvas.parentElement) ro.observe(canvas.parentElement);

    (async () => {
      try {
        if (!("gpu" in navigator)) {
          throw new Error(
            "WebGPU is not available in this browser. Use a recent Chrome or Edge build.",
          );
        }
        const mod = await loadEngine();
        if (disposed) return;
        const engine = new mod.Engine();
        const parent = canvas.parentElement!;
        const rect = parent.getBoundingClientRect();
        const dpr = Math.min(window.devicePixelRatio || 1, 2);
        const cssW = Math.max(1, Math.floor(rect.width));
        const cssH = Math.max(1, Math.floor(rect.height));
        const pixelW = Math.max(1, Math.floor(cssW * dpr));
        const pixelH = Math.max(1, Math.floor(cssH * dpr));
        canvas.width = pixelW;
        canvas.height = pixelH;
        await engine.init(canvas);
        if (disposed) return;
        engineRef.current = engine;
        engine.resize(cssW, cssH, pixelW, pixelH);
        onReady(engine);

        const tick = () => {
          if (disposed || !engineRef.current) return;
          const input: FrameInput = {
            ...inputRef.current,
            pointer_down: pointerDown,
            pointer_pressed: pressedThisFrame,
            pointer_released: releasedThisFrame,
            button,
            wheel_delta_y: wheelDelta,
            ctrl: keysRef.current.ctrl,
            shift: keysRef.current.shift,
            // Hand tool uses the same pan path as Space.
            space: isPanActive(),
            meta: keysRef.current.meta,
          };
          pressedThisFrame = false;
          releasedThisFrame = false;
          wheelDelta = 0;

          const result = engineRef.current.frame(input) as UiSnapshot;
          if (result?.needs_redraw) onSnapshot(result);
          raf = requestAnimationFrame(tick);
        };
        raf = requestAnimationFrame(tick);
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        onError(message);
      }
    })();

    return () => {
      disposed = true;
      cancelAnimationFrame(raf);
      ro.disconnect();
      canvas.removeEventListener("pointerdown", onPointerDown);
      canvas.removeEventListener("pointermove", onPointerMove);
      canvas.removeEventListener("pointerup", onPointerUp);
      canvas.removeEventListener("pointercancel", onPointerUp);
      canvas.removeEventListener("wheel", onWheel);
      window.removeEventListener("keydown", onKeyDown);
      window.removeEventListener("keyup", onKeyUp);
      window.removeEventListener("blur", onBlur);
    };
  }, [onReady, onSnapshot, onError]);

  return (
    <div
      className={`canvas-host${showHint && !error ? " is-idle" : ""}${
        tool === "pan" ? " is-panning" : ""
      }`}
    >
      <canvas ref={canvasRef} tabIndex={0} />
      {error ? (
        <div className="canvas-host__error">{error}</div>
      ) : showHint ? (
        <div className="canvas-host__hint">
          Hand tool or Space to pan · Ctrl + scroll to zoom
        </div>
      ) : null}
    </div>
  );
}

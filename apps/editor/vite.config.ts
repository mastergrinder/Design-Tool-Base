import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import path from "node:path";

export default defineConfig({
  plugins: [react(), wasm(), topLevelAwait()],
  resolve: {
    alias: {
      "@webgpu-canvas/protocol": path.resolve(__dirname, "../../packages/protocol/src/index.ts"),
    },
  },
  server: {
    port: 5173,
  },
  optimizeDeps: {
    exclude: ["@webgpu-canvas/protocol"],
  },
});

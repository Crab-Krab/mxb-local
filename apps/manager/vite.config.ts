import { defineConfig, type Plugin } from "vite";
import { mkdirSync, writeFileSync } from "node:fs";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import path from "node:path";

// @tauri-apps/cli sets TAURI_DEV_HOST when developing against a physical device.
const host = process.env.TAURI_DEV_HOST;

// Several worktrees of this repo get run at once, and `strictPort` means the second one to
// start just dies. `MXB_DEV_PORT` moves this instance out of the way; Tauri has to be told
// the same number, so it travels with a matching `--config '{"build":{"devUrl":…}}'`.
const port = Number(process.env.MXB_DEV_PORT) || 1420;

// Build-time evidence: Rollup reports which exports survive into production chunks.
// Kept outside dist so the installed app carries no dependency-review metadata.
function dependencyInventory(): Plugin {
  return {
    name: "mxb-local-dependency-inventory",
    apply: "build",
    generateBundle(_options, bundle) {
      const modules = Object.values(bundle).flatMap((chunk) =>
        chunk.type === "chunk" ? Object.entries(chunk.modules).map(([id, info]) => ({
          id: path.relative(__dirname, id).replace(/\\/g, "/"),
          renderedLength: info.renderedLength,
          renderedExports: info.renderedExports,
          removedExports: info.removedExports,
        })) : [],
      );
      const output = path.resolve(__dirname, "../../.security-results");
      mkdirSync(output, { recursive: true });
      writeFileSync(path.join(output, "frontend-runtime-modules.json"), JSON.stringify(modules, null, 2));
      for (const module of modules.filter((m) => /[\/](fflate|zod|source-map-js|nanoid)[\/]/.test(m.id))) {
        console.log("Runtime dependency review: " + JSON.stringify(module));
      }
    },
  };
}

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [react(), tailwindcss(), ...(process.env.MXB_DEPENDENCY_REVIEW ? [dependencyInventory()] : [])],

  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
      // `@/` is this app's own src; the shared package gets its own specifier so an
      // import line always says which side of the boundary a module is on.
      "@frost/shared": path.resolve(__dirname, "../../packages/shared/src"),
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`.
  //
  // 1. prevent Vite from obscuring Rust errors
  clearScreen: false,
  // 2. Tauri expects a fixed port, fail if that port is not available
  server: {
    port,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: port + 1,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  // 4. to make use of `TAURI_ENV_*` and app env variables in the frontend
  envPrefix: ["VITE_", "TAURI_ENV_"],
}));

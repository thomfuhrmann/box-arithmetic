import path from "node:path";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

// https://vite.dev/config/
export default defineConfig({
	base: "/box-arithmetic/",
	plugins: [react(), tailwindcss(), wasm(), topLevelAwait()],
	resolve: {
		alias: {
			"@": path.resolve(__dirname, "./src"),
			wasm: path.resolve(__dirname, "../wasm/pkg"),
		},
	},
	build: {
		target: "esnext",
		chunkSizeWarningLimit: 1500,
	},
});

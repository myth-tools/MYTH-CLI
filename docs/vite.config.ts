import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

export default defineConfig({
	plugins: [react(), tailwindcss()],
	base: "/",
	build: {
		outDir: "dist",
		chunkSizeWarningLimit: 2000,
		// target esnext for smallest modern output
		target: "esnext",
		rollupOptions: {
			output: {
				// Granular manual chunks — heavy deps get isolated for better caching
				manualChunks(id) {
					if (
						id.includes("node_modules/react") ||
						id.includes("node_modules/react-dom") ||
						id.includes("node_modules/react-router")
					) {
						return "react-vendor";
					}
					if (id.includes("node_modules/framer-motion")) {
						return "framer-motion";
					}
					if (id.includes("node_modules/reactflow")) {
						return "reactflow";
					}
					if (id.includes("node_modules/recharts")) {
						return "recharts";
					}
					if (id.includes("node_modules/@monaco-editor")) {
						return "monaco";
					}
					if (id.includes("node_modules/lucide-react")) {
						return "lucide";
					}
					if (id.includes("node_modules/gsap")) {
						return "gsap";
					}
				},
			},
		},
	},
	// Optimise dev server
	server: {
		warmup: {
			clientFiles: [
				"./src/main.tsx",
				"./src/App.tsx",
				"./src/components/Layout.tsx",
				"./src/data/content.ts",
			],
		},
	},
	// Inline small assets for fewer HTTP requests
	assetsInclude: ["**/*.svg"],
	optimizeDeps: {
		include: ["react", "react-dom", "react-router-dom", "framer-motion", "lucide-react"],
	},
});

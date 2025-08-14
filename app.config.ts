import { defineConfig } from "@solidjs/start/config";
import tailwindcss from '@tailwindcss/vite';
import path from "path";

export default defineConfig({
  vite: {
    plugins: [tailwindcss()],
    resolve: {
      alias: {
        "~": path.resolve(__dirname, "./src")
      }
    }
  }
});

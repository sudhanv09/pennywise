import { defineConfig } from "@solidjs/start/config";
import tailwindcss from '@tailwindcss/vite';
import path from "path";
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

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

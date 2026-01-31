import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import createSvgSpritePlugin from 'vite-plugin-svg-sprite'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    svelte(),
    createSvgSpritePlugin({
      symbolId: 'icon-[name]',
    }),
  ],

  // Vite options tailored for Tauri development
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    host: '127.0.0.1',
    watch: {
      // Tell vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**'],
    },
  },

  // Configure for Tauri
  envPrefix: ['VITE_', 'TAURI_'],

  build: {
    outDir: './dist/web',
    // Tauri uses Chromium on Windows and WebKit on macOS and Linux
    target: process.env.TAURI_ENV_PLATFORM === 'windows'
      ? 'chrome105'
      : 'safari14',
    // don't minify for debug builds
    minify: !process.env.TAURI_ENV_DEBUG ? 'esbuild' : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
})

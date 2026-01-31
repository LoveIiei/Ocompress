import { invoke, convertFileSrc } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { open as shellOpen } from '@tauri-apps/plugin-shell'
import type {
  IImageFile,
  IOptimizeRequest,
  PlaceholderRequest,
  SaveType,
} from '../common/types'
import type { ImagineAPI } from './interface'

// Simple console logger for Tauri
const logger = {
  info: (...args: unknown[]) => console.log('[INFO]', ...args),
  warn: (...args: unknown[]) => console.warn('[WARN]', ...args),
  error: (...args: unknown[]) => console.error('[ERROR]', ...args),
  debug: (...args: unknown[]) => console.debug('[DEBUG]', ...args),
  log: (...args: unknown[]) => console.log(...args),
}

// Event listeners storage
const eventListeners = new Map<string, UnlistenFn[]>()

/**
 * Convert file:// URL to Tauri asset URL for webview access
 */
function convertImageUrl(url: string): string {
  if (url.startsWith('file://')) {
    // Extract the path from file:// URL
    const filePath = url.replace('file://', '')
    return convertFileSrc(filePath)
  }
  return url
}

/**
 * Convert image file URLs to be accessible in webview
 */
function convertImageFile(image: IImageFile): IImageFile {
  return {
    ...image,
    url: convertImageUrl(image.url),
  }
}

/**
 * Register an event listener
 */
async function registerListener(
  eventName: string,
  callback: (payload: unknown) => void,
): Promise<UnlistenFn> {
  const unlisten = await listen(eventName, (event) => {
    callback(event.payload)
  })

  // Store for cleanup
  if (!eventListeners.has(eventName)) {
    eventListeners.set(eventName, [])
  }
  eventListeners.get(eventName)!.push(unlisten)

  return unlisten
}

/**
 * Process files (from drag-drop or file dialog)
 */
async function processFiles(filePaths: string[]): Promise<IImageFile[]> {
  const images: IImageFile[] = await invoke('process_files', { filePaths })
  return images.map(convertImageFile)
}

/**
 * Open file dialog
 */
async function openFileDialog(): Promise<IImageFile[]> {
  const images: IImageFile[] = await invoke('open_file_dialog')
  return images.map(convertImageFile)
}

/**
 * Optimize an image
 */
async function optimize(request: IOptimizeRequest): Promise<IImageFile> {
  const result: IImageFile = await invoke('optimize_image', { request })
  return convertImageFile(result)
}

/**
 * Save files
 */
async function saveFiles(images: IImageFile[], saveType: SaveType): Promise<void> {
  return invoke('save_files', {
    request: { images, saveType },
  })
}

/**
 * Open external URL in default browser
 */
async function openExternal(url: string): Promise<void> {
  await shellOpen(url)
}

/**
 * Listen for files dropped
 */
async function onFilesDropped(
  callback: (filePaths: string[]) => void,
): Promise<UnlistenFn> {
  return registerListener('files-dropped', callback as (p: unknown) => void)
}

/**
 * Listen for menu save events
 */
async function onMenuSave(
  callback: (saveType: SaveType) => void,
): Promise<UnlistenFn> {
  return registerListener('menu-save', callback as (p: unknown) => void)
}

/**
 * Listen for menu open events
 */
async function onMenuOpen(callback: () => void): Promise<UnlistenFn> {
  return registerListener('menu-open', callback as (p: unknown) => void)
}

/**
 * Listen for about menu events
 */
async function onMenuAbout(callback: () => void): Promise<UnlistenFn> {
  return registerListener('menu-about', callback as (p: unknown) => void)
}

/**
 * Clean temp directory
 */
async function cleanTempDir(): Promise<void> {
  return invoke('clean_temp_dir')
}

/**
 * Generate placeholder image
 */
async function generatePlaceholder(request: PlaceholderRequest): Promise<void> {
  return invoke('generate_placeholder', { request })
}

// Export the Tauri API implementation
export const tauriAPI: ImagineAPI = {
  logger,
  processFiles,
  openFileDialog,
  optimize,
  saveFiles,
  openExternal,
  onFilesDropped,
  onMenuSave,
  onMenuOpen,
  onMenuAbout,
  cleanTempDir,
  generatePlaceholder,
}

export default tauriAPI

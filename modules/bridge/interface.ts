import type { UnlistenFn } from '@tauri-apps/api/event'
import type {
  IImageFile,
  IOptimizeRequest,
  PlaceholderRequest,
  SaveType,
} from '../common/types'

export interface ImagineLogger {
  info(...args: unknown[]): void
  warn(...args: unknown[]): void
  error(...args: unknown[]): void
  debug(...args: unknown[]): void
  log(...args: unknown[]): void
}

export interface ImagineAPI {
  logger: ImagineLogger

  // File operations
  processFiles(filePaths: string[]): Promise<IImageFile[]>
  openFileDialog(): Promise<IImageFile[]>
  saveFiles(images: IImageFile[], saveType: SaveType): Promise<void>
  cleanTempDir(): Promise<void>

  // Optimization
  optimize(request: IOptimizeRequest): Promise<IImageFile>

  // Placeholder generation
  generatePlaceholder(request: PlaceholderRequest): Promise<void>

  // Shell
  openExternal(url: string): Promise<void>

  // Event listeners
  onFilesDropped(callback: (filePaths: string[]) => void): Promise<UnlistenFn>
  onMenuSave(callback: (saveType: SaveType) => void): Promise<UnlistenFn>
  onMenuOpen(callback: () => void): Promise<UnlistenFn>
  onMenuAbout(callback: () => void): Promise<UnlistenFn>
}

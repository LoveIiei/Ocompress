import { sleep } from '../../common/utils'
import type { ImagineAPI } from '../interface'
import type { IImageFile, IOptimizeRequest, SaveType } from '../../common/types'

const mockLogger = {
  info: jest.fn(),
  warn: jest.fn(),
  error: jest.fn(),
  debug: jest.fn(),
  log: jest.fn(),
}

export const imagineAPI: ImagineAPI = {
  logger: mockLogger,

  async processFiles(filePaths: string[]): Promise<IImageFile[]> {
    await sleep(10)
    return filePaths.map((file, index) => ({
      id: `mock-${index}`,
      url: `file://${file}`,
      size: 1000,
      ext: 'png' as const,
      originalName: file,
    }))
  },

  async openFileDialog(): Promise<IImageFile[]> {
    await sleep(10)
    return []
  },

  async optimize(request: IOptimizeRequest): Promise<IImageFile> {
    await sleep(100)
    return {
      ...request.image,
      id: `optimized-${request.image.id}`,
      size: Math.floor(request.image.size * 0.5),
    }
  },

  async saveFiles(_images: IImageFile[], _saveType: SaveType): Promise<void> {
    await sleep(10)
  },

  async openExternal(_url: string): Promise<void> {
    await sleep(10)
  },

  async cleanTempDir(): Promise<void> {
    await sleep(10)
  },

  async onFilesDropped(_callback: (filePaths: string[]) => void): Promise<() => void> {
    return () => {}
  },

  async onMenuSave(_callback: (saveType: SaveType) => void): Promise<() => void> {
    return () => {}
  },

  async onMenuOpen(_callback: () => void): Promise<() => void> {
    return () => {}
  },

  async onMenuAbout(_callback: () => void): Promise<() => void> {
    return () => {}
  },
}

// eslint-disable-next-line @typescript-eslint/ban-types
export type Empty = {}

export enum SupportedExt {
  png = 'png',
  jpg = 'jpg',
  webp = 'webp',
  ico = 'ico',
  bmp = 'bmp',
  gif = 'gif',
}

export const SupportedExtAlias: Record<string, SupportedExt> = {
  jpeg: SupportedExt.jpg,
}

export const enum TaskStatus {
  PENDING = 'PENDING',
  PROCESSING = 'PROCESSING',
  DONE = 'DONE',
  FAIL = 'FAIL',
}

export const enum SaveType {
  OVER = 'OVER',
  NEW_NAME = 'NEW_NAME',
  NEW_DIR = 'NEW_DIR',
  SAVE_AS = 'SAVE_AS',
}

export interface IImageFile {
  id: string
  url: string
  size: number
  ext: SupportedExt
  originalName: string
}

export interface IOptimizeOptions {
  /**
   * 2~256, for PNG
   */
  color?: number

  /**
   * 10~100, for JPEG
   */
  quality?: number

  exportExt?: SupportedExt
}

export interface ITaskItem {
  id: string
  image: IImageFile
  options: IOptimizeOptions
  optimized?: IImageFile
  status: TaskStatus
}

export interface IOptimizeRequest {
  image: IImageFile
  options: IOptimizeOptions
  exportExt?: SupportedExt
}

export interface IBackendState {
  taskCount: number
  aloneMode: boolean
}

export interface IUpdateInfo {
  version: string
  releaseDate?: string
  releaseNotes?: string
}

export interface IDefaultOptions {
  jpg: IOptimizeOptions
  png: IOptimizeOptions
  webp: IOptimizeOptions
  ico?: IOptimizeOptions
  bmp?: IOptimizeOptions
  gif?: IOptimizeOptions
}

export interface IGlobals {
  activeId?: string
  updateInfo?: IUpdateInfo
  optionsVisible: boolean
  defaultOptions: IDefaultOptions
}

export interface IState {
  tasks: ITaskItem[]
  globals: IGlobals
}

export type AsyncCall<Payload, Response> = (payload: Payload) => Promise<Response>

export interface PlaceholderRequest {
  width: number
  height: number
  backgroundColor: string
  textColor: string
  text?: string
  format: 'png' | 'jpg' | 'webp'
}

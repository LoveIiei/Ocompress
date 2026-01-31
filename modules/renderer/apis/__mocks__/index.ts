import { sleep } from '../../../common/utils'
import { IImageFile, SaveType } from '../../../common/types'

export const fileAdd = async (files: string[]): Promise<IImageFile[]> => {
  await sleep(10)
  return files.map((file, index) => ({
    id: `mock-${index}`,
    url: `file://${file}`,
    size: 1000,
    ext: 'png' as const,
    originalName: file,
  }))
}

export const fileSelect = async (): Promise<IImageFile[]> => {
  await sleep(10)
  return []
}

export const fileSave = async (_images: IImageFile[], _type: SaveType): Promise<void> => {
  await sleep(10)
}

export const fileSaveAll = async (_type: SaveType): Promise<void> => {
  await sleep(10)
}

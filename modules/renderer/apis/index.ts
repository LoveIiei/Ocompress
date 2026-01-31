import { appStore } from '../stores/app-store.svelte'
import {
  IImageFile,
  SaveType,
  ITaskItem,
} from '../../common/types'
import { cleanupArray } from '../../common/utils'
import { imagineAPI } from '../../bridge/web'

/**
 * Add files by path (from drag-drop)
 */
export const fileAdd = async (files: string[]) => {
  const images = await imagineAPI.processFiles(files)
  return images
}

/**
 * Open file selection dialog
 */
export const fileSelect = async () => {
  const images = await imagineAPI.openFileDialog()
  return images
}

/**
 * Save specific images
 */
export const fileSave = async (images: IImageFile[], type: SaveType) => {
  await imagineAPI.saveFiles(images, type)
}

/**
 * Save all optimized images
 */
export const fileSaveAll = async (type: SaveType) => {
  const images = cleanupArray(
    appStore.tasks.map((task: ITaskItem) => task.optimized),
  )
  if (!images.length) return

  await fileSave(images, type)
}

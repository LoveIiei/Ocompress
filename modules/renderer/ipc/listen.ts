import { appStore } from '../stores/app-store.svelte'
import { SaveType } from '../../common/types'
import { showMessage } from '../components/Messager.svelte'
import * as apis from '../apis'
import __ from '../../locales'
import { imagineAPI } from '../../bridge/web'

export default async function listenIpc() {
  if (!imagineAPI) {
    console.warn('imagineAPI is missing')
    return
  }

  // Listen for files dropped (from drag-drop or second instance)
  await imagineAPI.onFilesDropped(async (filePaths: string[]) => {
    try {
      const images = await apis.fileAdd(filePaths)
      if (images.length > 0) {
        appStore.taskAdd(images)
      }
    } catch (err) {
      imagineAPI.logger.error('Failed to process dropped files:', err)
    }
  })

  // Listen for menu save events
  await imagineAPI.onMenuSave(async (saveType: SaveType) => {
    const activeId = appStore.globals.activeId
    let task = appStore.activeTask

    if (activeId && task && task.optimized) {
      await apis.fileSave([task.optimized], saveType)
      showMessage({
        message: __('save_success'),
        type: 'success',
      })
      return
    }

    await apis.fileSaveAll(saveType)
    showMessage({
      message: __('save_success'),
      type: 'success',
    })
  })

  // Listen for menu open events
  await imagineAPI.onMenuOpen(async () => {
    try {
      const images = await apis.fileSelect()
      if (images.length > 0) {
        appStore.taskAdd(images)
      }
    } catch (err) {
      imagineAPI.logger.error('Failed to open files:', err)
    }
  })
}

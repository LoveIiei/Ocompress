import * as storage from '../store/storage'
import {
  IImageFile,
  IOptimizeOptions,
  ITaskItem,
  TaskStatus,
  IUpdateInfo,
  SupportedExt,
  IGlobals,
  IDefaultOptions,
} from '../../common/types'
import { isTaskSizeIncreased } from '../../common/task'

export const createOptimizeOptions = (ext: SupportedExt): IOptimizeOptions => {
  const optimizeOptions: IOptimizeOptions = {
    exportExt: ext,
  }

  switch (ext) {
    case SupportedExt.jpg:
    case SupportedExt.webp:
      Object.assign(optimizeOptions, {
        quality: 80,
      })
      break

    case SupportedExt.png:
      Object.assign(optimizeOptions, {
        color: 128,
      })
      break

    default:
  }

  return optimizeOptions
}

function getInitialTaskOptions(exportExt: SupportedExt, defaultOptions: IDefaultOptions): IOptimizeOptions {
  return {
    ...(defaultOptions[exportExt] || createOptimizeOptions(exportExt)),
    exportExt,
  }
}

const savedOptions = storage.getOptions()

class AppStore {
  tasks = $state<ITaskItem[]>([])

  globals = $state<IGlobals>({
    optionsVisible: false,
    defaultOptions: {
      png: createOptimizeOptions(SupportedExt.png),
      jpg: createOptimizeOptions(SupportedExt.jpg),
      webp: createOptimizeOptions(SupportedExt.webp),
    },
    ...savedOptions,
  })

  placeholderVisible = $state(false)

  // Derived values (replaces reselect selectors)
  get activeTask(): ITaskItem | undefined {
    return this.tasks.find((t) => t.id === this.globals.activeId)
  }

  get sizeIncreaseCount(): number {
    return this.tasks.filter(isTaskSizeIncreased).length
  }

  // Actions (replaces redux-actions)
  taskAdd(images: IImageFile[]): void {
    const { defaultOptions } = this.globals

    const newTasks = images
      .filter((image) => !this.tasks.some((task) => task.id === image.id))
      .map<ITaskItem>((image) => {
        const exportExt = (
          defaultOptions[image.ext]?.exportExt || image.ext
        )

        return {
          id: image.id,
          image,
          options: getInitialTaskOptions(exportExt, defaultOptions),
          status: TaskStatus.PENDING,
        }
      })

    this.tasks = [...this.tasks, ...newTasks]
  }

  taskDelete(ids: string[]): void {
    this.tasks = this.tasks.filter((task) => !ids.includes(task.id))
  }

  taskClear(): void {
    this.tasks = []
  }

  taskClearIncreased(): void {
    this.tasks = this.tasks.filter((task) => !isTaskSizeIncreased(task))
  }

  taskUpdateOptions(id: string, options: IOptimizeOptions): void {
    const index = this.tasks.findIndex((task) => task.id === id)
    if (index === -1) return

    this.tasks[index] = {
      ...this.tasks[index],
      options,
      status: TaskStatus.PENDING,
    }
  }

  taskUpdateExport(id: string, exportExt: SupportedExt): void {
    const index = this.tasks.findIndex((task) => task.id === id)
    if (index === -1) return

    this.tasks[index] = {
      ...this.tasks[index],
      options: getInitialTaskOptions(exportExt, this.globals.defaultOptions),
      status: TaskStatus.PENDING,
    }
  }

  taskOptimizeStart(id: string): void {
    const index = this.tasks.findIndex((task) => task.id === id)
    if (index === -1) return

    this.tasks[index] = {
      ...this.tasks[index],
      status: TaskStatus.PROCESSING,
    }
  }

  taskOptimizeSuccess(id: string, optimized: IImageFile): void {
    const index = this.tasks.findIndex((task) => task.id === id)
    if (index === -1) return

    this.tasks[index] = {
      ...this.tasks[index],
      optimized,
      status: TaskStatus.DONE,
    }
  }

  taskOptimizeFail(id: string): void {
    const index = this.tasks.findIndex((task) => task.id === id)
    if (index === -1) return

    this.tasks[index] = {
      ...this.tasks[index],
      status: TaskStatus.FAIL,
    }
  }

  taskDetail(id: string | undefined): void {
    this.globals = {
      ...this.globals,
      activeId: id,
    }
  }

  optionsApply(): void {
    const { defaultOptions } = this.globals

    this.tasks = this.tasks.map((item) => {
      const { exportExt } = defaultOptions[item.image.ext]
      return {
        ...item,
        options: getInitialTaskOptions(exportExt ?? item.image.ext, defaultOptions),
        status: TaskStatus.PENDING,
      }
    })
  }

  appUpdateInfo(updateInfo: IUpdateInfo): void {
    this.globals = {
      ...this.globals,
      updateInfo,
    }
  }

  optionsVisible(visible: boolean): void {
    this.globals = {
      ...this.globals,
      optionsVisible: visible,
    }
  }

  setPlaceholderVisible(visible: boolean): void {
    this.placeholderVisible = visible
  }

  defaultOptions(ext: SupportedExt, options: IOptimizeOptions): void {
    const defaultOptions = {
      ...this.globals.defaultOptions,
      [ext]: options,
    }

    // Save to localStorage
    storage.saveOptions({ defaultOptions })

    this.globals = {
      ...this.globals,
      defaultOptions,
    }
  }
}

export const appStore = new AppStore()

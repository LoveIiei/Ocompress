import { debounce } from 'lodash'
import { TaskStatus } from '../../common/types'
import { appStore } from '../stores/app-store.svelte'
import { imagineAPI } from '../../bridge/web'

const maxRunningNum = Math.max((navigator.hardwareConcurrency || 2) - 1, 1)

export class JobRunner {
  private runningNum = 0
  private previousTasksLength = 0
  private intervalId: ReturnType<typeof setInterval> | null = null

  trigger = debounce(() => {
    if (this.runningNum >= maxRunningNum) return
    this.run()
  }, 100)

  start() {
    // Poll for changes since we can't use $effect outside of components
    this.intervalId = setInterval(() => {
      const currentLength = appStore.tasks.length
      const hasPending = appStore.tasks.some(t => t.status === TaskStatus.PENDING)

      if (hasPending || currentLength !== this.previousTasksLength) {
        this.trigger()
        this.previousTasksLength = currentLength
      }
    }, 200)

    // Initial trigger
    this.trigger()
  }

  stop() {
    if (this.intervalId) {
      clearInterval(this.intervalId)
      this.intervalId = null
    }
  }

  private pickPendingTask() {
    return appStore.tasks.find((task) => task.status === TaskStatus.PENDING)
  }

  private async run() {
    this.runningNum += 1

    // eslint-disable-next-line no-constant-condition
    while (true) {
      const task = this.pickPendingTask()

      if (!task) break

      try {
        appStore.taskOptimizeStart(task.id)
        const optimized = await imagineAPI.optimize({
          image: task.image,
          options: task.options,
        })
        appStore.taskOptimizeSuccess(task.id, optimized)
      } catch (err) {
        imagineAPI?.logger.error(err)
        appStore.taskOptimizeFail(task.id)
      }
    }

    this.runningNum -= 1
  }
}

export default JobRunner

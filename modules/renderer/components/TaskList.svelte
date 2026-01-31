<script lang="ts">
  import { onMount } from 'svelte'
  import { appStore } from '../stores/app-store.svelte'
  import TaskView from './TaskView.svelte'
  import {
    type ITaskItem,
    type IOptimizeOptions,
    SaveType,
    type SupportedExt,
  } from '../../common/types'
  import * as apis from '../apis'
  import __ from '../../locales'

  const LIST_MARGIN = 10
  const columnWidth = 260
  const rowHeight = columnWidth + 60

  let containerEl: HTMLDivElement | undefined = $state()
  let width = $state(0)
  let height = $state(0)
  let scrollTop = $state(0)

  const tasks = $derived(appStore.tasks)
  const columnCount = $derived(Math.max(1, Math.floor((width - LIST_MARGIN * 2) / columnWidth)))
  const rowCount = $derived(Math.ceil(tasks.length / columnCount))
  const startRow = $derived(Math.floor(scrollTop / rowHeight))
  const endRow = $derived(Math.min(startRow + Math.ceil(height / rowHeight) + 2, rowCount))

  const visibleItems = $derived(
    tasks
      .map((task, index) => ({ task, index }))
      .filter(({ index }) => {
        const row = Math.floor(index / columnCount)
        return row >= startRow && row < endRow
      })
  )

  function handleScroll(e: Event) {
    const target = e.target as HTMLElement
    scrollTop = target.scrollTop
  }

  function handleRemove(task: ITaskItem) {
    appStore.taskDelete([task.id])
  }

  function handleClick(task: ITaskItem) {
    appStore.taskDetail(task.id)
  }

  function handleOptionsChange(id: string, options: IOptimizeOptions) {
    appStore.taskUpdateOptions(id, options)
  }

  function handleExportChange(id: string, ext: SupportedExt) {
    appStore.taskUpdateExport(id, ext)
  }

  function handleSave(task: ITaskItem, type: SaveType) {
    if (task.optimized) {
      apis.fileSave([task.optimized], type)
    }
  }

  onMount(() => {
    if (containerEl) {
      const observer = new ResizeObserver(([entry]) => {
        width = entry.contentRect.width
        height = entry.contentRect.height
      })
      observer.observe(containerEl)
      return () => observer.disconnect()
    }
  })
</script>

<div
  class="task-list-container"
  bind:this={containerEl}
>
  {#if tasks.length}
    <div
      class="task-list"
      onscroll={handleScroll}
      style="height: {height}px; overflow: auto;"
    >
      <div
        class="task-list-inner"
        style="height: {rowCount * rowHeight + LIST_MARGIN * 2}px; position: relative;"
      >
        {#each visibleItems as { task, index } (task.id)}
          {@const row = Math.floor(index / columnCount)}
          {@const col = index % columnCount}
          <div
            class="task-item"
            style="
              position: absolute;
              top: {row * rowHeight + LIST_MARGIN}px;
              left: {col * columnWidth + LIST_MARGIN}px;
              width: {columnWidth}px;
              height: {rowHeight}px;
            "
          >
            <TaskView
              {task}
              onRemove={handleRemove}
              onClick={handleClick}
              onSave={handleSave}
              onOptionsChange={handleOptionsChange}
              onExportChange={handleExportChange}
            />
          </div>
        {/each}
      </div>
    </div>
  {:else}
    <div class="task-list task-empty">
      <span>{__('drag_files')}</span>
    </div>
  {/if}
</div>

<style>
  @keyframes task-empty {
    from {
      opacity: 0;
      transform: scale(1.1);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }

  .task-list-container {
    position: relative;
    flex: 1;
    height: 0;
  }

  .task-list {
    width: 100%;
    height: 100%;
  }

  .task-empty {
    display: block;
    color: #ccc;
    text-align: center;
    padding: 40px;
    padding-top: 200px;
    font-size: 30px;
  }

  .task-empty > span {
    animation: task-empty ease 0.2s;
    display: inline-block;
  }
</style>

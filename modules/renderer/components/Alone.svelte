<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { appStore } from '../stores/app-store.svelte'
  import ImageOptions from './ImageOptions.svelte'
  import ImageViewer from './ImageViewer.svelte'
  import Modal from './Modal.svelte'
  import Icon from './Icon.svelte'
  import SizeReduce from './SizeReduce.svelte'
  import RadioGroup from './RadioGroup.svelte'
  import TargetTypeSelect from './TargetTypeSelect.svelte'
  import {
    type IOptimizeOptions,
    TaskStatus,
    type IImageFile,
    type SupportedExt,
  } from '../../common/types'
  import __ from '../../locales'

  const ImageStage = {
    beforeOptimized: 'before_optimized',
    afterOptimized: 'after_optimized',
  } as const

  type ImageStageType = typeof ImageStage[keyof typeof ImageStage]

  const imageStageList = [ImageStage.beforeOptimized, ImageStage.afterOptimized]

  let imageStage = $state<ImageStageType>(ImageStage.afterOptimized)

  const task = $derived(appStore.activeTask)
  const exportExt = $derived(task ? (task.options.exportExt || task.image.ext) : undefined)

  const image = $derived<IImageFile | undefined>(
    task
      ? (imageStage === ImageStage.afterOptimized ? task.optimized : task.image)
      : undefined
  )

  function handleClose() {
    appStore.taskDetail(undefined)
  }

  function handleOptionsChange(options: IOptimizeOptions) {
    if (task) {
      appStore.taskUpdateOptions(task.id, options)
    }
  }

  function handleExtChange(ext: SupportedExt) {
    if (task) {
      appStore.taskUpdateExport(task.id, ext)
    }
  }

  function handleImageStageChange(value: ImageStageType) {
    imageStage = value
  }

  function handleKeyPress(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      handleClose()
    }
  }

  onMount(() => {
    window.addEventListener('keyup', handleKeyPress)
  })

  onDestroy(() => {
    window.removeEventListener('keyup', handleKeyPress)
  })
</script>

<Modal class="alone-modal" visible={!!task} onClose={handleClose}>
  <ImageViewer src={image?.url} />
  {#if task}
    <div class="alone-controls">
      {#if task.status === TaskStatus.PROCESSING || task.status === TaskStatus.PENDING}
        <Icon class="-spin icon-color" name="color" />
      {/if}
      <div class="size-reduce-container">
        <SizeReduce {task} />
      </div>
      <div class="paper alone-options">
        <TargetTypeSelect
          sourceExt={task.image.ext}
          targetExt={exportExt!}
          onChange={handleExtChange}
        />
        <ImageOptions
          ext={exportExt!}
          options={task.options}
          precision
          onChange={handleOptionsChange}
        />
      </div>
      <div class="original-check-container">
        <RadioGroup
          class="original-check -standard"
          data={imageStageList}
          value={imageStage}
          renderItem={__}
          onChange={handleImageStageChange}
        />
      </div>
    </div>
  {/if}
</Modal>

<style>
  :global(.alone-modal .backdrop) {
    height: 100%;
  }

  .size-reduce-container {
    position: absolute;
    right: 60px;
    bottom: 40px;
    font-size: 28px;
    font-weight: 200;
    text-shadow: 1px 1px #fff, -1px -1px #fff, -1px 1px #fff, 1px -1px #fff;
  }

  .size-reduce-container :global(.size-sep) {
    margin: 0 5px;
    color: #999;
  }

  .size-reduce-container :global(.size-less) {
    margin-left: 10px;
  }

  .size-reduce-container :global(.size-q) {
    color: #999;
  }

  .size-reduce-container :global(.size-unit) {
    font-size: 12px;
    color: #999;
    margin-left: 2px;
  }

  :global(.alone-modal .icon-color) {
    position: absolute;
    top: 50%;
    left: 50%;
    font-size: 30px;
    margin-left: -15px;
    margin-top: -15px;
  }

  .original-check-container {
    position: absolute;
    top: 20px;
    left: 50%;
    transform: translateX(-50%);
  }

  .alone-options {
    position: absolute;
    left: 0;
    bottom: 0;
    right: 0;
    padding: 5px;
    font-size: 12px;
    display: flex;
    align-items: center;
  }

  .alone-options :global(.percent-symbol) {
    display: none;
  }

  .alone-options :global(.image-options) {
    flex: 1;
    padding: 0 10px;
  }
</style>

<script lang="ts">
  import Popper from './Popper.svelte'
  import Icon from './Icon.svelte'
  import ImageOptions from './ImageOptions.svelte'
  import SizeReduce from './SizeReduce.svelte'
  import TargetTypeSelect from './TargetTypeSelect.svelte'
  import {
    type ITaskItem,
    TaskStatus,
    type IOptimizeOptions,
    SaveType,
    type SupportedExt,
  } from '../../common/types'
  import __ from '../../locales'

  interface Props {
    task: ITaskItem
    onRemove: (task: ITaskItem) => void
    onClick: (task: ITaskItem) => void
    onSave: (task: ITaskItem, type: SaveType) => void
    onOptionsChange: (id: string, options: IOptimizeOptions) => void
    onExportChange: (id: string, ext: SupportedExt) => void
  }

  let {
    task,
    onRemove,
    onClick,
    onSave,
    onOptionsChange,
    onExportChange,
  }: Props = $props()

  const image = $derived(task.image)
  const options = $derived(task.options)
  const destImage = $derived(task.optimized || task.image)
  const isProcessing = $derived(task.status === TaskStatus.PROCESSING)
  const exportExt = $derived(options.exportExt || image.ext)

  function stopEvent(e: MouseEvent) {
    e.preventDefault()
    e.stopPropagation()
  }

  function handleClear(e: MouseEvent) {
    stopEvent(e)
    onRemove(task)
  }

  function handleClick() {
    if (task.status === 'FAIL') return
    onClick(task)
  }

  function handleOptionsChange(opts: IOptimizeOptions) {
    onOptionsChange(task.id, opts)
  }

  function handleExtChange(ext: SupportedExt) {
    onExportChange(task.id, ext)
  }

  function handleSave(e: MouseEvent, type: SaveType) {
    stopEvent(e)
    onSave(task, type)
  }

  function handleRefresh(e: MouseEvent) {
    stopEvent(e)
    onOptionsChange(task.id, task.options)
  }
</script>

<div class="task-view -{task.status}">
  <div
    class="image-view"
    onclick={handleClick}
    role="button"
    tabindex="0"
    onkeydown={(e) => e.key === 'Enter' && handleClick()}
  >
    <img
      src={destImage.url}
      loading="lazy"
      alt="task-cover"
    />
    <div class="image-view-menu">
      <Popper hoverMode>
        {#snippet popper()}
          <div class="popper-body">{task.status}</div>
        {/snippet}
        <span class="traffic">
          <Icon
            name={isProcessing ? 'color' : 'dot'}
            class={isProcessing ? '-spin' : ''}
          />
        </span>
      </Popper>

      {#if task.status === TaskStatus.FAIL}
        <button type="button" onclick={handleRefresh}>
          <Icon name="refresh" />
        </button>
      {/if}

      <Popper hoverMode>
        {#snippet popper()}
          <div class="popper-menu">
            <button
              type="button"
              onclick={(e) => handleSave(e, SaveType.OVER)}
            >
              {__('save')}
            </button>
            <button
              type="button"
              onclick={(e) => handleSave(e, SaveType.SAVE_AS)}
            >
              {__('save_as')}
            </button>
          </div>
        {/snippet}
        <button type="button" onclick={stopEvent}>
          <Icon name="down" />
        </button>
      </Popper>

      <span class="blank"></span>

      <button type="button" class="close" onclick={handleClear}>
        <Icon name="clear" />
      </button>
    </div>
  </div>
  <div class="image-profile">
    <ImageOptions
      ext={exportExt}
      {options}
      precision={false}
      onChange={handleOptionsChange}
    />
    <div>
      <div class="image-sizes">
        <TargetTypeSelect
          sourceExt={image.ext}
          targetExt={exportExt}
          onChange={handleExtChange}
        />
        <SizeReduce {task} />
      </div>
    </div>
  </div>
</div>

<style>
  .task-view {
    position: relative;
    box-sizing: border-box;
    background: #fff;
    box-shadow: 0 0 4px rgba(0, 0, 0, 0.2);
    overflow: hidden;
    border-radius: 5px;
    transition: box-shadow 0.4s;
    margin: 10px;
  }

  .task-view:hover {
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.2);
    background: #fafafa;
  }

  .image-view {
    position: relative;
    border-bottom: 1px solid #eee;
    cursor: pointer;
  }

  .image-view::after {
    content: '';
    padding-top: 100%;
    display: block;
  }

  .image-view img {
    position: absolute;
    max-width: 100%;
    max-height: 100%;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  .task-view :global(.image-options) {
    border-bottom: 1px solid #eee;
    padding: 0 10px;
  }

  .task-view :global(.image-options .color-number) {
    padding: 0 8px;
  }

  .task-view :global(.image-options input[type="number"]) {
    width: 24px;
    text-align: right;
    margin-left: 5px;
  }

  .task-view :global(.image-options input[type="number"]::-webkit-inner-spin-button),
  .task-view :global(.image-options input[type="number"]::-webkit-outer-spin-button) {
    display: none;
  }

  .image-profile {
    font-size: 12px;
  }

  .image-sizes {
    display: flex;
    padding: 6px 8px;
    align-items: center;
    font-size: 11px;
  }

  .task-view :global(.size-reduce) {
    flex: 1;
    text-align: right;
  }

  .task-view :global(.size-reduce .size-less) {
    color: #0c0;
    margin-left: 10px;
  }

  .task-view :global(.size-reduce .size-less.-increased) {
    color: #f60;
  }

  .task-view.-PENDING .traffic {
    color: #FDD835;
  }

  .task-view.-PROCESSING .traffic :global(.icon) {
    font-size: 2em;
  }

  .task-view.-DONE .traffic {
    color: #36CD40;
  }

  .task-view.-FAIL .image-view {
    cursor: default;
  }

  .task-view.-FAIL .traffic {
    color: #e36;
  }

  .task-view:hover .image-view-menu {
    background: rgba(255, 255, 255, 0.9);
  }

  .image-view-menu {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 30px;
    background: rgba(255, 255, 255, 0.3);
    display: flex;
    color: #555;
    font-size: 18px;
    transition: background 0.4s;
  }

  .image-view-menu button {
    background: none;
    border: 0;
    padding: 0;
    cursor: pointer;
    font-size: inherit;
  }

  .image-view-menu > :global(span),
  .image-view-menu > :global(button),
  .image-view-menu > :global(div) {
    width: 30px;
    display: flex;
    justify-content: center;
    align-items: center;
    color: currentColor;
  }

  .image-view-menu > :global(a:hover) {
    background: #669;
    color: #fff;
  }

  .blank {
    flex: 1;
  }

  .close {
    box-sizing: border-box;
    width: 30px;
    height: 30px;
    transition: 0.1s;
    color: #555;
    border-radius: 0 0 0 5px;
  }

  .close:hover {
    background-color: #c36;
    color: #fff;
  }

  .traffic {
    font-size: 10px;
    color: transparent;
    transition: 0.2s;
    transition-property: transform;
  }

  .traffic :global(.icon) {
    display: block;
  }
</style>

<script lang="ts">
  import { appStore } from './stores/app-store.svelte'
  import TaskList from './components/TaskList.svelte'
  import ActionBar from './components/ActionBar.svelte'
  import Alone from './components/Alone.svelte'
  import PlaceholderGenerator from './components/PlaceholderGenerator.svelte'
  import { prevent } from './utils/dom-event'
  import * as apis from './apis'
  import './images/symbols'

  const placeholderVisible = $derived(appStore.placeholderVisible)

  function handlePlaceholderClose() {
    appStore.setPlaceholderVisible(false)
  }

  let onion = $state(0)

  function handleDragEnter() {
    onion++
  }

  function handleDragLeave() {
    onion--
  }

  async function handleDragDrop(e: DragEvent) {
    e.preventDefault()
    onion = 0

    if (!e.dataTransfer) return

    const files = Array.from(e.dataTransfer.files)
      .filter((file) => !file.type || /png|jpeg|webp/.test(file.type))
      .map((file) => (file as any).path as string)

    try {
      const images = await apis.fileAdd(files)
      if (images.length > 0) {
        appStore.taskAdd(images)
      }
    } catch (err) {
      console.error('Failed to add files:', err)
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault()
  }
</script>

<div
  class="layout"
  class:-drag={!!onion}
  ondragover={handleDragOver}
  ondragenter={handleDragEnter}
  ondragleave={handleDragLeave}
  ondrop={handleDragDrop}
  role="application"
>
  <ActionBar />
  <TaskList />
</div>
<Alone />
<PlaceholderGenerator visible={placeholderVisible} onClose={handlePlaceholderClose} />

<style>
  :global(body) {
    margin: 0;
    background: #fafaf9;
    font-family: -apple-system, "Helvetica Neue", Helvetica, "Nimbus Sans L", Arial, "Liberation Sans", "PingFang SC", "Hiragino Sans GB", "Source Han Sans CN", "Source Han Sans SC", "Microsoft YaHei", "Wenquanyi Micro Hei", "WenQuanYi Zen Hei", "ST Heiti", SimHei, "WenQuanYi Zen Hei Sharp", sans-serif;
  }

  :global(.paper) {
    background: #fff;
    box-shadow: 0 0 6px rgba(0, 0, 0, 0.3);
  }

  :global(.fn-hide) {
    display: none !important;
  }

  :global(.ellipsis) {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  :global(.clearfix::after) {
    display: block;
    content: '';
    clear: both;
  }

  :global(:focus:not(:focus-visible)) {
    outline: none;
  }

  .layout {
    position: absolute;
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
  }

  .layout.-drag::after {
    content: '';
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    border: 0 solid #9cf;
    box-shadow: inset 0 0 60px #0cf;
    z-index: 100;
    pointer-events: none;
  }
</style>

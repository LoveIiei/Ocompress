<script lang="ts">
  import { appStore } from '../stores/app-store.svelte'
  import Icon from './Icon.svelte'
  import Popper from './Popper.svelte'
  import OptionsPanel from './OptionsPanel.svelte'
  import { SaveType } from '../../common/types'
  import * as apis from '../apis'
  import __ from '../../locales'
  import pkg from '../../../package.json'
  import { isTaskSizeIncreased } from '../../common/task'
  import { showMessage } from './Messager.svelte'
  import { imagineAPI } from '../../bridge/web'

  const count = $derived(appStore.tasks.length)
  const updateInfo = $derived(appStore.globals.updateInfo)
  const optionsVisible = $derived(appStore.globals.optionsVisible)
  const sizeIncreaseCount = $derived(
    appStore.tasks.reduce(
      (cnt, item) => cnt + (isTaskSizeIncreased(item) ? 1 : 0),
      0
    )
  )

  function handleOptionsVisibleClick() {
    appStore.optionsVisible(!optionsVisible)
  }

  function handleOptionsHide() {
    appStore.optionsVisible(false)
  }

  function onRemoveAll() {
    appStore.taskClear()
  }

  function onRemoveIncreased() {
    appStore.taskClearIncreased()
  }

  async function onAdd() {
    try {
      const images = await apis.fileSelect()
      if (images.length > 0) {
        appStore.taskAdd(images)
      }
    } catch (err) {
      console.error('Failed to select files:', err)
    }
  }

  async function onSave(type: SaveType) {
    try {
      await apis.fileSaveAll(type)
      showMessage({
        message: __('save_success'),
        type: 'success',
      })
    } catch (err) {
      console.error('Failed to save files:', err)
    }
  }

  function onUpdateClick() {
    imagineAPI.openExternal(`${pkg.homepage}/releases`)
  }

  function onPlaceholderClick() {
    appStore.setPlaceholderVisible(true)
  }
</script>

<div class="action-bar">
  <button type="button" onclick={onAdd}>
    <Icon name="add" />
    <span class="ellipsis">{__('add')}</span>
  </button>

  <Popper hoverMode>
    {#snippet popper()}
      <div class="popper-menu">
        <button type="button" onclick={() => onSave(SaveType.OVER)}>
          {__('save_cover')}
        </button>
        <button type="button" onclick={() => onSave(SaveType.NEW_NAME)}>
          {__('save_new')}
        </button>
        <button type="button" onclick={() => onSave(SaveType.NEW_DIR)}>
          {__('save_dir')}
        </button>
      </div>
    {/snippet}
    <button type="button" disabled={!count}>
      <div>
        <Icon name="down" />
        <span class="ellipsis">{__('save')}</span>
      </div>
      <Icon name="expand-more" class="expand" />
    </button>
  </Popper>

  <Popper hoverMode>
    {#snippet popper()}
      <div class="popper-menu">
        <button type="button" onclick={onRemoveAll}>
          {__('clear')}
        </button>
        <button type="button" onclick={onRemoveIncreased} disabled={!sizeIncreaseCount}>
          {__('clear_increased')} ({sizeIncreaseCount})
        </button>
      </div>
    {/snippet}
    <button type="button" disabled={!count}>
      <div>
        <Icon name="delete" />
        <span class="ellipsis">{__('clear')}</span>
      </div>
      <Icon name="expand-more" class="expand" />
      {#if sizeIncreaseCount}
        <i class="dot"></i>
      {/if}
    </button>
  </Popper>

  <button type="button" onclick={onPlaceholderClick}>
    <Icon name="placeholder" />
    <span class="ellipsis">{__('placeholder')}</span>
  </button>

  {#if updateInfo}
    <button type="button" onclick={onUpdateClick} class="has-update">
      <Icon name="up" />
      <span class="ellipsis">{__('new_version')}</span>
    </button>
  {/if}

  <div class="blank"></div>

  <Popper class="options-popper" visible={optionsVisible}>
    {#snippet popper()}
      <OptionsPanel onApplyClick={handleOptionsHide} />
    {/snippet}
    <button
      type="button"
      class:active={optionsVisible}
      onclick={handleOptionsVisibleClick}
    >
      <Icon name="tune" />
    </button>
  </Popper>
</div>

<style>
  .action-bar {
    flex-shrink: 0;
    display: flex;
    height: 60px;
    padding-left: 10px;
    background: #fff;
    box-shadow: 0 0 4px rgba(0, 0, 0, 0.2);
    position: relative;
    z-index: 1;
  }

  .action-bar > .blank {
    flex: 1;
  }

  .action-bar :global(button) {
    position: relative;
    background: none;
    border: 0;
    outline: none;
    padding: 0 20px;
    min-width: 70px;
    max-width: 120px;
    transition: 0.2s;
  }

  .action-bar :global(button.active),
  .action-bar :global(button.-active) {
    box-shadow: inset 1px 1px 2px rgba(0, 0, 0, 0.1);
  }

  .action-bar :global(button:disabled .icon),
  .action-bar :global(button:disabled span) {
    color: #ccc;
  }

  .action-bar :global(button:not(:disabled)) {
    cursor: pointer;
  }

  .action-bar :global(button:not(:disabled):hover .icon),
  .action-bar :global(button:not(:disabled):hover span) {
    color: #000;
  }

  .action-bar :global(.icon),
  .action-bar span {
    transition: all 0.2s;
  }

  .action-bar :global(.icon) {
    height: 20px;
    width: 20px;
    margin: 2px 0;
    color: #666;
  }

  .dot {
    position: absolute;
    width: 8px;
    height: 8px;
    border-radius: 8px;
    background: #fa0;
    right: 5px;
    top: 15px;
  }

  .action-bar :global(.expand) {
    position: absolute;
    right: 0;
    bottom: 4px;
  }

  .action-bar span {
    display: block;
    color: #888;
  }

  .has-update span,
  .has-update :global(.icon) {
    color: #f60;
  }

  :global(.options-popper) {
    border-radius: 0;
  }
</style>

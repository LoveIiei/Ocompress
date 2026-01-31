<script lang="ts">
  import { appStore } from '../stores/app-store.svelte'
  import { SupportedExt, type IOptimizeOptions } from '../../common/types'
  import Icon from './Icon.svelte'
  import Collapse from './Collapse.svelte'
  import ImageOptions from './ImageOptions.svelte'
  import TargetTypeSelect from './TargetTypeSelect.svelte'
  import __ from '../../locales'

  interface Props {
    onApplyClick?: () => void
  }

  let { onApplyClick }: Props = $props()

  const optionsMap = $derived(appStore.globals.defaultOptions)

  function handleOptionsChange(ext: SupportedExt, options: IOptimizeOptions) {
    appStore.defaultOptions(ext, options)
  }

  function handleExtChange(ext: SupportedExt, exportExt: SupportedExt) {
    appStore.defaultOptions(ext, {
      ...optionsMap[ext],
      exportExt,
    })
  }

  function handleApplyClick() {
    appStore.optionsApply()
    onApplyClick?.()
  }

  function handleClose() {
    appStore.optionsVisible(false)
  }
</script>

<div class="options">
  <div class="options-body">
    <Collapse title="PNG" initialVisible>
      <div class="collapse-row target-ext-select-row">
        <TargetTypeSelect
          class="target-ext-select"
          onChange={(ext) => handleExtChange(SupportedExt.png, ext)}
          sourceExt={SupportedExt.png}
          targetExt={optionsMap.png.exportExt || SupportedExt.png}
        />
      </div>
      <div class="collapse-row">
        <ImageOptions
          precision
          ext={SupportedExt.png}
          options={optionsMap.png}
          onChange={(opts) => handleOptionsChange(SupportedExt.png, opts)}
        />
      </div>
    </Collapse>

    <Collapse title="JPEG" initialVisible>
      <div class="collapse-row target-ext-select-row">
        <TargetTypeSelect
          class="target-ext-select"
          onChange={(ext) => handleExtChange(SupportedExt.jpg, ext)}
          sourceExt={SupportedExt.jpg}
          targetExt={optionsMap.jpg.exportExt || SupportedExt.jpg}
        />
      </div>
      <div class="collapse-row">
        <ImageOptions
          precision
          ext={SupportedExt.jpg}
          options={optionsMap.jpg}
          onChange={(opts) => handleOptionsChange(SupportedExt.jpg, opts)}
        />
      </div>
    </Collapse>

    <Collapse title="WebP" initialVisible>
      <div class="collapse-row">
        <ImageOptions
          precision
          ext={SupportedExt.webp}
          options={optionsMap.webp}
          onChange={(opts) => handleOptionsChange(SupportedExt.webp, opts)}
        />
      </div>
    </Collapse>
  </div>
  <footer class="clearfix">
    <button type="button" onclick={handleApplyClick}>
      <Icon name="doneall" />
      {__('apply_now')}
    </button>
    <div class="blank"></div>
    <button type="button" onclick={handleClose}>
      <Icon name="close" />
    </button>
  </footer>
</div>

<style>
  .options-body {
    width: 500px;
    padding: 10px;
  }

  .options-body :global(.percent-symbol) {
    display: none;
  }

  .options-body :global(.collapse-title) {
    letter-spacing: 2px;
  }

  .options > footer {
    display: flex;
    padding: 0;
    background: #fafafa;
    border-top: 1px solid #eee;
  }

  .options > footer > .blank {
    flex: 1;
  }

  footer button {
    padding: 8px 10px;
    border-radius: 2px;
    font-size: 12px;
    outline: 0;
    border: 0;
    background: transparent;
    color: #667;
    line-height: 14px;
    transition: 0.4s;
    cursor: pointer;
  }

  footer button :global(.icon-doneall) {
    font-size: 14px;
    margin-right: 4px;
    vertical-align: top;
  }

  footer button:hover,
  footer button:active {
    color: #000;
    background: #eee;
  }

  footer button :global(.icon-close) {
    font-size: 12px;
    display: block;
  }

  .target-ext-select-row {
    position: relative;
  }

  :global(.target-ext-select) {
    position: absolute;
    bottom: 11px;
    right: 0;
  }
</style>

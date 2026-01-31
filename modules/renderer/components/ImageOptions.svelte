<script lang="ts">
  import { SupportedExt, type IOptimizeOptions } from '../../common/types'
  import ColorNumber from './ColorNumber.svelte'
  import Quality from './Quality.svelte'
  import __ from '../../locales'

  interface Props {
    ext: SupportedExt
    options: IOptimizeOptions
    precision: boolean
    onChange: (options: IOptimizeOptions) => void
  }

  let { ext, options, precision, onChange }: Props = $props()

  function handleColorChange(color: number) {
    onChange({ ...options, color })
  }

  function handleQualityChange(quality: number) {
    onChange({ ...options, quality })
  }

  const nativeStep = $derived(precision ? 0.1 : 1)
</script>

{#if ext === SupportedExt.png}
  <div class="image-options">
    <div>{__('colors')}</div>
    <ColorNumber
      value={options.color || 0}
      onChange={handleColorChange}
      {nativeStep}
    />
  </div>
{:else if ext === SupportedExt.jpg || ext === SupportedExt.webp}
  <div class="image-options">
    <div>{__('quality')}</div>
    <Quality
      value={options.quality || 0}
      onChange={handleQualityChange}
      {nativeStep}
    />
    <span class="percent-symbol">%</span>
  </div>
{/if}

<style>
  .image-options {
    display: flex;
    align-items: center;
  }

  .image-options > div:first-child {
    margin-right: 10px;
  }

  .image-options :global(.ranger) {
    flex: 1;
  }
</style>

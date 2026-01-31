<script lang="ts">
  import { SupportedExt } from '../../common/types'
  import Select from './Select.svelte'

  interface Props {
    class?: string
    sourceExt: SupportedExt
    targetExt: SupportedExt
    onChange: (ext: SupportedExt) => void
  }

  let {
    class: className = '',
    sourceExt,
    targetExt,
    onChange,
  }: Props = $props()

  function handleChange(e: Event) {
    const select = e.target as HTMLSelectElement
    onChange(select.value as SupportedExt)
  }
</script>

<Select
  class="select-ext-type {className}"
  value={targetExt}
  onchange={handleChange}
>
  <option value={SupportedExt.png}>To PNG</option>
  <option value={SupportedExt.jpg}>To JPG</option>
  <option value={SupportedExt.webp}>To WebP</option>
  <option value={SupportedExt.ico}>To ICO</option>
  <option value={SupportedExt.bmp}>To BMP</option>
  <option value={SupportedExt.gif}>To GIF</option>
</Select>

<style>
  :global(.select-ext-type) {
    font-size: 11px;
    width: 64px;
  }
</style>

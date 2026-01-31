<script lang="ts">
  import type { ITaskItem } from '../../common/types'
  import * as utils from '../../common/utils'

  interface Props {
    task: ITaskItem
  }

  let { task }: Props = $props()

  const beforeSize = $derived(utils.size(task.image.size))
  const afterSize = $derived(task.optimized ? utils.size(task.optimized.size) : null)
  const percent = $derived(
    task.optimized
      ? utils.percent((task.image.size - task.optimized.size) / task.image.size)
      : 0
  )
</script>

<div class="size-reduce">
  {#if task.optimized && afterSize}
    <span class="size-number">{afterSize[0]}</span>
    <span class="size-unit">{afterSize[1]}</span>
    <span class="size-sep"> / </span>
    <span class="size-number">{beforeSize[0]}</span>
    <span class="size-unit">{beforeSize[1]}</span>
    <span class="size-less" class:-decreased={percent > 0} class:-increased={percent <= 0}>
      {percent >= 0 ? '-' : '+'}
      <span class="size-number">{Math.abs(percent)}</span>
      <span class="size-unit">%</span>
    </span>
  {:else}
    <span class="size-number">{beforeSize[0]}</span>
    <span class="size-unit">{beforeSize[1]}</span>
  {/if}
</div>

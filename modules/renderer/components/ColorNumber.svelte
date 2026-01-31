<script lang="ts">
  import Ranger from './Ranger.svelte'

  interface Props {
    value: number
    nativeStep: number
    onChange: (value: number) => void
  }

  let { value, nativeStep, onChange }: Props = $props()

  const transformInput = (val: number) => Math.log2(val)
  const transformOutput = (val: number) => 2 ** val
</script>

<Ranger
  min={2}
  max={256}
  {value}
  {nativeStep}
  {onChange}
  {transformInput}
  {transformOutput}
/>

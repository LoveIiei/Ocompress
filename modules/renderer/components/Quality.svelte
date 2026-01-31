<script lang="ts">
  import Ranger from './Ranger.svelte'

  interface Props {
    value: number
    nativeStep: number
    onChange: (value: number) => void
  }

  let { value, nativeStep, onChange }: Props = $props()

  const transformInput = (val: number) => val / 10
  const transformOutput = (val: number) => val * 10
</script>

<Ranger
  min={10}
  max={100}
  {value}
  {nativeStep}
  {onChange}
  {transformInput}
  {transformOutput}
/>

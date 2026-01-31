<script lang="ts">
  import { coop2 } from '../../common/utils'

  interface Props {
    value: number
    max: number
    min: number
    nativeStep: number
    inputReadOnly?: boolean
    transformInput?: (value: number) => number
    transformOutput?: (value: number) => number
    onChange: (value: number) => void
  }

  const pass = (x: number) => x

  let {
    value,
    max,
    min,
    nativeStep,
    inputReadOnly = false,
    transformInput = pass,
    transformOutput = pass,
    onChange,
  }: Props = $props()

  function fixValue(val: number): number {
    return Math.round(coop2(min, max, val))
  }

  function handleInputChange(e: Event) {
    const input = e.target as HTMLInputElement
    onChange(fixValue(transformOutput(Number(input.value))))
  }

  function handleNumberInputChange(e: Event) {
    const input = e.target as HTMLInputElement
    onChange(fixValue(Number(input.value)))
  }

  const nativeValue = $derived(transformInput(value))
  const nativeMin = $derived(transformInput(min))
  const nativeMax = $derived(transformInput(max))
</script>

<div class="ranger">
  <input
    type="range"
    value={nativeValue}
    min={nativeMin}
    max={nativeMax}
    step={nativeStep}
    oninput={handleInputChange}
  />
  <input
    type="number"
    readonly={inputReadOnly}
    {value}
    oninput={handleNumberInputChange}
  />
</div>

<style>
  .ranger {
    box-sizing: border-box;
    display: flex;
    height: 25px;
    align-items: center;
  }

  .ranger input[type="range"] {
    flex: 1;
    -webkit-appearance: none;
    background: none;
    outline: 0;
    margin: 0;
  }

  .ranger input[type="range"]::-webkit-slider-runnable-track {
    -webkit-appearance: none;
    width: 100%;
    height: 3px;
    border: 0;
    cursor: pointer;
    background: #B2DFDB;
    border-radius: 1.3px;
    outline: 0;
  }

  .ranger input[type="range"]::-webkit-slider-thumb {
    -webkit-appearance: none;
    border: 0;
    outline: 0;
    background: #00897B;
    width: 10px;
    height: 10px;
    border-radius: 10px;
    margin-top: -3px;
    cursor: pointer;
  }

  .ranger input[type="number"] {
    width: 40px;
    outline: 0;
    border: 0;
    margin-left: 10px;
    font-size: 12px;
    background: none;
  }

  .ranger input[type="number"]::-webkit-inner-spin-button,
  .ranger input[type="number"]::-webkit-outer-spin-button {
    opacity: 1;
  }
</style>

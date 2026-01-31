<script lang="ts">
  interface Props {
    value: string
    onChange: (value: string) => void
    class?: string
  }

  let {
    value,
    onChange,
    class: className = '',
  }: Props = $props()

  function handleInput(e: Event) {
    const target = e.target as HTMLInputElement
    onChange(target.value)
  }
</script>

<div class="color-picker {className}">
  <input
    type="color"
    {value}
    oninput={handleInput}
  />
  <span class="color-value">{value.toUpperCase()}</span>
</div>

<style>
  .color-picker {
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }

  .color-picker input[type="color"] {
    -webkit-appearance: none;
    appearance: none;
    width: 32px;
    height: 32px;
    border: 1px solid #ccc;
    border-radius: 4px;
    cursor: pointer;
    padding: 0;
    background: none;
  }

  .color-picker input[type="color"]::-webkit-color-swatch-wrapper {
    padding: 2px;
  }

  .color-picker input[type="color"]::-webkit-color-swatch {
    border: none;
    border-radius: 2px;
  }

  .color-value {
    font-family: monospace;
    font-size: 13px;
    color: #666;
  }
</style>

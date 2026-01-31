<script lang="ts" generics="T">
  interface Props {
    value: T
    data: T[]
    onChange: (value: T) => void
    renderItem?: (item: T) => string
    itemValue?: (item: T) => T
    class?: string
  }

  let {
    value,
    data,
    onChange,
    renderItem = (item: T) => String(item),
    itemValue = (item: T) => item,
    class: className = '-standard',
  }: Props = $props()

  function handleClick(val: T) {
    onChange(val)
  }
</script>

<div class="radio-group {className}">
  {#each data as item (itemValue(item))}
    {@const val = itemValue(item)}
    <div
      class="radio-item"
      class:-checked={val === value}
      onclick={() => handleClick(val)}
      role="button"
      tabindex="0"
      onkeydown={(e) => e.key === 'Enter' && handleClick(val)}
    >
      {renderItem(item)}
    </div>
  {/each}
</div>

<style>
  .radio-group.-standard {
    display: inline-flex;
    font-size: 12px;
  }

  .radio-group.-standard .radio-item {
    position: relative;
    padding: 3px 6px;
    background: #fff;
    cursor: pointer;
    transition: 0.2s;
    box-shadow: 0 0 3px rgba(0, 0, 0, 0.2);
  }

  .radio-group.-standard .radio-item:first-child {
    border-radius: 5px 0 0 5px;
  }

  .radio-group.-standard .radio-item:last-child {
    border-radius: 0 5px 5px 0;
  }

  .radio-group.-standard .radio-item.-checked {
    z-index: 1;
    cursor: default;
    background: #09f;
    color: #fff;
  }
</style>

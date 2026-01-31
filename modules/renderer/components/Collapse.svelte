<script lang="ts">
  import type { Snippet } from 'svelte'

  interface Props {
    title: string
    initialVisible?: boolean
    children?: Snippet
  }

  let { title, initialVisible = false, children }: Props = $props()
  let visible = $state(initialVisible)

  function handleClick() {
    visible = !visible
  }
</script>

<div class="collapse" class:-hide={!visible}>
  <h3
    class="collapse-title"
    onclick={handleClick}
    role="button"
    tabindex="0"
    onkeydown={(e) => e.key === 'Enter' && handleClick()}
  >
    {title}
  </h3>
  <div class="collapse-content">
    {#if children}
      {@render children()}
    {/if}
  </div>
</div>

<style>
  .collapse {
    margin-bottom: 15px;
  }

  .collapse-title {
    position: relative;
    font-weight: normal;
    font-size: 12px;
    padding: 3px;
    padding-left: 16px;
    transition: 0.1s;
    margin: 10px 0;
    color: #999;
    font-weight: 100;
    cursor: pointer;
  }

  .collapse-title:hover {
    color: #000;
  }

  .collapse-title::before {
    content: '';
    position: absolute;
    width: 0;
    height: 0;
    border: 4px solid transparent;
    border-left-color: currentColor;
    top: 50%;
    left: 6px;
    margin-top: -4px;
    transform: rotate(45deg);
    transform-origin: 1px 50%;
    transition: transform 0.2s;
  }

  .collapse-content {
    padding-left: 4px;
    color: #666;
  }

  :global(.collapse-row) {
    margin: 5px;
  }

  .collapse.-hide .collapse-content {
    display: none;
  }

  .collapse.-hide .collapse-title::before {
    transform: rotate(0);
  }
</style>

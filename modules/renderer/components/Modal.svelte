<script lang="ts">
  import { fade, fly } from 'svelte/transition'
  import Portal from './Portal.svelte'
  import Icon from './Icon.svelte'
  import type { Snippet } from 'svelte'

  interface Props {
    visible: boolean
    class?: string
    onClose?: () => void
    children?: Snippet
  }

  let {
    visible,
    class: className = '',
    onClose,
    children,
  }: Props = $props()
</script>

<Portal>
  <div class="modal-container {className}">
    {#if visible}
      <div
        class="modal"
        transition:fly={{ y: 20, duration: 200 }}
      >
        <button
          type="button"
          class="close"
          onclick={onClose}
        >
          <Icon name="close" />
        </button>
        {#if children}
          {@render children()}
        {/if}
      </div>
    {/if}
  </div>
</Portal>

<style>
  .modal-container {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    z-index: 200;
  }

  .modal {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: #fff;
    z-index: 200;
    pointer-events: auto;
  }

  .close {
    position: absolute;
    top: 10px;
    right: 10px;
    border: 0;
    outline: 0;
    padding: 10px;
    background: none;
    z-index: 10;
    color: #666;
    cursor: pointer;
    transition: color 0.3s;
  }

  .close:hover {
    color: #0cf;
  }

  .close :global(.icon) {
    font-size: 24px;
  }
</style>

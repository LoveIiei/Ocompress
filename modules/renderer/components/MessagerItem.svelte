<script lang="ts">
  import { fly } from 'svelte/transition'
  import Icon from './Icon.svelte'

  interface Props {
    message: string
    type?: 'error' | 'warning' | 'success' | 'info'
    visible?: boolean
  }

  let { message, type = 'info', visible = true }: Props = $props()

  const iconName = $derived(type === 'warning' ? 'error' : type)
</script>

{#if visible}
  <div class="messager -{type}" transition:fly={{ y: -50, duration: 300 }}>
    <Icon class="messager-icon" name={iconName} />
    {message}
  </div>
{/if}

<style>
  :global(.global-messager) {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 1000;
  }

  :global(.global-messager .messager) {
    position: absolute;
    width: 100%;
  }

  .messager {
    position: relative;
    padding: 6px 45px;
    font-size: 12px;
  }

  .messager.-info {
    background: #aef;
  }

  .messager.-info :global(.messager-icon) {
    color: #09f;
  }

  .messager.-warning {
    background: #fe9;
  }

  .messager.-warning :global(.messager-icon) {
    color: #f60;
  }

  .messager.-error {
    background: #e66;
    color: #fff;
  }

  .messager.-success {
    background: #cfc;
  }

  .messager.-success :global(.messager-icon) {
    color: #0c0;
  }

  :global(.messager-icon) {
    position: absolute;
    left: 20px;
    top: 6px;
    font-size: 16px;
  }
</style>

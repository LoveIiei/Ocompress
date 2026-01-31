<script lang="ts">
  import type { Snippet } from 'svelte'

  interface Props {
    children?: Snippet
  }

  let { children }: Props = $props()

  function portal(node: HTMLElement) {
    const target = document.body
    target.appendChild(node)

    return {
      destroy() {
        node.remove()
      }
    }
  }
</script>

<div use:portal style="display: contents;">
  {#if children}
    {@render children()}
  {/if}
</div>

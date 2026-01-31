<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { computePosition, flip, offset, shift, arrow, autoUpdate, type Placement } from '@floating-ui/dom'
  import Portal from './Portal.svelte'
  import type { Snippet } from 'svelte'

  interface Props {
    popper: Snippet
    placement?: Placement
    hoverMode?: boolean
    visible?: boolean
    class?: string
    children: Snippet
  }

  let {
    popper,
    placement = 'bottom',
    hoverMode = false,
    visible: propVisible = false,
    class: className = '',
    children,
  }: Props = $props()

  let referenceEl: HTMLElement | undefined = $state()
  let floatingEl: HTMLElement | undefined = $state()
  let arrowEl: HTMLElement | undefined = $state()
  let hoverVisible = $state(false)
  let enterTimer = -1
  let leaveTimer = -1
  let cleanup: (() => void) | undefined

  const isVisible = $derived(hoverMode ? hoverVisible : propVisible)

  function onmouseover() {
    clearTimeout(leaveTimer)
    enterTimer = window.setTimeout(() => {
      hoverVisible = true
    }, 200)
  }

  function onmouseleave() {
    clearTimeout(enterTimer)
    leaveTimer = window.setTimeout(() => {
      hoverVisible = false
    }, 200)
  }

  $effect(() => {
    if (!referenceEl || !floatingEl) return

    cleanup = autoUpdate(referenceEl, floatingEl, async () => {
      if (!referenceEl || !floatingEl) return

      const result = await computePosition(referenceEl, floatingEl, {
        placement,
        middleware: [
          offset(8),
          flip(),
          shift(),
          arrowEl ? arrow({ element: arrowEl }) : undefined,
        ].filter(Boolean),
      })

      floatingEl.style.left = `${result.x}px`
      floatingEl.style.top = `${result.y}px`

      if (arrowEl && result.middlewareData.arrow) {
        const { x, y } = result.middlewareData.arrow
        arrowEl.style.left = x != null ? `${x}px` : ''
        arrowEl.style.top = y != null ? `${y}px` : ''
      }
    })

    return () => {
      cleanup?.()
    }
  })

  $effect(() => {
    if (hoverMode && referenceEl) {
      referenceEl.addEventListener('mouseover', onmouseover)
      referenceEl.addEventListener('mouseleave', onmouseleave)

      return () => {
        referenceEl?.removeEventListener('mouseover', onmouseover)
        referenceEl?.removeEventListener('mouseleave', onmouseleave)
      }
    }
  })

  onDestroy(() => {
    cleanup?.()
    clearTimeout(enterTimer)
    clearTimeout(leaveTimer)
  })
</script>

<Portal>
  <div
    class="popper {className}"
    class:-hidden={!isVisible}
    bind:this={floatingEl}
    onmouseover={hoverMode ? onmouseover : undefined}
    onmouseleave={hoverMode ? onmouseleave : undefined}
    role="tooltip"
  >
    <div class="arrow" bind:this={arrowEl}></div>
    {@render popper()}
  </div>
</Portal>

<div bind:this={referenceEl} style="display: contents;">
  {@render children()}
</div>

<style>
  .popper {
    position: absolute;
    background: #fff;
    color: #000;
    z-index: 100;
    border-radius: 5px;
    box-shadow: 0 0 15px 0 rgba(0, 0, 0, 0.2);
    font-size: 12px;
    transition-duration: 0.2s;
    transition-property: visibility, opacity;
  }

  .popper.-hidden {
    visibility: hidden;
    opacity: 0;
  }

  .arrow {
    position: absolute;
    width: 0;
    height: 0;
    border: 5px solid transparent;
    border-bottom-color: #fff;
    top: -10px;
  }

  :global(.popper-body) {
    padding: 10px;
  }

  :global(.popper-menu) {
    padding: 10px 0;
  }

  :global(.popper-menu > a),
  :global(.popper-menu > button) {
    display: block;
    padding: 4px 15px;
    min-width: 100px;
    text-align: center;
    text-decoration: none;
    color: #333;
    background: none;
    border: 0;
    width: 100%;
    font-size: inherit;
    cursor: pointer;
  }

  :global(.popper-menu > a:hover),
  :global(.popper-menu > button:hover) {
    background: #669;
    color: #fff;
  }
</style>

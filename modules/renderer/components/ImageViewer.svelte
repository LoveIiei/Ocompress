<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import { getWheelFromEvent, eventOffset } from '../utils/dom-event'
  import { coop } from '../../common/utils'
  import RadioGroup from './RadioGroup.svelte'
  import Icon from './Icon.svelte'

  interface Props {
    src?: string
    width?: number
    height?: number
  }

  let { src, width, height }: Props = $props()

  const ZOOM_MIN = 1 / 8
  const ZOOM_MAX = 16
  const zoomCoop = coop(ZOOM_MIN, ZOOM_MAX)

  const roundZoom = (zoom: number) => 2 ** Math.round(Math.log2(zoom))
  const floorZoom = (zoom: number) => 2 ** Math.floor(Math.log2(zoom))

  const materials = [
    'gradient',
    'transparent',
    'white',
    'black',
    'red',
    'blue',
    'yellow',
  ]

  let image: HTMLImageElement | undefined = $state()
  let backdrop: HTMLDivElement | undefined = $state()
  let imageNaturalWidth = $state(0)
  let zoom = $state(1)
  let x = $state(0)
  let y = $state(0)
  let material = $state(materials[0])
  let imageError = $state(false)

  let prevScreenX = 0
  let prevScreenY = 0
  let dragging = false

  function initialZoom() {
    if (!image || !backdrop) {
      return 1
    }

    const { naturalWidth, naturalHeight } = image
    const { clientWidth, clientHeight } = backdrop

    return floorZoom(Math.min(clientWidth / naturalWidth, clientHeight / naturalHeight) * 0.9)
  }

  function handleImageLoad() {
    if (!image) return

    imageError = false
    imageNaturalWidth = image.naturalWidth

    if (!imageNaturalWidth) {
      zoom = initialZoom()
    }
  }

  function handleImageError() {
    imageError = true
  }

  function handleMouseDown(e: MouseEvent) {
    e.preventDefault()
    dragging = true
    prevScreenX = e.screenX
    prevScreenY = e.screenY
  }

  function handleMouseMove(e: MouseEvent) {
    if (!dragging) return

    const { screenX, screenY } = e
    x = x + screenX - prevScreenX
    y = y + screenY - prevScreenY

    prevScreenX = screenX
    prevScreenY = screenY
  }

  function handleMouseUp() {
    dragging = false
  }

  function handleWheel(e: WheelEvent) {
    e.preventDefault()

    const wheelData = getWheelFromEvent(e)
    const target = e.currentTarget as HTMLElement

    if (wheelData.type === 'zoom') {
      const mousePosition = eventOffset(e, target)

      const newZoom = zoomCoop(zoom * wheelData.zoom)
      const mouseX = mousePosition.x - target.clientWidth / 2
      const newX = mouseX - ((mouseX - x) / zoom) * newZoom
      const mouseY = mousePosition.y - target.clientHeight / 2
      const newY = mouseY - ((mouseY - y) / zoom) * newZoom

      x = newX
      y = newY
      zoom = newZoom
    } else {
      x = x - wheelData.x
      y = y - wheelData.y
    }
  }

  function handleZoomOut(e: MouseEvent) {
    e.stopPropagation()
    zoom = roundZoom(zoom * 2)
    x = 0
    y = 0
  }

  function handleZoomIn(e: MouseEvent) {
    e.stopPropagation()
    zoom = roundZoom(zoom / 2)
    x = 0
    y = 0
  }

  function handleFocusCenter(e: MouseEvent) {
    e.stopPropagation()
    x = 0
    y = 0
    zoom = initialZoom()
  }

  function handleMaterialChange(value: string) {
    material = value
  }

  function renderMaterialItem(mat: string) {
    return mat
  }

  onMount(() => {
    document.addEventListener('mousemove', handleMouseMove)
    document.addEventListener('mouseup', handleMouseUp)
  })

  onDestroy(() => {
    document.removeEventListener('mousemove', handleMouseMove)
    document.removeEventListener('mouseup', handleMouseUp)
  })
</script>

<div
  class="backdrop"
  onwheel={handleWheel}
  bind:this={backdrop}
  role="presentation"
>
  <div class="material-wall -{material}"></div>
  <div
    class="image-wrapper"
    onmousedown={handleMouseDown}
    role="presentation"
  >
    {#if imageError}
      <div class="image-fail">FAILED</div>
    {:else if src}
      <img
        class="image -transition"
        {src}
        alt=""
        onload={handleImageLoad}
        onerror={handleImageError}
        bind:this={image}
        style="transform: translate({x}px, {y}px) scale({zoom})"
      />
    {/if}
  </div>

  <div class="zoom-control">
    <button
      type="button"
      class="paper"
      onclick={handleFocusCenter}
    >
      <Icon name="focus" />
    </button>
    <button
      type="button"
      class="paper"
      onclick={handleZoomOut}
      disabled={zoom >= ZOOM_MAX}
    >
      +
    </button>
    <button
      type="button"
      class="paper"
      onclick={handleZoomIn}
      disabled={zoom <= ZOOM_MIN}
    >
      -
    </button>
  </div>

  <div class="material-list-container">
    {#each materials as mat}
      <div
        class="material-item"
        class:-checked={mat === material}
        onclick={() => handleMaterialChange(mat)}
        role="button"
        tabindex="0"
        onkeydown={(e) => e.key === 'Enter' && handleMaterialChange(mat)}
      >
        <div class="material-cube -{mat}"></div>
      </div>
    {/each}
  </div>
</div>

<style>
  .backdrop {
    position: relative;
    image-rendering: pixelated;
    overflow: hidden;
    width: 100%;
    height: 100%;
  }

  .material-wall {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
  }

  .material-wall.-transparent,
  .material-cube.-transparent {
    background-image: url(../images/opacity.png);
    image-rendering: pixelated;
  }

  .material-wall.-gradient,
  .material-cube.-gradient {
    background-image:
      linear-gradient(limegreen, transparent),
      linear-gradient(90deg, skyblue, transparent),
      linear-gradient(-90deg, coral, transparent);
    background-blend-mode: screen;
  }

  .material-wall.-white,
  .material-cube.-white {
    background: white;
  }

  .material-wall.-black,
  .material-cube.-black {
    background: black;
  }

  .material-wall.-blue,
  .material-cube.-blue {
    background: #96CEB4;
  }

  .material-wall.-red,
  .material-cube.-red {
    background: #FF6F69;
  }

  .material-wall.-yellow,
  .material-cube.-yellow {
    background: #FFEEAD;
  }

  .material-list-container {
    position: absolute;
    left: 10px;
    bottom: 50px;
    display: flex;
    flex-direction: column;
  }

  .material-item {
    cursor: pointer;
    position: relative;
    border: 2px solid #333;
    margin-top: -2px;
    background: #fff;
  }

  .material-item.-checked {
    border-color: #0c9;
    outline: 1px solid #0c9;
    z-index: 1;
  }

  .material-cube {
    width: 40px;
    height: 40px;
  }

  .material-cube.-transparent {
    background-size: 300%;
  }

  .image-wrapper {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
  }

  .image {
    display: block;
    image-rendering: pixelated;
  }

  .image.-transition {
    transition: transform ease 0.1s;
  }

  .image-fail {
    font-size: 40px;
    color: #999;
    font-weight: 200;
    letter-spacing: 2px;
  }

  .zoom-control {
    position: absolute;
    bottom: 40px;
    right: 15px;
  }

  .zoom-control button {
    display: block;
    width: 25px;
    height: 25px;
    line-height: 25px;
    font-size: 16px;
    text-align: center;
    color: #333;
    border: 0;
    padding: 0;
    margin-bottom: 10px;
    cursor: pointer;
    outline: none;
    transition: all 0.4s;
  }

  .zoom-control button :global(.icon) {
    vertical-align: top;
    height: 100%;
  }

  .zoom-control button:hover,
  .zoom-control button:active {
    color: #0cf;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.4);
  }

  .zoom-control button:disabled {
    color: #ccc;
    cursor: default;
  }
</style>

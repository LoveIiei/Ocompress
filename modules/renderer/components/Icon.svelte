<script lang="ts">
  import '../images/symbols'

  interface Props {
    name: string
    class?: string
  }

  let { name, class: className = '' }: Props = $props()
</script>

<svg class="icon icon-{name} {className}">
  <use href="#icon-{name}" />
</svg>

<style>
  @keyframes spin {
    from {
      transform: rotate(0);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .icon {
    display: inline-block;
    width: 1em;
    height: 1em;
    fill: currentColor;
    overflow: visible;
  }

  :global(.icon.-spin) {
    animation: spin infinite 0.6s linear;
  }
</style>

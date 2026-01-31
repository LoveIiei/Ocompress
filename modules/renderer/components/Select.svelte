<script lang="ts">
  import Icon from './Icon.svelte'
  import type { Snippet } from 'svelte'
  import type { HTMLSelectAttributes } from 'svelte/elements'

  interface Props extends HTMLSelectAttributes {
    class?: string
    iconName?: string
    children?: Snippet
  }

  let {
    style = '',
    class: className = '',
    iconName = 'select',
    children,
    ...rest
  }: Props = $props()
</script>

<div {style} class="select {className}">
  <select {...rest}>
    {#if children}
      {@render children()}
    {/if}
  </select>
  <Icon name={iconName} />
</div>

<style>
  .select {
    position: relative;
    display: inline-block;
    transition: border-color 0.3s;
  }

  .select select {
    position: relative;
    -webkit-appearance: none;
    border: 0;
    outline: 0;
    background: none;
    color: currentColor;
    cursor: pointer;
    padding: 1px 2px;
    padding-right: 15px;
    width: 100%;
    z-index: 1;
  }

  .select :global(.icon) {
    position: absolute;
    right: 2px;
    top: 10%;
    bottom: 0;
    height: 80%;
  }
</style>

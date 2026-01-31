<script lang="ts">
  import Modal from './Modal.svelte'
  import Select from './Select.svelte'
  import ColorPicker from './ColorPicker.svelte'
  import RadioGroup from './RadioGroup.svelte'
  import { imagineAPI } from '../../bridge/web'
  import { showMessage } from './Messager.svelte'
  import __ from '../../locales'
  import type { PlaceholderRequest } from '../../common/types'

  interface Props {
    visible: boolean
    onClose: () => void
  }

  let { visible, onClose }: Props = $props()

  type Preset = {
    label: string
    width: number
    height: number
  }

  const presets: Preset[] = [
    { label: 'Medium Rectangle (300x250)', width: 300, height: 250 },
    { label: 'Leaderboard (728x90)', width: 728, height: 90 },
    { label: 'Wide Skyscraper (160x600)', width: 160, height: 600 },
    { label: 'Full HD (1920x1080)', width: 1920, height: 1080 },
    { label: 'Social Square (1080x1080)', width: 1080, height: 1080 },
    { label: 'Thumbnail (150x150)', width: 150, height: 150 },
    { label: 'Custom', width: 0, height: 0 },
  ]

  const formats = ['png', 'jpg', 'webp'] as const

  let selectedPreset = $state(0)
  let width = $state(300)
  let height = $state(250)
  let backgroundColor = $state('#CCCCCC')
  let textColor = $state('#666666')
  let customText = $state('')
  let format = $state<'png' | 'jpg' | 'webp'>('png')
  let isGenerating = $state(false)

  function handlePresetChange(e: Event) {
    const target = e.target as HTMLSelectElement
    const index = parseInt(target.value, 10)
    selectedPreset = index
    const preset = presets[index]
    if (preset.width > 0 && preset.height > 0) {
      width = preset.width
      height = preset.height
    }
  }

  function handleWidthChange(e: Event) {
    const target = e.target as HTMLInputElement
    width = Math.max(1, Math.min(4096, parseInt(target.value, 10) || 1))
    selectedPreset = presets.length - 1 // Switch to Custom
  }

  function handleHeightChange(e: Event) {
    const target = e.target as HTMLInputElement
    height = Math.max(1, Math.min(4096, parseInt(target.value, 10) || 1))
    selectedPreset = presets.length - 1 // Switch to Custom
  }

  async function handleGenerate() {
    if (isGenerating) return

    isGenerating = true
    try {
      const request: PlaceholderRequest = {
        width,
        height,
        backgroundColor,
        textColor,
        text: customText || undefined,
        format,
      }
      await imagineAPI.generatePlaceholder(request)
      showMessage({
        message: __('placeholder_saved'),
        type: 'success',
      })
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : String(err)
      console.error('Failed to generate placeholder:', errorMessage)
      showMessage({
        message: `${__('placeholder_error')}: ${errorMessage}`,
        type: 'error',
      })
    } finally {
      isGenerating = false
    }
  }
</script>

<Modal {visible} {onClose}>
  <div class="placeholder-generator">
    <h2>{__('placeholder_title')}</h2>

    <div class="form-group">
      <label for="placeholder-preset">{__('placeholder_preset')}</label>
      <Select id="placeholder-preset" value={selectedPreset} onchange={handlePresetChange}>
        {#each presets as preset, index}
          <option value={index}>{preset.label}</option>
        {/each}
      </Select>
    </div>

    <div class="form-row">
      <div class="form-group">
        <label for="placeholder-width">{__('placeholder_width')}</label>
        <input
          id="placeholder-width"
          type="number"
          value={width}
          onchange={handleWidthChange}
          min="1"
          max="4096"
        />
      </div>
      <div class="form-group">
        <label for="placeholder-height">{__('placeholder_height')}</label>
        <input
          id="placeholder-height"
          type="number"
          value={height}
          onchange={handleHeightChange}
          min="1"
          max="4096"
        />
      </div>
    </div>

    <div class="form-group">
      <span class="label-text">{__('placeholder_background')}</span>
      <ColorPicker
        value={backgroundColor}
        onChange={(v) => (backgroundColor = v)}
      />
    </div>

    <div class="form-group">
      <span class="label-text">{__('placeholder_text_color')}</span>
      <ColorPicker
        value={textColor}
        onChange={(v) => (textColor = v)}
      />
    </div>

    <div class="form-group">
      <label for="placeholder-text">{__('placeholder_text')}</label>
      <input
        id="placeholder-text"
        type="text"
        bind:value={customText}
        placeholder="{width}x{height}"
      />
      <span class="hint">{__('placeholder_text_hint')}</span>
    </div>

    <div class="form-group">
      <span class="label-text">{__('placeholder_format')}</span>
      <RadioGroup
        value={format}
        data={formats}
        onChange={(v) => (format = v)}
        renderItem={(item) => item.toUpperCase()}
      />
    </div>

    <div class="form-actions">
      <button
        type="button"
        class="generate-btn"
        onclick={handleGenerate}
        disabled={isGenerating}
      >
        {isGenerating ? __('placeholder_generating') : __('placeholder_generate')}
      </button>
    </div>
  </div>
</Modal>

<style>
  .placeholder-generator {
    padding: 40px;
    max-width: 400px;
    margin: 0 auto;
  }

  h2 {
    margin: 0 0 24px;
    font-size: 20px;
    font-weight: 500;
    color: #333;
  }

  .form-group {
    margin-bottom: 16px;
  }

  .form-group label,
  .form-group .label-text {
    display: block;
    margin-bottom: 6px;
    font-size: 13px;
    color: #666;
  }

  .form-row {
    display: flex;
    gap: 16px;
  }

  .form-row .form-group {
    flex: 1;
  }

  .placeholder-generator input[type="number"],
  .placeholder-generator input[type="text"] {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 14px;
    outline: none;
    transition: border-color 0.2s;
    box-sizing: border-box;
  }

  .placeholder-generator input:focus {
    border-color: #09f;
  }

  .hint {
    display: block;
    margin-top: 4px;
    font-size: 11px;
    color: #999;
  }

  .form-actions {
    margin-top: 24px;
    text-align: right;
  }

  .generate-btn {
    padding: 10px 24px;
    background: #09f;
    color: #fff;
    border: none;
    border-radius: 4px;
    font-size: 14px;
    cursor: pointer;
    transition: background 0.2s;
  }

  .generate-btn:hover:not(:disabled) {
    background: #08e;
  }

  .generate-btn:disabled {
    background: #ccc;
    cursor: not-allowed;
  }

  .placeholder-generator :global(.select) {
    width: 100%;
  }

  .placeholder-generator :global(.select select) {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 14px;
    background: #fff;
  }
</style>

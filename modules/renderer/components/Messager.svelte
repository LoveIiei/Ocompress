<script lang="ts" module>
  import { mount, unmount } from 'svelte'
  import MessagerItem from './MessagerItem.svelte'

  export interface IMessagerProps {
    message: string
    type?: 'error' | 'warning' | 'success' | 'info'
    duration?: number
  }

  let messagerContainer: HTMLDivElement | null = null
  let messagerComponent: Record<string, any> | null = null
  let closeTimer = 0

  export function showMessage(options: string | IMessagerProps) {
    if (!messagerContainer) {
      messagerContainer = document.createElement('div')
      messagerContainer.className = 'global-messager'
      document.body.appendChild(messagerContainer)
    }

    const props: IMessagerProps = typeof options === 'string'
      ? { message: options, type: 'info' }
      : options

    window.clearTimeout(closeTimer)

    if (messagerComponent) {
      unmount(messagerComponent)
    }

    messagerComponent = mount(MessagerItem, {
      target: messagerContainer,
      props: {
        message: props.message,
        type: props.type || 'info',
        visible: true,
      },
    })

    closeTimer = window.setTimeout(() => {
      if (messagerComponent) {
        unmount(messagerComponent)
        messagerComponent = null
      }
    }, props.duration || 3000)
  }
</script>

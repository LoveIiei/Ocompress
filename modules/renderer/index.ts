import { mount } from 'svelte'
import App from './App.svelte'
import { JobRunner } from './store/job-runner'
import listenIpc from './ipc/listen'
import { setup as setupLocales } from '../locales'

setupLocales(navigator.language)

const runner = new JobRunner()
runner.start()

// Initialize IPC listeners (async)
listenIpc()

const app = mount(App, {
  target: document.getElementById('app')!,
})

export default app

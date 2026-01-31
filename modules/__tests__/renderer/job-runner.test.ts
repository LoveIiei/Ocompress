/**
 * @jest-environment jsdom
 */
import '../_tools/before-test'

import { createStore } from '../../renderer/store/store'
import JobRunner from '../../renderer/store/job-runner'
import actions from '../../renderer/store/actionCreaters'
import { sleep } from '../../common/utils'
import {
  IImageFile, TaskStatus,
} from '../../common/types'

jest.mock('../../bridge/web')

const mockImages: IImageFile[] = [
  {
    id: 'test-1',
    url: 'file:///test/image1.png',
    size: 1000,
    ext: 'png' as const,
    originalName: '/test/image1.png',
  },
  {
    id: 'test-2',
    url: 'file:///test/image2.png',
    size: 2000,
    ext: 'png' as const,
    originalName: '/test/image2.png',
  },
]

test('optimize JobRunner', async () => {
  const store = createStore()
  new JobRunner().watch(store)
  let state

  store.dispatch(actions.taskAdd(mockImages))

  // for debounce
  await sleep(100)

  state = store.getState()

  expect(state.tasks[0].status).toBe(TaskStatus.PROCESSING)
  expect(state.tasks[1].status).toBe(TaskStatus.PENDING)

  // enough for processing two images
  await sleep(500)

  state = store.getState()
  expect(state.tasks[0].status).toBe(TaskStatus.DONE)
  expect(state.tasks[1].status).toBe(TaskStatus.DONE)

  await sleep(10)

  // update options and auto optimized
  store.dispatch(actions.taskUpdateOptions(mockImages[0].id, {
    color: 8,
  }))

  // for debounce
  await sleep(100)

  state = store.getState()
  expect(state.tasks[0].status).toBe(TaskStatus.PROCESSING)
  expect(state.tasks[1].status).toBe(TaskStatus.DONE)

  await sleep(200)

  state = store.getState()
  expect(state.tasks[0].status).toBe(TaskStatus.DONE)
})

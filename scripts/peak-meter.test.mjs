import assert from 'node:assert/strict'
import test from 'node:test'
import { JSDOM } from 'jsdom'

const dom = new JSDOM('<!doctype html><div id="root"></div>', { pretendToBeVisual: true })
const { window } = dom
// A manual frame clock: the tests decide when time passes.
let queue = new Map()
let nextId = 1
let clock = 1000
window.requestAnimationFrame = cb => { const id = nextId++; queue.set(id, cb); return id }
window.cancelAnimationFrame = id => { queue.delete(id) }
Object.assign(globalThis, {
  window, document: window.document, requestAnimationFrame: window.requestAnimationFrame,
  cancelAnimationFrame: window.cancelAnimationFrame, IS_REACT_ACT_ENVIRONMENT: true
})
Object.defineProperty(globalThis, 'navigator', { value: window.navigator, configurable: true })
let reduced = false
const motionListeners = new Set()
const mq = { get matches() { return reduced }, addEventListener: (_, l) => motionListeners.add(l), removeEventListener: (_, l) => motionListeners.delete(l) }
window.matchMedia = globalThis.matchMedia = () => mq

const React = await import('react')
const { createRoot } = await import('react-dom/client')
const { PeakMeter } = await import('../dist/index.js')
const { act, Profiler, createElement: h, createRef } = React

const flush = async (frames, ms = 16) => {
  for (let i = 0; i < frames; i++) {
    const due = [...queue]
    queue = new Map()
    clock += ms
    await act(async () => { for (const [, cb] of due) cb(clock) })
  }
}
const mount = async element => {
  const host = document.createElement('div')
  document.body.append(host)
  const root = createRoot(host)
  await act(async () => root.render(element))
  return { host, root, meter: host.firstElementChild }
}

test('renders N bricks for each size, plus one peak marker', async () => {
  const full = await mount(h(PeakMeter, { db: -Infinity }))
  assert.equal(full.meter.querySelectorAll('.sz-meter-brick').length, 40)
  assert.equal(full.meter.querySelectorAll('.sz-meter-peak').length, 1)
  const mini = await mount(h(PeakMeter, { db: -Infinity, mini: true, size: 2, className: 'tv' }))
  assert.equal(mini.meter.querySelectorAll('.sz-meter-brick').length, 16)
  assert.ok(mini.meter.classList.contains('mini') && mini.meter.classList.contains('tv'))
  assert.equal(mini.meter.style.getPropertyValue('--sz-meter-scale'), '2')
  assert.equal(queue.size, 0, 'a silent meter schedules no frame')
  for (const m of [full, mini]) { m.root.unmount(); m.host.remove() }
})

test('a prop change leaves the DOM alone and the loop animates without React renders', async () => {
  let commits = 0
  const ui = db => h(Profiler, { id: 'm', onRender: () => { commits++ } }, h(PeakMeter, { db }))
  const { host, root, meter } = await mount(ui(-Infinity))
  const bricks = [...meter.children]
  let mutations = 0
  const observer = new window.MutationObserver(records => { mutations += records.length })
  observer.observe(meter, { childList: true, subtree: true })
  await act(async () => root.render(ui(-12)))
  const afterPropChange = commits
  assert.ok(queue.size > 0, 'a live level starts the rAF loop')
  await flush(30)
  await new Promise(r => setTimeout(r, 0)) // let MutationObserver deliver
  assert.equal(commits, afterPropChange, 'frames do not re-render React')
  assert.equal(mutations, 0, 'no nodes were added or removed')
  assert.deepEqual([...meter.children], bricks, 'brick elements keep their identity')
  const lit = meter.children[0].lastElementChild.style.transform
  assert.equal(lit, 'scaleX(1)')
  assert.equal(meter.querySelector('.sz-meter-peak').style.opacity, '1')
  observer.disconnect()
  root.unmount(); host.remove()
})

test('the imperative handle drives the meter with zero React renders', async () => {
  let commits = 0
  const ref = createRef()
  const { host, root, meter } = await mount(h(Profiler, { id: 'h', onRender: () => { commits++ } }, h(PeakMeter, { db: -Infinity, ref })))
  const before = commits
  await act(async () => ref.current.push(-3))
  await flush(20)
  assert.equal(commits, before)
  assert.ok(meter.children[37].lastElementChild.style.transform !== 'scaleX(0)')
  root.unmount(); host.remove()
})

test('the rAF loop stops at rest, restarts on new input, and stops when hidden or disabled', async () => {
  const { host, root } = await mount(h(PeakMeter, { db: -6 }))
  assert.ok(queue.size > 0)
  await act(async () => root.render(h(PeakMeter, { db: -Infinity })))
  await flush(120, 100) // hold, fall, afterglow
  assert.equal(queue.size, 0, 'at rest: no frame is pending')
  await act(async () => root.render(h(PeakMeter, { db: -10 })))
  assert.ok(queue.size > 0, 'new input restarts the loop')

  Object.defineProperty(document, 'hidden', { value: true, configurable: true })
  await act(async () => { document.dispatchEvent(new window.Event('visibilitychange')) })
  assert.equal(queue.size, 0, 'hidden document cancels the frame')
  Object.defineProperty(document, 'hidden', { value: false, configurable: true })
  await act(async () => { document.dispatchEvent(new window.Event('visibilitychange')) })
  assert.ok(queue.size > 0, 'visible again resumes with the latest level')

  await act(async () => root.render(h(PeakMeter, { db: -10, enabled: false })))
  assert.equal(queue.size, 0, 'disabled: no frame')
  root.unmount(); host.remove()
})

test('reduced motion shows the level and held peak without running a loop', async () => {
  reduced = true
  const { host, root, meter } = await mount(h(PeakMeter, { db: -20, peakDb: -6 }))
  assert.equal(queue.size, 0)
  assert.equal(meter.children[0].lastElementChild.style.transform, 'scaleX(1)')
  assert.equal(meter.querySelector('.sz-meter-peak').style.opacity, '1')
  assert.equal(meter.children[0].firstElementChild.style.opacity || '0', '0')
  reduced = false
  root.unmount(); host.remove()
})

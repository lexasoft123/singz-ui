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

test('bitmap TV meters cap paint cadence and eliminate per-brick DOM mutations', async () => {
  Object.assign(globalThis, { MutationObserver: window.MutationObserver, getComputedStyle: window.getComputedStyle.bind(window) })
  const bounds = window.HTMLElement.prototype.getBoundingClientRect
  window.HTMLElement.prototype.getBoundingClientRect = () => ({ width: 120, height: 16 })
  const contexts = new Map()
  const getContext = window.HTMLCanvasElement.prototype.getContext
  window.HTMLCanvasElement.prototype.getContext = function () {
    if (!contexts.has(this)) contexts.set(this, { paints: 0, clearRect() { this.paints++ }, save() {}, restore() {}, beginPath() {}, roundRect() {}, fill() {}, clip() {}, fillRect() {} })
    return contexts.get(this)
  }
  const measure = async (renderMode, maxFps) => {
    const refs = [createRef(), createRef()]
    const mounted = await mount(h('div', null, ...refs.map((ref, i) => h(PeakMeter, { key: i, ref, db: -Infinity, mini: true, renderMode, maxFps }))))
    let mutations = 0
    const observer = new window.MutationObserver(records => { mutations += records.length })
    observer.observe(mounted.host, { attributes: true, subtree: true })
    const startPaints = [...contexts.values()].reduce((n, c) => n + c.paints, 0)
    for (let i = 0; i < 1200; i++) {
      if (i % 6 === 0) refs.forEach((ref, slot) => ref.current.push(-32 + Math.sin(i / 31 + slot) * 20))
      await flush(1, 1000 / 120)
    }
    const paints = [...contexts.values()].reduce((n, c) => n + c.paints, 0) - startPaints
    const nodes = mounted.host.querySelectorAll('.sz-meter *').length
    observer.disconnect(); await act(async () => mounted.root.unmount()); mounted.host.remove()
    return { mutations, paints, nodes }
  }
  const domResult = await measure('dom', 0)
  const canvasResult = await measure('canvas', 30)
  console.log('Mac 120 Hz / 10 s / two moving mini meters:', JSON.stringify({ dom: domResult, canvas30: canvasResult }))
  assert.equal(domResult.nodes, 98)
  assert.equal(canvasResult.nodes, 2)
  assert.equal(canvasResult.mutations, 0, 'frames never mutate bitmap DOM attributes')
  assert.ok(canvasResult.paints > 100 && canvasResult.paints <= 602, 'at most thirty dirty bitmap paints per second per meter')
  assert.ok(domResult.mutations > 1000, 'the DOM control actually moves')
  assert.equal(queue.size, 0, 'unmount cancels both loops')
  window.HTMLCanvasElement.prototype.getContext = getContext
  window.HTMLElement.prototype.getBoundingClientRect = bounds
})

test('canvas damage is gated on whole pixels and never adds a glow', async () => {
  const { paintMeterCanvas } = await import('../dist/meters/paint-canvas.js')
  const { emptyPeakMeter } = await import('../dist/meters/peak-meter-motion.js')
  const ctx = { paints: 0, clearRect() { this.paints++ }, save() {}, restore() {}, beginPath() {}, roundRect() {}, fill() {}, clip() {}, fillRect() {},
    set shadowBlur(value) { assert.equal(value, 0) } }
  const palette = { zones: ['#5d9', '#fc5', '#f65'], faint: '#555', danger: '#f65', text: '#fff' }
  const motion = { ...emptyPeakMeter(16), liveDb: -44, peakDb: -44 }
  const key = paintMeterCanvas(ctx, motion, 16, 120, 16, 2, 1, palette, false, '')
  const again = paintMeterCanvas(ctx, { ...motion, liveDb: -44.001 }, 16, 120, 16, 2, 1, palette, false, key)
  assert.equal(again, key)
  assert.equal(ctx.paints, 1)
})


test('instant canvas attack paints on the first frame and traces only changed live pixels', async () => {
  Object.assign(globalThis, { MutationObserver: window.MutationObserver, getComputedStyle: window.getComputedStyle.bind(window) })
  const bounds = window.HTMLElement.prototype.getBoundingClientRect
  const getContext = window.HTMLCanvasElement.prototype.getContext
  window.HTMLElement.prototype.getBoundingClientRect = () => ({ width: 120, height: 16 })
  window.HTMLCanvasElement.prototype.getContext = () => ({ clearRect() {}, save() {}, restore() {}, beginPath() {}, roundRect() {}, fill() {}, clip() {}, fillRect() {} })
  const ref = createRef(), paints = []
  const ui = await mount(h(PeakMeter, { db: -Infinity, ref, renderMode: 'canvas', maxFps: 30, attackMs: 0, onPaint: (id, at) => paints.push({ id, at }) }))
  await act(async () => ref.current.push(-12, -12, 7))
  await flush(1, 1000 / 30)
  assert.deepEqual(paints, [{ id: 7, at: clock }], 'new live pixels appear on the first available frame')
  await act(async () => ref.current.push(-12, -12, 8))
  await flush(4, 1000 / 30)
  assert.equal(paints.length, 1, 'trail-only paints never attribute an unchanged reading')
  await act(async () => ref.current.push(-6, -6, 9))
  await flush(1, 1000 / 30)
  assert.equal(paints.at(-1).id, 9)
  await act(async () => ui.root.unmount()); ui.host.remove()
  assert.equal(queue.size, 0)
  window.HTMLCanvasElement.prototype.getContext = getContext
  window.HTMLElement.prototype.getBoundingClientRect = bounds
})

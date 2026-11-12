import assert from 'node:assert/strict'
import test from 'node:test'
import { emptyPeakMeter, meterBrickFill, meterIndex, meterZone, peakMeterAtRest, stepPeakMeter } from '../dist/meters/index.js'

test('holds a peak while live level falls and leaves a fading gradient behind it', () => {
  let state = stepPeakMeter(emptyPeakMeter(40), -6, 16)
  for (let i = 0; i < 6; i++) state = stepPeakMeter(state, -Infinity, 100)
  assert.equal(state.peakDb, -6)
  assert.ok(state.liveDb < -30)
  const end = meterIndex(state.peakDb, 40)
  assert.ok(state.trail[end - 1] > state.trail[end - 8])
  assert.ok(state.trail[end - 8] > 0)
})

test('lets the trail disappear under a sustained peak so the live edge stays visible', () => {
  let state = emptyPeakMeter(16)
  for (let frame = 0; frame < 180; frame++) state = stepPeakMeter(state, -15, 1000 / 60, -0.3)
  assert.equal(state.peakDb, -0.3)
  assert.ok(Math.abs(state.liveDb + 15) < 0.01)
  assert.ok(state.trail.every(value => value === 0))
  state = stepPeakMeter(state, -15, 16, -0.1)
  assert.ok(Math.max(...state.trail) > 0)
  assert.ok(Math.max(...state.trail) < 0.32)
})

test('ignores smaller peaks and punches the held brick forward on a stronger peak', () => {
  let state = stepPeakMeter(emptyPeakMeter(40), -18, 16)
  state = stepPeakMeter(state, -24, 100)
  assert.equal(state.peakDb, -18)
  state = stepPeakMeter(state, -1, 16)
  assert.equal(state.peakDb, -1)
  assert.equal(state.holdMs, 1100)
})

test('falls back after the hold and fully settles after playback stops', () => {
  let state = stepPeakMeter(emptyPeakMeter(12), -1, 16)
  for (let i = 0; i < 12; i++) state = stepPeakMeter(state, -Infinity, 100)
  assert.ok(state.peakDb < -1)
  for (let i = 0; i < 100; i++) state = stepPeakMeter(state, -Infinity, 100)
  assert.equal(peakMeterAtRest(state), true)
})

test('uses corresponding green/yellow/red zones for live, trail and peak in both sizes', () => {
  for (const count of [12, 16, 24, 40]) {
    assert.equal(meterZone(meterIndex(-18, count), count), 0)
    assert.equal(meterZone(meterIndex(-6, count), count), 1)
    assert.equal(meterZone(meterIndex(-1, count), count), 2)
    assert.equal(meterIndex(Infinity, count), -1)
  }
})

test('keeps the live bar close to the held peak on dense material', () => {
  let state = emptyPeakMeter(16)
  const levels = []
  for (let frame = 0; frame < 240; frame++) {
    state = stepPeakMeter(state, -1 + 0.6 * Math.sin(frame / 12), 1000 / 60)
    if (frame > 60) {
      levels.push(state.liveDb)
      assert.ok(state.peakDb - state.liveDb < 1.5)
    }
  }
  assert.ok(Math.max(...levels) - Math.min(...levels) > 0.8)
})

test('shows changes smaller than one compact segment instead of snapping to a full brick', () => {
  const first = meterBrickFill(-4.7, 22, 24)
  const second = meterBrickFill(-4.1, 22, 24)
  assert.ok(first > 0)
  assert.ok(second > first)
  assert.ok(second < 1)
})

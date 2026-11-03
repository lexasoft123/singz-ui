import assert from 'node:assert/strict'
import test from 'node:test'
import { headerLayout } from '../dist/native/header-layout.js'

test('counter plus Log use their natural width and keep a centered title readable', () => {
  const layout = headerLayout(390, 48, 100, 1, 'center')
  assert.equal(layout.stackActions, false)
  assert.equal(48 + layout.leadingSpace, 100 + layout.trailingSpace)
})

test('narrow screens move crowded actions onto their own row instead of overlapping the title', () => {
  for (const width of [280, 320, 360]) {
    const layout = headerLayout(width, 48, 160, 1, 'center')
    assert.equal(layout.stackActions, true)
    assert.equal(layout.leadingSpace + layout.trailingSpace, 0)
  }
})

test('large accessibility fonts get the same minimum readable identity space', () => {
  assert.equal(headerLayout(390, 48, 100, 1.6, 'center').stackActions, true)
  assert.equal(headerLayout(768, 48, 160, 1.6, 'center').stackActions, false)
})

test('branding and rich song identity use available space without empty symmetric slots', () => {
  const layout = headerLayout(390, 86, 96, 1, 'start')
  assert.deepEqual(layout, { stackActions: false, leadingSpace: 0, trailingSpace: 0 })
  assert.equal(headerLayout(320, 86, 96, 1, 'start').stackActions, true)
})

test('an action-free header never adds an empty actions row', () => {
  assert.equal(headerLayout(320, 48, 0, 3, 'center').stackActions, false)
  assert.equal(headerLayout(0, 48, 160, 1, 'center').stackActions, false)
})

import assert from 'node:assert/strict'
import test from 'node:test'
import { readFileSync } from 'node:fs'
import vm from 'node:vm'

// Execute the shipped component, keeping native primitives as inspectable
// elements. These tests cover the props sent to native text/layout; actual
// Yoga geometry is additionally verified in the consuming app's simulators.
const element = (type, props) => ({ type, props })
const theme = { text: '#fff', dim: '#aaa', controlFill: '#333' }
const context = vm.createContext({})
const mockModule = exports => new vm.SyntheticModule(Object.keys(exports), function () {
  for (const [key, value] of Object.entries(exports)) this.setExport(key, value)
}, { context })
const dependencies = {
  'react/jsx-runtime': mockModule({ jsx: element, jsxs: element, Fragment: 'Fragment' }),
  'react-native': mockModule({ Pressable: 'Pressable', Text: 'Text', View: 'View', StyleSheet: { create: value => value, hairlineWidth: 1 } }),
  './theme.js': mockModule({ useNativeTheme: () => theme, nativeGlassStyle: () => ({}) })
}
const source = readFileSync(process.env.UIKIT_HEADER_MODULE ?? new URL('../dist/native/components.js', import.meta.url), 'utf8')
const components = new vm.SourceTextModule(source, { context })
await components.link(name => {
  assert.ok(dependencies[name], `Unexpected header dependency ${name}`)
  return dependencies[name]
})
await components.evaluate()
const { GlassHeader, HeaderAction } = components.namespace
const flatten = value => Object.assign({}, ...[value].flat(Infinity).filter(Boolean))
const nodes = value => {
  if (!value || typeof value !== 'object') return []
  if (typeof value.type === 'function') return nodes(value.type(value.props))
  return [value, ...[value.props.children].flat(Infinity).flatMap(nodes)]
}
const render = props => nodes(GlassHeader(props))
const titleOf = rendered => rendered.find(node => node.props.accessibilityRole === 'header')

test('glass and plain headers remain exactly 62pt even with a consumer height override', () => {
  for (const variant of ['glass', 'plain']) {
    const onLayout = () => {}
    const rendered = render({ title: 'A long localized screen title', variant, style: { height: 140, paddingVertical: 40 }, onLayout })
    const frame = flatten(rendered[0].props.style)
    assert.equal(frame.height, 62)
    assert.equal(frame.minHeight, 62)
    assert.equal(frame.maxHeight, 62)
    assert.equal(frame.paddingVertical, 0)
    assert.equal(frame.flexDirection, 'row')
    assert.equal(frame.flexWrap, 'nowrap')
    assert.equal(rendered[0].props.onLayout, onLayout)
  }
})

test('visible title is smaller, single-line, tail-truncated and retains its full accessible name', () => {
  const title = 'Очень длинное название упражнения для узкого экрана'
  const rendered = render({ title })
  const node = titleOf(rendered)
  assert.equal(flatten(node.props.style).fontSize, 17)
  assert.equal(node.props.numberOfLines, 1)
  assert.equal(node.props.ellipsizeMode, 'tail')
  assert.equal(node.props.maxFontSizeMultiplier, 1.3)
  assert.equal(node.props.accessibilityLabel, title)
  assert.equal(node.props.onTextLayout, undefined)
})

test('optional subtitle shares the same fixed bar and single-line text policy', () => {
  const rendered = render({ title: 'Song', subtitle: 'Artist · 120 BPM · G major' })
  const subtitle = rendered.find(node => node.type === 'Text' && node.props.children === 'Artist · 120 BPM · G major')
  assert.equal(flatten(subtitle.props.style).fontSize, 11)
  assert.equal(subtitle.props.numberOfLines, 1)
  assert.equal(subtitle.props.ellipsizeMode, 'tail')
  assert.equal(subtitle.props.maxFontSizeMultiplier, 1.3)
  assert.equal(flatten(rendered[0].props.style).height, 62)
})

test('back and trailing actions keep fixed 44pt hit surfaces and never create wrapping rows', () => {
  let tapped = 0
  const rendered = render({ title: 'Melody', onBack: () => tapped++, backLabel: 'Back',
    trailing: HeaderAction({ label: 'Log', onPress: () => tapped++, children: 'icon' }) })
  const actions = rendered.filter(node => node.type === 'Pressable')
  assert.equal(actions.length, 2)
  for (const action of actions) {
    const frame = flatten(action.props.style({ pressed: false }))
    assert.equal(frame.width, 44)
    assert.equal(frame.height, 44)
    assert.equal(frame.minWidth, 44)
    assert.equal(frame.minHeight, 44)
    assert.equal(action.props.hitSlop, 10)
    assert.equal(action.props.pressRetentionOffset, 12)
    assert.equal(action.props.collapsable, false)
    action.props.onPress()
  }
  assert.equal(tapped, 2)
  assert.ok(rendered.every(node => typeof node.props.style === 'function' || flatten(node.props.style).flexWrap !== 'wrap'))
})

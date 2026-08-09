/*
 * Emit the CSS custom-property block from tokens.ts.
 *
 * Runs AFTER tsc, against the compiled dist/tokens/tokens.js, so there is
 * exactly one definition of every value and the TS object cannot drift from
 * the stylesheet. Never hand-edit dist/*.css.
 */
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT = join(dirname(fileURLToPath(import.meta.url)), '..')
const DIST = join(ROOT, 'dist')

const { toCss } = await import(join(DIST, 'tokens', 'tokens.js'))

const HEADER = `/* GENERATED from src/tokens/tokens.ts — do not edit. */\n`
const tokensCss = HEADER + toCss()

mkdirSync(DIST, { recursive: true })
writeFileSync(join(DIST, 'tokens.css'), tokensCss)

// kit.css is the single stylesheet a consumer imports: the token block, then
// each layer in extraction order, so the cascade stays predictable. Layers
// are concatenated rather than @import-ed — an @import must precede all other
// rules, and it costs a second request in any host that doesn't bundle.
const LAYERS = ['primitives.css', 'overlays.css']
const layers = LAYERS.map((f) => readFileSync(join(ROOT, 'src', 'styles', f), 'utf8'))
const kitCss = [HEADER, tokensCss.slice(HEADER.length), ...layers].join('\n')
writeFileSync(join(DIST, 'kit.css'), kitCss)

console.log(
  `wrote dist/tokens.css (${tokensCss.split('\n').length - 1} lines) and ` +
    `dist/kit.css (${kitCss.split('\n').length - 1} lines, layers: ${LAYERS.join(', ')})`
)

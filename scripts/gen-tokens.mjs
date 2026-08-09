/*
 * Emit the CSS custom-property block from tokens.ts.
 *
 * Runs AFTER tsc, against the compiled dist/tokens/tokens.js, so there is
 * exactly one definition of every value and the TS object cannot drift from
 * the stylesheet. Never hand-edit dist/*.css.
 */
import { mkdirSync, writeFileSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT = join(dirname(fileURLToPath(import.meta.url)), '..')
const DIST = join(ROOT, 'dist')

const { toCss } = await import(join(DIST, 'tokens', 'tokens.js'))

const HEADER = `/* GENERATED from src/tokens/tokens.ts — do not edit. */\n`
const tokensCss = HEADER + toCss()

mkdirSync(DIST, { recursive: true })
writeFileSync(join(DIST, 'tokens.css'), tokensCss)

// kit.css is the single stylesheet a consumer imports. Today it is only the
// token block; primitives, overlays, audio and chrome layers append here as
// they are extracted, in that order, so the cascade stays predictable.
writeFileSync(join(DIST, 'kit.css'), tokensCss)

console.log(`wrote dist/tokens.css and dist/kit.css (${tokensCss.split('\n').length - 1} lines)`)

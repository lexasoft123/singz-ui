/*
 * "Would this kit actually re-theme?"
 *
 * Every literal colour outside tokens.css is a colour a consumer cannot
 * change. ChordZ — the first other app that wants this kit — runs a LIGHT
 * paper palette (#F6F0E4 ground, #B5491C accent), so anything hardcoded here
 * stays stubbornly dark on their screen and looks like a bug in the kit.
 *
 * This is the acceptance test for the token set being complete. It reports
 * every offender rather than just failing, because the list IS the work.
 *
 * ALLOWED lists the literals that are deliberately not tokens, each with a
 * reason. Adding to it should feel like a decision.
 */
import { readFileSync, readdirSync } from 'node:fs'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const ROOT = join(dirname(fileURLToPath(import.meta.url)), '..')
const STYLES = join(ROOT, 'src', 'styles')

const ALLOWED = new Map([
  ['#fff', 'text on the Windows close button, which is always red'],
  ['#c42b1c', "Windows' own close-button red — an OS convention, not ours"],
  ['#ffbe58', 'the light end of the primary gradient; pairs with --sz-accent-deep'],
  ['rgba(255, 160, 40, 0.7)', 'focus ring — deliberately the accent at fixed alpha'],
  ['rgba(255,160,40,0.7)', 'focus ring'],
  ['rgba(255, 255, 255, 0.25)', 'inset highlight on the primary gradient'],
  ['rgba(255, 150, 40, 0.28)', 'primary glow'],
  ['rgba(255, 150, 40, 0.4)', 'primary glow, hover'],
  ['rgba(88, 214, 138, 0.7)', 'glow around --sz-success'],
  ['rgba(255, 160, 40, 0.7)', 'glow around --sz-accent'],
  ['rgba(8, 6, 3, 0.62)', 'modal scrim — a darkener, not a surface colour'],
  ['rgba(8, 6, 3, 0.82)', 'modal scrim, Windows (no blur)'],
  ['rgba(0, 0, 0, 0.6)', 'drop shadow'],
  ['rgba(0, 0, 0, 0.5)', 'drop shadow'],
  ['rgba(255, 240, 214, 0.07)', 'window-button hover wash'],
  ['rgba(255, 240, 214, 0.14)', 'frameless window hairline']
])

const COLOUR = /#[0-9a-fA-F]{3,8}\b|rgba?\([^)]*\)/g

let bad = 0
let allowed = 0
for (const f of readdirSync(STYLES).filter((x) => x.endsWith('.css'))) {
  const src = readFileSync(join(STYLES, f), 'utf8')
  src.split('\n').forEach((line, i) => {
    if (line.trim().startsWith('*') || line.trim().startsWith('/*')) return
    for (const m of line.match(COLOUR) ?? []) {
      if (ALLOWED.has(m)) {
        allowed += 1
        continue
      }
      bad += 1
      console.log(`${f}:${i + 1}  ${m}`)
    }
  })
}

console.log(
  `\n${bad} un-tokenised colour${bad === 1 ? '' : 's'} outside tokens.css ` +
    `(${allowed} allowed by name)`
)
if (bad > 0) {
  console.log('Each of these is a colour a consumer cannot re-theme.')
  process.exit(1)
}

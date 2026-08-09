/*
 * The night-studio design tokens — the source of truth.
 *
 * Pure data, ZERO imports. That is deliberate and load-bearing: React Native
 * cannot read CSS custom properties, so the phone needs these same values as
 * a plain object, and a file with no imports can be consumed by anything
 * (Vite, Metro, vitest, jest, node) without dragging a DOM or RN type
 * environment along with it.
 *
 * tokens.css is GENERATED from this file — never edit it by hand.
 *
 * Values here are exactly what SingZ's styles.css :root held before the kit
 * existed. Phase 1 moves ownership, not appearance.
 */

/** Colour, surface and type tokens. Consumers override these to re-theme. */
export const tokens = {
  // surfaces
  bg: '#12100d',
  panel: '#1b1814',
  'panel-deep': '#0f0d0a',

  // hairlines
  line: 'rgba(255, 240, 214, 0.08)',
  'line-strong': 'rgba(255, 240, 214, 0.2)',

  // foreground
  text: '#f4efe6',
  dim: '#9b917e',
  faint: '#6b6355',

  // brand
  accent: '#ffa028',
  'accent-deep': '#ff8a1f',
  'accent-soft': 'rgba(255, 160, 40, 0.13)',
  /** Text ON an accent fill — dark enough to read against #ffa028. */
  'accent-ink': '#241705',

  // status
  danger: '#ff7a5c',
  success: '#58d68a',

  // type. The kit declares the variables but ships no fonts: the two
  // @fontsource-variable packages belong to the app, and a duplicate
  // @font-face set would double the woff2 in every consumer's bundle.
  'font-display': 'system-ui, sans-serif',
  'font-mono': "ui-monospace, 'SF Mono', monospace"
} as const

export type TokenName = keyof typeof tokens

/** `--sz-accent` etc. The prefix exists so a consumer's own :root cannot
 *  silently collide with the kit's — a collision should be a rename error,
 *  not a cascade coin-flip decided by stylesheet order. */
export const cssVar = (name: TokenName): string => `--sz-${name}`

/** The whole set as a `:root { … }` block. Used to generate tokens.css. */
export function toCss(): string {
  const body = (Object.keys(tokens) as TokenName[])
    .map((k) => `  ${cssVar(k)}: ${tokens[k]};`)
    .join('\n')
  return `:root {\n${body}\n}\n`
}
